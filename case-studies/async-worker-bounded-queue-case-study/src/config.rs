use std::time::Duration;

/// Runtime configuration for the worker.
#[derive(Debug, Clone)]
pub struct WorkerConfig {
    /// Maximum number of jobs waiting in the channel.
    /// Backpressure is applied when this capacity is reached.
    pub queue_size: usize,

    /// Maximum attempts per job. Must be at least 1.
    pub max_attempts: usize,

    /// Timeout applied to each single attempt.
    pub per_attempt_timeout: Duration,

    /// Initial retry delay.
    pub base_backoff: Duration,

    /// Maximum retry delay.
    pub max_backoff: Duration,
}

impl Default for WorkerConfig {
    fn default() -> Self {
        Self {
            queue_size: 100,
            max_attempts: 3,
            per_attempt_timeout: Duration::from_secs(3),
            base_backoff: Duration::from_millis(100),
            max_backoff: Duration::from_secs(2),
        }
    }
}

impl WorkerConfig {
    /// Returns a safe normalized config.
    /// This keeps examples robust even when callers accidentally pass zeros.
    pub fn normalized(mut self) -> Self {
        self.queue_size = self.queue_size.max(1);
        self.max_attempts = self.max_attempts.max(1);

        if self.per_attempt_timeout.is_zero() {
            self.per_attempt_timeout = Duration::from_secs(1);
        }

        if self.base_backoff.is_zero() {
            self.base_backoff = Duration::from_millis(10);
        }

        if self.max_backoff < self.base_backoff {
            self.max_backoff = self.base_backoff;
        }

        self
    }
}
