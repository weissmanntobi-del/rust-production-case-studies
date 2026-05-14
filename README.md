# Rust Production Case Studies

Production-oriented Rust examples for developers who want to move beyond syntax and learn how real Rust systems are structured, tested, observed, and hardened.

This repository is a companion codebase for the **Rust Production Cheat Sheet**. It contains compact but realistic Rust case studies focused on practical production concerns such as graceful shutdown, structured logging, health checks, secure file handling, bounded queues, retry policies, typed errors, metrics, and CI verification.

> These examples are educational and intentionally compact. They demonstrate production-oriented patterns, but they are not a replacement for your own threat modeling, load testing, security review, compliance review, and deployment-specific hardening.

---

## What This Repository Teaches

This repository focuses on production thinking, not only Rust syntax.

You will see examples of:

- clear Rust project structure
- typed request and response models
- typed error handling
- `cargo fmt`, `cargo clippy`, and `cargo test`
- graceful shutdown
- health and readiness endpoints
- structured logging with `tracing`
- secure file intake patterns
- path validation and size limits
- defensive parsing
- bounded async queues
- backpressure
- retry policies
- transient vs permanent errors
- timeout handling
- metrics-style counters
- integration tests
- CI verification using GitHub Actions

---

## Repository Structure

```text
rust-production-case-studies/
  .github/
    workflows/
      rust.yml

  case-studies/
    axum-api-case-study/
      Cargo.toml
      Cargo.lock
      rust-toolchain.toml
      src/
      tests/
      README.md

    secure-file-scanner-case-study/
      Cargo.toml
      Cargo.lock
      rust-toolchain.toml
      src/
      tests/
      README.md

    async-worker-bounded-queue-case-study/
      Cargo.toml
      Cargo.lock
      rust-toolchain.toml
      src/
      tests/
      examples/
      README.md
      HOW_TO_RUN.txt

  README.md
  Technical-Verification-Notes.md
```

---

## Case Studies Included

### 1. Axum API Case Study

Folder:

```text
case-studies/axum-api-case-study
```

This case study demonstrates how to structure a small Axum HTTP API with production-oriented foundations.

Focus areas:

- Axum HTTP API
- `/health` endpoint
- `/readyz` endpoint
- typed request and response models
- typed API errors
- structured logging with `tracing`
- graceful shutdown
- integration tests
- Docker-ready structure

Run locally:

```bash
cd case-studies/axum-api-case-study

cargo fmt --check
cargo clippy -- -D warnings
cargo test
cargo run
```

Example endpoints:

```text
GET  /health
GET  /readyz
POST /users
```

Example request:

```bash
curl -X POST http://localhost:3000/users \
  -H "Content-Type: application/json" \
  -d '{"email":"user@example.com"}'
```

Production concerns demonstrated:

- typed API boundaries
- explicit error mapping
- health/readiness endpoints
- graceful shutdown
- structured logs
- testable router structure

---

### 2. Secure File Scanner Case Study

Folder:

```text
case-studies/secure-file-scanner-case-study
```

This case study demonstrates safe file intake and defensive file processing patterns.

Focus areas:

- safe file intake
- path validation
- canonical path checks
- file size limits
- basic file signature classification
- structured scanner errors
- no panic on invalid input
- tests for malformed or unsafe inputs

Run locally:

```bash
cd case-studies/secure-file-scanner-case-study

cargo fmt --check
cargo clippy -- -D warnings
cargo test
cargo run
```

Production concerns demonstrated:

- never trust user-supplied paths blindly
- canonicalize and validate paths
- apply size limits before reading files
- classify files using signatures, not only extensions
- return structured errors instead of crashing
- avoid logging sensitive file contents

---

### 3. Async Worker with Bounded Queue Case Study

Folder:

```text
case-studies/async-worker-bounded-queue-case-study
```

This case study demonstrates how to build an async worker with bounded queues, retry behavior, timeout handling, and metrics-style counters.

Focus areas:

- Tokio async worker
- bounded `mpsc` queue
- backpressure
- retry policy
- transient vs permanent errors
- job timeout
- metrics-style counters
- worker tests

Run locally:

```bash
cd case-studies/async-worker-bounded-queue-case-study

cargo fmt --check
cargo clippy -- -D warnings
cargo test
cargo run
```

Production concerns demonstrated:

- avoid unbounded queues
- define queue-full behavior clearly
- separate transient and permanent failures
- use retry limits
- apply per-job timeouts
- record accepted, rejected, succeeded, failed, and retried jobs
- design for shutdown and idempotency

---

## CI Verification

This repository is designed to be verified with GitHub Actions.

Each case study is checked separately with:

```bash
cargo fmt --check
cargo clippy -- -D warnings
cargo test
```

The workflow file is located at:

```text
.github/workflows/rust.yml
```

The CI workflow should verify these project folders:

