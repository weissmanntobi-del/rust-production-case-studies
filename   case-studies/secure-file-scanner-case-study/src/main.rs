use std::{path::PathBuf, process::ExitCode};

use anyhow::Context;
use clap::Parser;
use secure_file_scanner::{scan_path, ScanPolicy, Verdict};
use tracing_subscriber::EnvFilter;

#[derive(Debug, Parser)]
#[command(name = "secure-file-scanner")]
#[command(
    about = "Safely scan files using root containment, size limits, signatures, hashes, and structured reports."
)]
struct Cli {
    /// File or directory to scan.
    #[arg(value_name = "PATH")]
    path: PathBuf,

    /// Allowed root. Scanned files must remain inside this canonical directory.
    #[arg(long, default_value = ".")]
    root: PathBuf,

    /// Maximum accepted file size in bytes.
    #[arg(long, default_value_t = 10 * 1024 * 1024)]
    max_bytes: u64,

    /// Recursively scan a directory. Symlinks are not followed by directory traversal.
    #[arg(long)]
    recursive: bool,

    /// Emit JSON instead of a human-readable report.
    #[arg(long)]
    json: bool,

    /// Return non-zero when a scanned file requires review or is blocked.
    #[arg(long)]
    fail_on_review: bool,
}

fn main() -> ExitCode {
    init_tracing();

    match run() {
        Ok(exit_code) => exit_code,
        Err(error) => {
            eprintln!("error: {error:#}");
            ExitCode::from(1)
        }
    }
}

fn run() -> anyhow::Result<ExitCode> {
    let cli = Cli::parse();
    let policy = ScanPolicy::new(cli.root.clone(), cli.max_bytes).context("create scan policy")?;
    let batch = scan_path(&cli.path, &policy, cli.recursive);

    if cli.json {
        println!("{}", serde_json::to_string_pretty(&batch)?);
    } else {
        print_human_report(&batch);
    }

    if !batch.errors.is_empty() {
        return Ok(ExitCode::from(2));
    }

    if cli.fail_on_review
        && batch
            .reports
            .iter()
            .any(|report| matches!(report.verdict, Verdict::Review | Verdict::Blocked))
    {
        return Ok(ExitCode::from(3));
    }

    Ok(ExitCode::SUCCESS)
}

fn init_tracing() {
    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("warn"));
    tracing_subscriber::fmt().with_env_filter(filter).init();
}

fn print_human_report(batch: &secure_file_scanner::ScanBatch) {
    if batch.reports.is_empty() && batch.errors.is_empty() {
        println!("No files scanned.");
        return;
    }

    for report in &batch.reports {
        println!("Path: {}", report.path.display());
        println!("  Size: {} bytes", report.size_bytes);
        println!("  SHA-256: {}", report.sha256);
        println!("  Kind: {:?}", report.kind);
        println!("  Verdict: {:?}", report.verdict);
        if report.risks.is_empty() {
            println!("  Risks: none");
        } else {
            println!("  Risks: {:?}", report.risks);
        }
        println!();
    }

    for error in &batch.errors {
        eprintln!("Scan error: {error}");
    }
}
