use thiserror::Error;

#[derive(Error, Debug)]
pub enum VaultProovError {
    #[error("I/O error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("parsing/verification error {0}")]
    ParsingError(#[from] c2pa::Error),
    #[error("{0}")]
    Other(String),
}

