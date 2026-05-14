# Rust Production Case Studies

Production-oriented Rust examples for developers who want to move beyond syntax and learn how real Rust systems are structured, tested, observed, and hardened.

This repository is a companion codebase for the **Rust Production Cheat Sheet**. It contains small but realistic Rust case studies focused on practical production concerns such as graceful shutdown, structured logging, health checks, secure file handling, bounded queues, retry policies, typed errors, metrics, and CI verification.

> This repository is educational. The examples are intentionally compact so that the production patterns are easy to understand. Before using any pattern in a real system, review your own security, privacy, performance, deployment, and compliance requirements.

---

## What This Repository Teaches

This repository focuses on production thinking, not only Rust syntax.

You will see examples of:

- typed error handling
- clear project structure
- `cargo fmt`, `cargo clippy`, and `cargo test`
- graceful shutdown
- health and readiness endpoints
- structured logging with `tracing`
- secure file intake patterns
- size limits and defensive parsing
- bounded async queues
- retry policies
- backpressure
- metrics counters
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
      src/
      tests/
      README.md

    secure-file-scanner-case-study/
      Cargo.toml
      Cargo.lock
      src/
      tests/
      README.md

    async-worker-bounded-queue-case-study/
      Cargo.toml
      Cargo.lock
      src/
      tests/
      examples/
      README.md
      HOW_TO_RUN.txt

  README.md
```