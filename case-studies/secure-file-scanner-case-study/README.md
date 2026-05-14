# Secure File Scanner

A production-oriented Rust CLI and library for safe file intake.

It demonstrates:

- Canonical path validation against an allowed root
- File size gates before scanning
- Bounded streaming reads
- SHA-256 identity calculation
- Magic-byte file classification
- Risk flags for unknown files, extension mismatch, archives, executables, and empty files
- JSON output for automation
- Recursive directory scanning without following symlinks
- Unit/integration tests
- GitHub Actions CI for `fmt`, `clippy`, and `test`

This is intentionally a **safe intake and signature classification** project, not a malware engine. A real malware scanner would need sandboxing, format-specific parsers, archive/decompression limits, signature databases, timeout isolation, and dedicated security review.

## Project structure

```text
secure-file-scanner/
  src/
    lib.rs
    main.rs
    classifier.rs
    error.rs
    policy.rs
    report.rs
    scanner.rs
  tests/
    scanner_tests.rs
  .github/workflows/ci.yml
  Cargo.toml
  rust-toolchain.toml
  deny.toml
  README.md
```

## Quick start

```bash
cargo build
cargo test
cargo run -- ./README.md --root .
```

Scan a single file:

```bash
cargo run -- ./README.md --root . --json
```

Scan a directory recursively:

```bash
cargo run -- ./src --root . --recursive --json
```

Reject files above a maximum size:

```bash
cargo run -- ./large-file.bin --root . --max-bytes 1048576
```

Return a non-zero exit code when a file needs review:

```bash
cargo run -- ./some-file.bin --root . --fail-on-review
```

## Exit codes

| Code | Meaning |
|---:|---|
| 0 | Scan completed and no review condition triggered |
| 1 | CLI or runtime error |
| 2 | One or more scan errors occurred |
| 3 | `--fail-on-review` was used and at least one file requires review or is blocked |

## Example JSON output

```json
{
  "reports": [
    {
      "path": "/workspace/README.md",
      "size_bytes": 1234,
      "bytes_sampled": 1234,
      "sha256": "...",
      "kind": "text",
      "extension": "md",
      "risks": [],
      "verdict": "allowed"
    }
  ],
  "errors": []
}
```

## Security design notes

### Root containment

The scanner canonicalizes both the allowed root and the input path. A file is accepted only when its canonical path starts with the canonical allowed root. This blocks common path traversal attempts such as `../secret.txt`.

### Size gates

The scanner checks file metadata before opening the file and also enforces the byte limit while streaming the file. The second check helps reduce time-of-check/time-of-use risk when a file changes during scanning.

### Symlinks

Recursive directory traversal uses `walkdir` with `follow_links(false)`. Individual input paths are canonicalized, so symlink targets outside the allowed root are rejected.

### Logging

The code never logs file contents. In real systems, also consider whether filenames or full paths are sensitive in your environment.

### Archives

ZIP files are detected and marked with `archive_needs_deep_scan`. This project does not decompress archives. A production archive scanner should enforce:

- maximum compressed size
- maximum uncompressed size
- maximum file count
- maximum nesting depth
- decompression ratio limits
- per-file and total scan timeouts

## Recommended verification workflow

```bash
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test
cargo deny check
```

Install optional tooling:

```bash
cargo install cargo-deny
cargo install cargo-audit
```

## Suggested production extensions

- Add `axum` API wrapper around the scanner
- Add Prometheus metrics: scanned files, rejected files, error types, scan latency
- Add structured audit logs
- Add archive/decompression safety limits
- Add file-type-specific parsers behind sandboxed workers
- Add request IDs when used as a service
- Add a dead-letter/quarantine workflow for blocked files
- Add policy configuration via TOML/YAML
- Add fuzz tests for file signature classification

## License

MIT for educational/demo use.
