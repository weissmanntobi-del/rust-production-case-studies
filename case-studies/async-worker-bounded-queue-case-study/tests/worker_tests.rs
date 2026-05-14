use async_worker_bounded_queue::{start_worker, Job, JobError, SubmitError, WorkerConfig};
use std::{
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
    },
    time::Duration,
};

fn test_config() -> WorkerConfig {
    WorkerConfig {
        queue_size: 2,
        max_attempts: 3,
        per_attempt_timeout: Duration::from_millis(50),
        base_backoff: Duration::from_millis(1),
        max_backoff: Duration::from_millis(5),
    }
}

#[tokio::test]
async fn queue_rejects_when_full() {
    let config = WorkerConfig {
        queue_size: 1,
        max_attempts: 1,
        per_attempt_timeout: Duration::from_secs(1),
        base_backoff: Duration::from_millis(1),
        max_backoff: Duration::from_millis(1),
    };

    let worker = start_worker(config, |job: Job| async move {
        if job.id == 1 {
            tokio::time::sleep(Duration::from_millis(200)).await;
        }
        Ok(())
    });

    let queue = worker.queue();

    queue.submit(Job::new(1, "slow")).unwrap();
    tokio::time::sleep(Duration::from_millis(30)).await;

    queue.submit(Job::new(2, "queued")).unwrap();
    let third = queue.submit(Job::new(3, "should be rejected"));

    assert!(matches!(third, Err(SubmitError::QueueFull(_))));

    let snapshot = worker.shutdown().await.unwrap();
    assert_eq!(snapshot.rejected_queue_full, 1);
}

#[tokio::test]
async fn permanent_error_does_not_retry() {
    let attempts = Arc::new(AtomicUsize::new(0));
    let attempts_for_handler = attempts.clone();

    let worker = start_worker(test_config(), move |_job: Job| {
        let attempts = attempts_for_handler.clone();
        async move {
            attempts.fetch_add(1, Ordering::SeqCst);
            Err(JobError::permanent("invalid input"))
        }
    });

    let queue = worker.queue();
    queue.submit(Job::new(1, "bad")).unwrap();

    let snapshot = worker.shutdown().await.unwrap();

    assert_eq!(attempts.load(Ordering::SeqCst), 1);
    assert_eq!(snapshot.permanent_failures, 1);
    assert_eq!(snapshot.failed, 1);
    assert_eq!(snapshot.retried, 0);
}

#[tokio::test]
async fn transient_error_retries_until_max_attempts() {
    let attempts = Arc::new(AtomicUsize::new(0));
    let attempts_for_handler = attempts.clone();

    let worker = start_worker(test_config(), move |_job: Job| {
        let attempts = attempts_for_handler.clone();
        async move {
            attempts.fetch_add(1, Ordering::SeqCst);
            Err(JobError::transient("dependency unavailable"))
        }
    });

    let queue = worker.queue();
    queue.submit(Job::new(1, "retry me")).unwrap();

    let snapshot = worker.shutdown().await.unwrap();

    assert_eq!(attempts.load(Ordering::SeqCst), 3);
    assert_eq!(snapshot.retried, 2);
    assert_eq!(snapshot.failed, 1);
}

#[tokio::test]
async fn transient_error_can_eventually_succeed() {
    let attempts = Arc::new(AtomicUsize::new(0));
    let attempts_for_handler = attempts.clone();

    let worker = start_worker(test_config(), move |_job: Job| {
        let attempts = attempts_for_handler.clone();
        async move {
            let current = attempts.fetch_add(1, Ordering::SeqCst) + 1;
            if current < 2 {
                Err(JobError::transient("temporary failure"))
            } else {
                Ok(())
            }
        }
    });

    let queue = worker.queue();
    queue.submit(Job::new(1, "eventual success")).unwrap();

    let snapshot = worker.shutdown().await.unwrap();

    assert_eq!(attempts.load(Ordering::SeqCst), 2);
    assert_eq!(snapshot.retried, 1);
    assert_eq!(snapshot.succeeded, 1);
    assert_eq!(snapshot.failed, 0);
}

#[tokio::test]
async fn timeout_is_recorded_as_failure() {
    let config = WorkerConfig {
        queue_size: 2,
        max_attempts: 1,
        per_attempt_timeout: Duration::from_millis(5),
        base_backoff: Duration::from_millis(1),
        max_backoff: Duration::from_millis(1),
    };

    let worker = start_worker(config, |_job: Job| async move {
        tokio::time::sleep(Duration::from_millis(50)).await;
        Ok(())
    });

    let queue = worker.queue();
    queue.submit(Job::new(1, "slow")).unwrap();

    let snapshot = worker.shutdown().await.unwrap();

    assert_eq!(snapshot.timed_out, 1);
    assert_eq!(snapshot.failed, 1);
    assert_eq!(snapshot.succeeded, 0);
}
