//! Async worker with bounded queue, retry, backpressure, and metrics counters.
//!
//! This crate is intentionally small and production-oriented: it demonstrates the
//! core shape of a background worker that protects memory with a bounded queue,
//! separates transient and permanent failures, retries with bounded backoff, and
//! exposes counters for operational visibility.

pub mod config;
pub mod error;
pub mod job;
pub mod metrics;
pub mod retry;
pub mod worker;

pub use config::WorkerConfig;
pub use error::{JobError, SubmitError, WorkerShutdownError};
pub use job::Job;
pub use metrics::{MetricsSnapshot, WorkerMetrics};
pub use worker::{start_worker, JobQueue, WorkerSystem};
