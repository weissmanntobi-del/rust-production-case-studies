use std::{fs, path::Path};

use secure_file_scanner::{scan_file, FileKind, RiskFlag, ScanError, ScanPolicy, Verdict};
use tempfile::tempdir;

fn write_file(path: &Path, bytes: &[u8]) {
    fs::write(path, bytes).expect("write test file");
}

#[test]
fn classifies_pdf_signature() {
    let root = tempdir().unwrap();
    let file = root.path().join("report.pdf");
    write_file(&file, b"%PDF-1.7\nbody");

    let policy = ScanPolicy::new(root.path(), 1024).unwrap();
    let report = scan_file(&file, &policy).unwrap();

    assert_eq!(report.kind, FileKind::Pdf);
    assert_eq!(report.verdict, Verdict::Allowed);
}

#[test]
fn rejects_file_outside_allowed_root() {
    let root = tempdir().unwrap();
    let outside = tempdir().unwrap();
    let file = outside.path().join("outside.txt");
    write_file(&file, b"hello");

    let policy = ScanPolicy::new(root.path(), 1024).unwrap();
    let error = scan_file(&file, &policy).unwrap_err();

    assert!(matches!(error, ScanError::PathOutsideRoot { .. }));
}

#[test]
fn rejects_file_above_max_size() {
    let root = tempdir().unwrap();
    let file = root.path().join("large.txt");
    write_file(&file, b"1234567890");

    let policy = ScanPolicy::new(root.path(), 4).unwrap();
    let error = scan_file(&file, &policy).unwrap_err();

    assert!(matches!(error, ScanError::TooLarge { .. }));
}

#[test]
fn flags_extension_mismatch() {
    let root = tempdir().unwrap();
    let file = root.path().join("image.png");
    write_file(&file, b"%PDF-1.7\nbody");

    let policy = ScanPolicy::new(root.path(), 1024).unwrap();
    let report = scan_file(&file, &policy).unwrap();

    assert_eq!(report.kind, FileKind::Pdf);
    assert!(report.risks.contains(&RiskFlag::ExtensionMismatch));
    assert_eq!(report.verdict, Verdict::Review);
}

#[test]
fn empty_file_requires_review_without_panicking() {
    let root = tempdir().unwrap();
    let file = root.path().join("empty.txt");
    write_file(&file, b"");

    let policy = ScanPolicy::new(root.path(), 1024).unwrap();
    let report = scan_file(&file, &policy).unwrap();

    assert!(report.risks.contains(&RiskFlag::EmptyFile));
    assert_eq!(report.verdict, Verdict::Review);
}

#[test]
fn executable_binary_is_blocked() {
    let root = tempdir().unwrap();
    let file = root.path().join("program.bin");
    write_file(&file, &[0x7f, b'E', b'L', b'F', 0, 0, 0]);

    let policy = ScanPolicy::new(root.path(), 1024).unwrap();
    let report = scan_file(&file, &policy).unwrap();

    assert_eq!(report.kind, FileKind::ElfExecutable);
    assert!(report.risks.contains(&RiskFlag::ExecutableBinary));
    assert_eq!(report.verdict, Verdict::Blocked);
}
