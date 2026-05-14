# Production Checklist

## API boundary

- [ ] Validate all external input
- [ ] Set body size limits
- [ ] Set request timeouts
- [ ] Use stable error response formats
- [ ] Avoid logging secrets or personally sensitive data

## Observability

- [ ] Structured logs enabled
- [ ] Request IDs propagated
- [ ] Request path, status, and latency available in logs
- [ ] Metrics added for request count, latency, and error rate
- [ ] Alerts configured for 5xx rate and readiness failures

## Graceful shutdown

- [ ] SIGINT handled locally
- [ ] SIGTERM handled in containers/Kubernetes
- [ ] In-flight requests allowed to finish within a bounded grace period
- [ ] Database/message connections closed cleanly

## Deployment

- [ ] Runs as non-root user
- [ ] Health and readiness probes configured
- [ ] CPU and memory limits configured
- [ ] Dependency audit runs before release
- [ ] CI runs fmt, clippy, and tests