```text
case-studies/axum-api-case-study
case-studies/secure-file-scanner-case-study
case-studies/async-worker-bounded-queue-case-study
```

When CI is passing, the examples can be treated as verified educational examples. They are still not a replacement for production review, security review, threat modeling, load testing, dependency review, and deployment-specific hardening.

---

## Recommended Rust Version

The examples are written for:

```text
Rust 2021 edition
Stable Rust toolchain
```

Check your local version:

```bash
rustc --version
cargo --version
```

Update Rust:

```bash
rustup update stable
```

---

## Local Verification Commands

Run these commands inside each case study folder:

```bash
cargo fmt --check
cargo clippy -- -D warnings
cargo test
```

For a stricter release-style check, you can also run:

```bash
cargo build --release
cargo test --release
```

Optional security and dependency checks:

```bash
cargo audit
cargo deny check
cargo tree
```

If `cargo audit` or `cargo deny` is not installed, install them with:

```bash
cargo install cargo-audit
cargo install cargo-deny
```

---

## How to Use This Repository

Recommended learning path:

1. Start with `axum-api-case-study` to understand production-style HTTP service structure.
2. Study `secure-file-scanner-case-study` to understand defensive input handling.
3. Study `async-worker-bounded-queue-case-study` to understand async backpressure and retry behavior.
4. Run the tests.
5. Read the source code.
6. Modify the code.
7. Break something intentionally.
8. Fix it using compiler errors, tests, and logs.
9. Add one new production feature to each case study.

Suggested extensions:

- add request IDs to the Axum API
- add a real database readiness check
- add request body limits and timeout layers
- add ZIP archive inspection to the file scanner
- add SHA-256 hashing to scan reports
- add a dead-letter queue to the async worker
- add Prometheus metrics exporter
- add Docker Compose for local testing
- add load tests and benchmark notes
- add GitHub Actions security checks

---

## What This Repository Is Not

This repository is not a complete production platform.

It does not include every feature a real production system may require, such as:

- full authentication and authorization
- real database migrations
- secrets management
- Kubernetes deployment manifests
- complete observability stack
- distributed tracing backend
- load testing setup
- full threat model
- compliance review
- advanced rate limiting
- complete incident response workflow
- production-grade persistence layer
- multi-region deployment strategy

The goal is to teach production-oriented Rust patterns in a compact and understandable way.

---

## Suggested Production Checklist

Before adapting these examples to a real system, review:

- input validation
- authentication and authorization
- error handling
- timeout policy
- retry policy
- idempotency rules
- logging and privacy rules
- metrics and alerting
- dependency security
- Docker image hardening
- CI/CD pipeline
- secrets management
- database consistency
- graceful shutdown
- load testing
- threat model
- incident response process

---

## Case Study Comparison

| Case Study          | Main Topic              | Production Concern                                          |
|---------------------|-------------------------|-------------------------------------------------------------|
| Axum API            | HTTP backend service    | health checks, readiness, tracing, graceful shutdown        |
| Secure File Scanner | Defensive file handling | path validation, size limits, safe classification           |
| Async Worker        | Background processing   | bounded queues, backpressure, retry, metrics-style counters |

---

## Common Commands

Format code:

```bash
cargo fmt
```

Check formatting:

```bash
cargo fmt --check
```

Run Clippy:

```bash
cargo clippy -- -D warnings
```

Run tests:

```bash
cargo test
```

Build release binary:

```bash
cargo build --release
```

Show dependency tree:

```bash
cargo tree
```

Run dependency audit:

```bash
cargo audit
```

Run license/advisory policy checks:

```bash
cargo deny check
```

---

## Notes on “Production-Ready”

The examples in this repository are production-oriented, but intentionally compact.

That means:

- they demonstrate real production concerns
- they include tests
- they are designed for CI verification
- they use typed errors and clear boundaries
- they are still educational examples
- they should be adapted before real production use

A real production system should also include:

- environment-specific configuration
- secrets management
- strong authentication and authorization
- rate limiting
- dependency readiness checks
- observability dashboards
- alerting
- deployment manifests
- backup and recovery plans
- incident response documentation
- threat modeling
- load testing
- compliance review where required

---

## Technical Verification Notes

The companion examples should be verified with:

```bash
cargo fmt --check
cargo clippy -- -D warnings
cargo test
```

Recommended additional checks:

```bash
cargo build --release
cargo test --release
cargo audit
cargo deny check
```

Do not claim that an example is production-ready only because it compiles. Production readiness also depends on operational context, security requirements, dependency review, observability, deployment environment, and incident response planning.

---

## Related Material

This repository accompanies the **Rust Production Cheat Sheet** and related Rust learning material.

Complete Rust Material:

```text
https://tobiweissmann.gumroad.com/l/gnuvxu
```

---

## License

Use this repository for learning and educational purposes.

Before reusing code in a real project, review it carefully and adapt it to your own production requirements.

---

## Author

Created by Tobias Weissmann.