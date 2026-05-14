use std::sync::atomic::{AtomicU64, AtomicUsize, Ordering};

/// In-process metrics counters.
///
/// In a real service you can export these counters to Prometheus/OpenTelemetry,
/// but atomic counters keep the example small and dependency-light.
#[derive(Debug)]
pub struct WorkerMetrics {
    accepted: AtomicU64,
    rejected_queue_full: AtomicU64,
    rejected_queue_closed: AtomicU64,
    succeeded: AtomicU64,
    failed: AtomicU64,
    retried: AtomicU64,
    timed_out: AtomicU64,
    permanent_failures: AtomicU64,
    transient_failures: AtomicU64,
    shutdown_signals: AtomicU64,
    in_flight: AtomicU64,
    queue_depth: AtomicUsize,
    queue_capacity: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MetricsSnapshot {
    pub accepted: u64,
    pub rejected_queue_full: u64,
    pub rejected_queue_closed: u64,
    pub succeeded: u64,
    pub failed: u64,
    pub retried: u64,
    pub timed_out: u64,
    pub permanent_failures: u64,
    pub transient_failures: u64,
    pub shutdown_signals: u64,
    pub in_flight: u64,
    pub queue_depth: usize,
    pub queue_capacity: usize,
}

impl WorkerMetrics {
    pub fn new(queue_capacity: usize) -> Self {
        Self {
            accepted: AtomicU64::new(0),
            rejected_queue_full: AtomicU64::new(0),
            rejected_queue_closed: AtomicU64::new(0),
            succeeded: AtomicU64::new(0),
            failed: AtomicU64::new(0),
            retried: AtomicU64::new(0),
            timed_out: AtomicU64::new(0),
            permanent_failures: AtomicU64::new(0),
            transient_failures: AtomicU64::new(0),
            shutdown_signals: AtomicU64::new(0),
            in_flight: AtomicU64::new(0),
            queue_depth: AtomicUsize::new(0),
            queue_capacity,
        }
    }

    pub fn snapshot(&self) -> MetricsSnapshot {
        MetricsSnapshot {
            accepted: self.accepted.load(Ordering::Relaxed),
            rejected_queue_full: self.rejected_queue_full.load(Ordering::Relaxed),
            rejected_queue_closed: self.rejected_queue_closed.load(Ordering::Relaxed),
            succeeded: self.succeeded.load(Ordering::Relaxed),
            failed: self.failed.load(Ordering::Relaxed),
            retried: self.retried.load(Ordering::Relaxed),
            timed_out: self.timed_out.load(Ordering::Relaxed),
            permanent_failures: self.permanent_failures.load(Ordering::Relaxed),
            transient_failures: self.transient_failures.load(Ordering::Relaxed),
            shutdown_signals: self.shutdown_signals.load(Ordering::Relaxed),
            in_flight: self.in_flight.load(Ordering::Relaxed),
            queue_depth: self.queue_depth.load(Ordering::Relaxed),
            queue_capacity: self.queue_capacity,
        }
    }

    pub(crate) fn inc_accepted(&self) {
        self.accepted.fetch_add(1, Ordering::Relaxed);
    }

    pub(crate) fn inc_rejected_queue_full(&self) {
        self.rejected_queue_full.fetch_add(1, Ordering::Relaxed);
    }

    pub(crate) fn inc_rejected_queue_closed(&self) {
        self.rejected_queue_closed.fetch_add(1, Ordering::Relaxed);
    }

    pub(crate) fn inc_succeeded(&self) {
        self.succeeded.fetch_add(1, Ordering::Relaxed);
    }

    pub(crate) fn inc_failed(&self) {
        self.failed.fetch_add(1, Ordering::Relaxed);
    }

    pub(crate) fn inc_retried(&self) {
        self.retried.fetch_add(1, Ordering::Relaxed);
    }

    pub(crate) fn inc_timed_out(&self) {
        self.timed_out.fetch_add(1, Ordering::Relaxed);
    }

    pub(crate) fn inc_permanent_failure(&self) {
        self.permanent_failures.fetch_add(1, Ordering::Relaxed);
    }

    pub(crate) fn inc_transient_failure(&self) {
        self.transient_failures.fetch_add(1, Ordering::Relaxed);
    }

    pub(crate) fn inc_shutdown_signal(&self) {
        self.shutdown_signals.fetch_add(1, Ordering::Relaxed);
    }

    pub(crate) fn inc_in_flight(&self) {
        self.in_flight.fetch_add(1, Ordering::Relaxed);
    }

    pub(crate) fn dec_in_flight(&self) {
        self.in_flight.fetch_sub(1, Ordering::Relaxed);
    }

    pub(crate) fn set_queue_depth(&self, depth: usize) {
        self.queue_depth.store(depth, Ordering::Relaxed);
    }
}
