use async_worker_bounded_queue::{start_worker, Job, JobError, WorkerConfig};
use std::time::Duration;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter("info,async_worker_bounded_queue=debug")
        .init();

    let config = WorkerConfig {
        queue_size: 5,
        max_attempts: 3,
        per_attempt_timeout: Duration::from_secs(2),
        base_backoff: Duration::from_millis(50),
        max_backoff: Duration::from_millis(500),
    };

    let worker = start_worker(config, |job: Job| async move {
        if job.payload.trim().is_empty() {
            return Err(JobError::permanent("payload cannot be empty"));
        }

        if job.payload.contains("transient") {
            return Err(JobError::transient(
                "simulated temporary dependency failure",
            ));
        }

        tracing::info!(job_id = job.id, key = %job.idempotency_key, "processed payload");
        Ok(())
    });

    let queue = worker.queue();

    for id in 1..=3 {
        queue.submit(Job::new(id, format!("payload-{id}")))?;
    }

    // This one is a permanent validation error.
    let _ = queue.submit(Job::new(4, ""));

    // This one will retry and eventually fail.
    let _ = queue.submit(Job::new(5, "transient dependency problem"));

    let snapshot = worker.shutdown().await?;
    println!("Final metrics: {snapshot:#?}");

    Ok(())
}
