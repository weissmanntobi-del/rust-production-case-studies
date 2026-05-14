use crate::Job;
use thiserror::Error;

/// Errors returned by the actual job handler.
#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub enum JobError {
    #[error("transient error: {0}")]
    Transient(String),

    #[error("permanent error: {0}")]
    Permanent(String),

    #[error("job attempt timed out")]
    Timeout,
}

impl JobError {
    pub fn transient(message: impl Into<String>) -> Self {
        Self::Transient(message.into())
    }

    pub fn permanent(message: impl Into<String>) -> Self {
        Self::Permanent(message.into())
    }

    pub fn is_retryable(&self) -> bool {
        matches!(self, Self::Transient(_) | Self::Timeout)
    }
}

/// Returned to callers when the bounded queue cannot accept work.
#[derive(Debug, Error)]
pub enum SubmitError {
    #[error("job queue is full; the job was returned to the caller")]
    QueueFull(Job),

    #[error("job queue is closed; the job was returned to the caller")]
    QueueClosed(Job),
}

impl SubmitError {
    pub fn into_job(self) -> Job {
        match self {
            Self::QueueFull(job) | Self::QueueClosed(job) => job,
        }
    }
}

#[derive(Debug, Error)]
pub enum WorkerShutdownError {
    #[error("worker task failed to join: {0}")]
    Join(#[from] tokio::task::JoinError),
}
