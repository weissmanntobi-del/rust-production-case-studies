use std::{io, path::PathBuf};

use thiserror::Error;

#[derive(Debug, Error)]
pub enum ScanError {
    #[error("invalid policy: {0}")]
    InvalidPolicy(String),

    #[error("allowed root does not exist or cannot be accessed: {path}: {source}")]
    InvalidRoot { path: PathBuf, source: io::Error },

    #[error("input path does not exist or cannot be accessed: {path}: {source}")]
    InvalidInputPath { path: PathBuf, source: io::Error },

    #[error("path is outside the allowed root: {path}")]
    PathOutsideRoot { path: PathBuf },

    #[error("path is not a regular file: {path}")]
    NotRegularFile { path: PathBuf },

    #[error("file is too large: {path}: {actual} bytes; limit is {limit} bytes")]
    TooLarge {
        path: PathBuf,
        actual: u64,
        limit: u64,
    },

    #[error("io error while accessing {path}: {source}")]
    Io { path: PathBuf, source: io::Error },
}

impl ScanError {
    pub fn io(path: impl Into<PathBuf>, source: io::Error) -> Self {
        Self::Io {
            path: path.into(),
            source,
        }
    }
}
