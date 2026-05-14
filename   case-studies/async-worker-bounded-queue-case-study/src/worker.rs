use crate::{
    retry::backoff_delay, Job, JobError, MetricsSnapshot, SubmitError, WorkerConfig, WorkerMetrics,
};
use std::{future::Future, pin::Pin, sync::Arc};
use tokio::{
    sync::mpsc,
    task::JoinHandle,
    time::{sleep, timeout},
};

pub type BoxJobFuture = Pin<Box<dyn Future<Output = Result<(), JobError>> + Send + 'static>>;

/// Implement this trait for the actual background operation.
///
/// A blanket implementation allows you to pass async closures directly into
/// `start_worker`.
pub trait JobHandler: Send + Sync + 'static {
    fn handle(&self, job: Job) -> BoxJobFuture;
}

impl<F, Fut> JobHandler for F
where
    F: Fn(Job) -> Fut + Send + Sync + 'static,
    Fut: Future<Output = Result<(), JobError>> + Send + 'static,
{
    fn handle(&self, job: Job) -> BoxJobFuture {
        Box::pin((self)(job))
    }
}

#[derive(Debug)]
enum WorkerCommand {
    Job(Job),
    Shutdown,
}

/// Cloneable queue handle used by producers.
#[derive(Clone)]
pub struct JobQueue {
    sender: mpsc::Sender<WorkerCommand>,
    metrics: Arc<WorkerMetrics>,
    queue_size: usize,
}

impl JobQueue {
    /// Non-blocking submit. This is the backpressure boundary.
    ///
    /// When the bounded channel is full, the job is returned to the caller so it
    /// can be rejected, persisted elsewhere, or retried by the caller.
    pub fn submit(&self, job: Job) -> Result<(), SubmitError> {
        match self.sender.try_send(WorkerCommand::Job(job)) {
            Ok(()) => {
                self.metrics.inc_accepted();
                self.update_queue_depth_from_sender();
                Ok(())
            }
            Err(mpsc::error::TrySendError::Full(WorkerCommand::Job(job))) => {
                self.metrics.inc_rejected_queue_full();
                self.update_queue_depth_from_sender();
                Err(SubmitError::QueueFull(job))
            }
            Err(mpsc::error::TrySendError::Closed(WorkerCommand::Job(job))) => {
                self.metrics.inc_rejected_queue_closed();
                self.update_queue_depth_from_sender();
                Err(SubmitError::QueueClosed(job))
            }
            Err(mpsc::error::TrySendError::Full(WorkerCommand::Shutdown))
            | Err(mpsc::error::TrySendError::Closed(WorkerCommand::Shutdown)) => {
                unreachable!("submit only sends job commands")
            }
        }
    }

    pub fn metrics(&self) -> MetricsSnapshot {
        self.metrics.snapshot()
    }

    fn update_queue_depth_from_sender(&self) {
        let free_capacity = self.sender.capacity();
        let depth = self.queue_size.saturating_sub(free_capacity);
        self.metrics.set_queue_depth(depth);
    }
}

pub struct WorkerSystem {
    queue: JobQueue,
    join_handle: JoinHandle<()>,
}

impl WorkerSystem {
    pub fn queue(&self) -> JobQueue {
        self.queue.clone()
    }

    /// Sends a graceful shutdown command, closes the receiver, drains already
    /// queued jobs, waits for the worker task, and returns final metrics.
    pub async fn shutdown(self) -> Result<MetricsSnapshot, crate::WorkerShutdownError> {
        let WorkerSystem { queue, join_handle } = self;

        // If this fails, the worker has already stopped. We still join the task
        // to surface panics/cancellation.
        let _ = queue.sender.send(WorkerCommand::Shutdown).await;

        join_handle.await?;
        Ok(queue.metrics.snapshot())
    }
}

pub fn start_worker<H>(config: WorkerConfig, handler: H) -> WorkerSystem
where
    H: JobHandler,
{
    let config = config.normalized();
    let (sender, receiver) = mpsc::channel::<WorkerCommand>(config.queue_size);
    let metrics = Arc::new(WorkerMetrics::new(config.queue_size));
    let handler = Arc::new(handler);

    let join_handle = tokio::spawn(run_worker(
        receiver,
        handler,
        config.clone(),
        metrics.clone(),
    ));

    WorkerSystem {
        queue: JobQueue {
            sender,
            metrics,
            queue_size: config.queue_size,
        },
        join_handle,
    }
}

async fn run_worker<H>(
    mut receiver: mpsc::Receiver<WorkerCommand>,
    handler: Arc<H>,
    config: WorkerConfig,
    metrics: Arc<WorkerMetrics>,
) where
    H: JobHandler,
{
    while let Some(command) = receiver.recv().await {
        metrics.set_queue_depth(receiver.len());

        match command {
            WorkerCommand::Job(job) => {
                process_job(handler.as_ref(), &config, metrics.as_ref(), job).await;
            }
            WorkerCommand::Shutdown => {
                metrics.inc_shutdown_signal();
                receiver.close();

                while let Some(command) = receiver.recv().await {
                    metrics.set_queue_depth(receiver.len());
                    if let WorkerCommand::Job(job) = command {
                        process_job(handler.as_ref(), &config, metrics.as_ref(), job).await;
                    }
                }

                break;
            }
        }
    }

    metrics.set_queue_depth(0);
}

async fn process_job<H>(handler: &H, config: &WorkerConfig, metrics: &WorkerMetrics, job: Job)
where
    H: JobHandler,
{
    let job_id = job.id;
    metrics.inc_in_flight();

    let result = handle_with_retry(handler, config, metrics, job).await;

    metrics.dec_in_flight();

    match result {
        Ok(()) => {
            metrics.inc_succeeded();
            tracing::info!(job_id, "job succeeded");
        }
        Err(error) => {
            metrics.inc_failed();
            tracing::warn!(job_id, %error, "job failed");
        }
    }
}

async fn handle_with_retry<H>(
    handler: &H,
    config: &WorkerConfig,
    metrics: &WorkerMetrics,
    job: Job,
) -> Result<(), JobError>
where
    H: JobHandler,
{
    for attempt in 1..=config.max_attempts {
        let result = timeout(config.per_attempt_timeout, handler.handle(job.clone())).await;

        match result {
            Ok(Ok(())) => return Ok(()),
            Ok(Err(error)) if !error.is_retryable() => {
                metrics.inc_permanent_failure();
                return Err(error);
            }
            Ok(Err(error)) if attempt == config.max_attempts => {
                metrics.inc_transient_failure();
                return Err(error);
            }
            Ok(Err(error)) => {
                metrics.inc_transient_failure();
                metrics.inc_retried();
                tracing::warn!(
                    job_id = job.id,
                    attempt,
                    error = %error,
                    "job attempt failed; retrying"
                );
                sleep(backoff_delay(
                    config.base_backoff,
                    config.max_backoff,
                    attempt,
                    job.id,
                ))
                .await;
            }
            Err(_) if attempt == config.max_attempts => {
                metrics.inc_timed_out();
                return Err(JobError::Timeout);
            }
            Err(_) => {
                metrics.inc_timed_out();
                metrics.inc_retried();
                tracing::warn!(job_id = job.id, attempt, "job attempt timed out; retrying");
                sleep(backoff_delay(
                    config.base_backoff,
                    config.max_backoff,
                    attempt,
                    job.id,
                ))
                .await;
            }
        }
    }

    Err(JobError::Timeout)
}
