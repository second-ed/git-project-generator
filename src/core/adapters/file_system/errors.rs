use std::path::PathBuf;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum FileSystemError {
    #[error("file not found at path: {0}")]
    NotFound(PathBuf),
    #[error("failed to create directory: {0}")]
    FailedToCreateDirectory(PathBuf),
    #[error("IoError: {0}")]
    Io(#[from] std::io::Error),
}
