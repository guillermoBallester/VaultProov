use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum VaultProovError {
    #[error("I/O error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("C2PA/error {0}")]
    C2PAError(#[from] c2pa::Error),
    #[error("multipart error {0}")]
    MultipartError(#[from] axum::extract::multipart::MultipartError),
    #[error("{0}")]
    Other(String),
}

impl IntoResponse for VaultProovError {
    fn into_response(self) -> Response {
        match self {
            VaultProovError::IoError(_) => (StatusCode::INTERNAL_SERVER_ERROR, self.to_string()).into_response(),
            VaultProovError::C2PAError(_) => (StatusCode::BAD_REQUEST, self.to_string()).into_response(),
            VaultProovError::MultipartError(_) => (StatusCode::BAD_REQUEST, self.to_string()).into_response(),
            VaultProovError::Other(_) => (StatusCode::BAD_REQUEST, self.to_string()).into_response(),
        }
    }
}

