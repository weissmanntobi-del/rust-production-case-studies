# Axum API Case Study

Production-oriented Rust/Axum case study with:

- Axum 0.8 HTTP API
- `/health` and `/readyz` endpoints
- Typed JSON request/response structs
- Typed application errors mapped to HTTP responses
- Structured tracing logs
- Request IDs via `x-request-id`
- Request body limits
- Request timeout layer
- Graceful shutdown for SIGINT/SIGTERM
- Integration tests with `tower::ServiceExt`
- Dockerfile and GitHub Actions CI

## Endpoints

| Method | Path      | Purpose                                                          |
|--------|-----------|------------------------------------------------------------------|
| GET    | `/health` | Liveness check. Returns `204 No Content`.                        |
| GET    | `/readyz` | Readiness check. Returns `204` when ready, `503` when not ready. |
| POST   | `/users`  | Creates a user from JSON input.                                  |

Example request:

```bash
curl -i -X POST http://localhost:3000/users \
  -H 'content-type: application/json' \
  -d '{"email":"ada@example.com"}'
```

## Run locally

```bash
cargo run
```

Optional configuration:

```bash
APP_ADDR=127.0.0.1:3000 \
REQUEST_TIMEOUT_SECS=5 \
MAX_BODY_BYTES=65536 \
cargo run
```

## Verify

```bash
cargo fmt --check
cargo clippy -- -D warnings
cargo test
```

## Production notes

This is intentionally small, but it demonstrates a realistic production shape:

- Keep `/health` simple for process liveness.
- Use `/readyz` for dependency readiness checks such as database connectivity.
- Do not log raw secrets or sensitive values. This example logs only the email domain.
- Add authentication, authorization, rate limits, stricter CORS, database migrations, and metrics before using the pattern in a real system.
- In Kubernetes, SIGTERM is handled through graceful shutdown.

## Suggested next improvements

- Add PostgreSQL with SQLx and a `/users/:id` endpoint.
- Add Prometheus metrics.
- Add OpenTelemetry tracing.
- Add auth middleware.
- Add a stricter email validator crate if business rules require it.
