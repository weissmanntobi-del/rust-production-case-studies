# Async Worker Bounded Queue

A small production-oriented Rust example showing a background worker with:

- bounded queue backpressure
- non-blocking job submission with returned jobs on rejection
- transient vs permanent error handling
- retry with bounded exponential backoff and deterministic jitter
- per-attempt timeout
- graceful shutdown with queue drain
- in-process metrics counters
- idempotency key in the job model
- unit/integration tests
- GitHub Actions CI workflow

This project is designed to accompany the **Rust Production Cheat Sheet** case study:

> Async worker with bounded queue, retry, backpressure, and metrics.

## Project structure

```text
async-worker-bounded-queue/
  src/
    lib.rs
    config.rs
    error.rs
    job.rs
    metrics.rs
    retry.rs
    worker.rs
  examples/
    demo.rs
  tests/
    worker_tests.rs
  .github/workflows/
    ci.yml
  Cargo.toml
  rust-toolchain.toml
  README.md
```

## Quick start

```bash
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-features
cargo run --example demo
```

## Core idea

The worker uses a bounded `tokio::sync::mpsc` channel. Producers call `try_send` through `JobQueue::submit`.
When the queue is full, the job is returned to the caller instead of being silently dropped or stored in an
unbounded queue.

```rust
let queue = worker.queue();
queue.submit(Job::new(1, "payload"))?;
```

This is the backpressure boundary. In a real HTTP API, a full queue could map to `429 Too Many Requests` or
`503 Service Unavailable`, depending on your product semantics.

## Metrics included

The example tracks:

- accepted jobs
- queue-full rejections
- queue-closed rejections
- succeeded jobs
- failed jobs
- retries
- timeouts
- permanent failures
- transient failures
- shutdown signals
- in-flight jobs
- queue depth
- queue capacity

The metrics are stored as atomic counters. In a real service, you can export them to Prometheus or OpenTelemetry.

## Production notes

This is intentionally small. For a larger production system, add:

- persistence for jobs that must survive process restarts
- dead-letter queue for repeated failures
- random jitter for distributed workers
- idempotency storage to prevent duplicate writes
- Prometheus/OpenTelemetry exporter
- structured request/job IDs propagated from API to worker
- dashboard and alerts for queue depth, rejection rate, latency, and failure rate

## Why bounded queues matter

Unbounded queues hide overload until the process runs out of memory. Bounded queues force you to decide what should
happen when the system is saturated: reject, shed load, persist elsewhere, or slow the producer.
