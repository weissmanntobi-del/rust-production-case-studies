//! Secure file scanner library.
//!
//! This crate demonstrates a production-oriented safe file intake pattern:
//! canonical path checks, root containment, size gates, streaming reads,
//! signature classification, SHA-256 identity, and structured reports.

pub mod classifier;
pub mod error;
pub mod policy;
pub mod report;
pub mod scanner;

pub use classifier::{classify_prefix, FileKind};
pub use error::ScanError;
pub use policy::ScanPolicy;
pub use report::{RiskFlag, ScanReport, Verdict};
pub use scanner::{scan_file, scan_path, ScanBatch};
