use std::{
    fs::{self, File},
    io::Read,
    path::{Path, PathBuf},
};

use serde::Serialize;
use sha2::{Digest, Sha256};
use tracing::{debug, warn};
use walkdir::WalkDir;

use crate::{
    classifier::{classify_prefix, expected_extensions},
    FileKind, RiskFlag, ScanError, ScanPolicy, ScanReport, Verdict,
};

#[derive(Debug, Serialize)]
pub struct ScanBatch {
    pub reports: Vec<ScanReport>,
    pub errors: Vec<String>,
}

impl ScanBatch {
    pub fn has_findings(&self) -> bool {
        !self.errors.is_empty() || self.reports.iter().any(ScanReport::is_review_required)
    }
}

pub fn scan_path(path: impl AsRef<Path>, policy: &ScanPolicy, recursive: bool) -> ScanBatch {
    let path = path.as_ref();
    let canonical = match path.canonicalize() {
        Ok(path) => path,
        Err(source) => {
            return ScanBatch {
                reports: Vec::new(),
                errors: vec![ScanError::InvalidInputPath {
                    path: path.to_path_buf(),
                    source,
                }
                .to_string()],
            };
        }
    };

    if canonical.is_dir() {
        if recursive {
            scan_directory(&canonical, policy)
        } else {
            ScanBatch {
                reports: Vec::new(),
                errors: vec![format!(
                    "input path is a directory; pass --recursive to scan it: {}",
                    canonical.display()
                )],
            }
        }
    } else {
        match scan_file(&canonical, policy) {
            Ok(report) => ScanBatch {
                reports: vec![report],
                errors: Vec::new(),
            },
            Err(error) => ScanBatch {
                reports: Vec::new(),
                errors: vec![error.to_string()],
            },
        }
    }
}

fn scan_directory(root: &Path, policy: &ScanPolicy) -> ScanBatch {
    let mut reports = Vec::new();
    let mut errors = Vec::new();

    for entry in WalkDir::new(root).follow_links(false) {
        let entry = match entry {
            Ok(entry) => entry,
            Err(error) => {
                errors.push(error.to_string());
                continue;
            }
        };

        if !entry.file_type().is_file() {
            continue;
        }

        match scan_file(entry.path(), policy) {
            Ok(report) => reports.push(report),
            Err(error) => errors.push(error.to_string()),
        }
    }

    ScanBatch { reports, errors }
}

pub fn scan_file(path: impl AsRef<Path>, policy: &ScanPolicy) -> Result<ScanReport, ScanError> {
    let input_path = path.as_ref();
    let canonical = canonicalize_input(input_path)?;
    ensure_inside_root(&canonical, policy)?;

    let metadata =
        fs::metadata(&canonical).map_err(|source| ScanError::io(canonical.clone(), source))?;
    if !metadata.is_file() {
        return Err(ScanError::NotRegularFile { path: canonical });
    }

    if metadata.len() > policy.max_bytes {
        return Err(ScanError::TooLarge {
            path: canonical,
            actual: metadata.len(),
            limit: policy.max_bytes,
        });
    }

    let (sha256, prefix, total_bytes) = read_bounded(&canonical, policy)?;
    let kind = classify_prefix(&prefix);
    let extension = normalized_extension(&canonical);
    let risks = build_risks(kind, extension.as_deref(), total_bytes, policy.max_bytes);
    let verdict = decide_verdict(&risks);

    debug!(path = %canonical.display(), ?kind, ?verdict, "file scanned");

    Ok(ScanReport {
        path: canonical,
        size_bytes: total_bytes,
        bytes_sampled: prefix.len(),
        sha256,
        kind,
        extension,
        risks,
        verdict,
    })
}

fn canonicalize_input(input_path: &Path) -> Result<PathBuf, ScanError> {
    input_path
        .canonicalize()
        .map_err(|source| ScanError::InvalidInputPath {
            path: input_path.to_path_buf(),
            source,
        })
}

fn ensure_inside_root(path: &Path, policy: &ScanPolicy) -> Result<(), ScanError> {
    if path.starts_with(&policy.allowed_root) {
        Ok(())
    } else {
        warn!(
            path = %path.display(),
            allowed_root = %policy.allowed_root.display(),
            "blocked path outside allowed root"
        );
        Err(ScanError::PathOutsideRoot {
            path: path.to_path_buf(),
        })
    }
}

fn read_bounded(path: &Path, policy: &ScanPolicy) -> Result<(String, Vec<u8>, u64), ScanError> {
    let mut file = File::open(path).map_err(|source| ScanError::io(path.to_path_buf(), source))?;
    let mut hasher = Sha256::new();
    let mut prefix = Vec::with_capacity(policy.max_prefix_bytes.min(8192));
    let mut total_bytes = 0u64;
    let mut buffer = [0u8; 8192];

    loop {
        let n = file
            .read(&mut buffer)
            .map_err(|source| ScanError::io(path.to_path_buf(), source))?;
        if n == 0 {
            break;
        }

        total_bytes = total_bytes.saturating_add(n as u64);
        if total_bytes > policy.max_bytes {
            return Err(ScanError::TooLarge {
                path: path.to_path_buf(),
                actual: total_bytes,
                limit: policy.max_bytes,
            });
        }

        hasher.update(&buffer[..n]);

        if prefix.len() < policy.max_prefix_bytes {
            let remaining = policy.max_prefix_bytes - prefix.len();
            let take = remaining.min(n);
            prefix.extend_from_slice(&buffer[..take]);
        }
    }

    Ok((hex::encode(hasher.finalize()), prefix, total_bytes))
}

fn normalized_extension(path: &Path) -> Option<String> {
    path.extension()
        .and_then(|value| value.to_str())
        .map(|value| value.to_ascii_lowercase())
}

fn build_risks(
    kind: FileKind,
    extension: Option<&str>,
    total_bytes: u64,
    max_bytes: u64,
) -> Vec<RiskFlag> {
    let mut risks = Vec::new();

    if total_bytes == 0 {
        risks.push(RiskFlag::EmptyFile);
    }

    if matches!(kind, FileKind::Unknown) {
        risks.push(RiskFlag::UnknownType);
    }

    if matches!(kind, FileKind::Zip) {
        risks.push(RiskFlag::ArchiveNeedsDeepScan);
    }

    if matches!(kind, FileKind::ElfExecutable | FileKind::WindowsExecutable) {
        risks.push(RiskFlag::ExecutableBinary);
    }

    if extension_mismatch(kind, extension) {
        risks.push(RiskFlag::ExtensionMismatch);
    }

    if total_bytes > max_bytes.saturating_mul(9) / 10 {
        risks.push(RiskFlag::LargeFileNearLimit);
    }

    risks
}

fn extension_mismatch(kind: FileKind, extension: Option<&str>) -> bool {
    let Some(extension) = extension else {
        return false;
    };

    let expected = expected_extensions(kind);
    !expected.is_empty() && !expected.contains(&extension)
}

fn decide_verdict(risks: &[RiskFlag]) -> Verdict {
    if risks
        .iter()
        .any(|risk| matches!(risk, RiskFlag::ExecutableBinary))
    {
        Verdict::Blocked
    } else if risks.is_empty() {
        Verdict::Allowed
    } else {
        Verdict::Review
    }
}
