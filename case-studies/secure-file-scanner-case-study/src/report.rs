use std::path::PathBuf;

use serde::Serialize;

use crate::FileKind;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum RiskFlag {
    EmptyFile,
    UnknownType,
    ArchiveNeedsDeepScan,
    ExecutableBinary,
    ExtensionMismatch,
    LargeFileNearLimit,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Verdict {
    Allowed,
    Review,
    Blocked,
}

#[derive(Debug, Clone, Serialize)]
pub struct ScanReport {
    pub path: PathBuf,
    pub size_bytes: u64,
    pub bytes_sampled: usize,
    pub sha256: String,
    pub kind: FileKind,
    pub extension: Option<String>,
    pub risks: Vec<RiskFlag>,
    pub verdict: Verdict,
}

impl ScanReport {
    pub fn is_review_required(&self) -> bool {
        matches!(self.verdict, Verdict::Review | Verdict::Blocked)
    }
}
