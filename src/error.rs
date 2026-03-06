use thiserror::Error;

#[derive(Error, Debug)]
pub enum VaultProovError {
    #[error("I/O error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("C2PA/error {0}")]
    C2PAError(#[from] c2pa::Error),
    #[error("{0}")]
    Other(String),
}

