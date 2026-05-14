# syntax=docker/dockerfile:1

FROM rust:1-bookworm AS builder
WORKDIR /app
COPY Cargo.toml Cargo.lock* ./
COPY src ./src
RUN cargo build --release

FROM debian:bookworm-slim AS runtime
RUN useradd --create-home --uid 10001 appuser \
    && apt-get update \
    && apt-get install -y --no-install-recommends ca-certificates \
    && rm -rf /var/lib/apt/lists/*
WORKDIR /app
COPY --from=builder /app/target/release/axum-api-case-study /usr/local/bin/axum-api-case-study
USER appuser
ENV APP_ADDR=0.0.0.0:3000
EXPOSE 3000
HEALTHCHECK --interval=30s --timeout=3s --retries=3 CMD ["/bin/sh", "-c", "true"]
CMD ["axum-api-case-study"]
