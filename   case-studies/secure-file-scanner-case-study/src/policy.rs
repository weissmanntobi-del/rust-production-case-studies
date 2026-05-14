use std::{path::PathBuf, time::Duration};

use crate::ScanError;

/// Controls how the scanner accepts and reads files.
#[derive(Debug, Clone)]
pub struct ScanPolicy {
    /// Canonical directory that all scanned files must live under.
    pub allowed_root: PathBuf,
    /// Maximum file size accepted before scanning.
    pub max_bytes: u64,
    /// Number of prefix bytes used for file kind classification.
    pub max_prefix_bytes: usize,
    /// Optional per-file timeout for callers that run scans in a timeout wrapper.
    pub scan_timeout: Duration,
}

impl ScanPolicy {
    pub fn new(allowed_root: impl Into<PathBuf>, max_bytes: u64) -> Result<Self, ScanError> {
        if max_bytes == 0 {
            return Err(ScanError::InvalidPolicy(
                "max_bytes must be greater than zero".to_string(),
            ));
        }

        let root = allowed_root.into();
        let canonical_root = root
            .canonicalize()
            .map_err(|source| ScanError::InvalidRoot {
                path: root.clone(),
                source,
            })?;

        if !canonical_root.is_dir() {
            return Err(ScanError::InvalidPolicy(format!(
                "allowed_root is not a directory: {}",
                canonical_root.display()
            )));
        }

        Ok(Self {
            allowed_root: canonical_root,
            max_bytes,
            max_prefix_bytes: 4096,
            scan_timeout: Duration::from_secs(5),
        })
    }

    pub fn with_prefix_bytes(mut self, max_prefix_bytes: usize) -> Result<Self, ScanError> {
        if max_prefix_bytes == 0 {
            return Err(ScanError::InvalidPolicy(
                "max_prefix_bytes must be greater than zero".to_string(),
            ));
        }
        self.max_prefix_bytes = max_prefix_bytes;
        Ok(self)
    }

    pub fn with_timeout(mut self, timeout: Duration) -> Result<Self, ScanError> {
        if timeout.is_zero() {
            return Err(ScanError::InvalidPolicy(
                "scan_timeout must be greater than zero".to_string(),
            ));
        }
        self.scan_timeout = timeout;
        Ok(self)
    }
}
