//! Custom error types for the HTTP client.

use thiserror::Error;

/// Errors that can occur when using the HTTP client.
#[derive(Debug, Error)]
pub enum HttpClientError {
    /// Failed to build the HTTP client (e.g., invalid configuration).
    #[error("Failed to build HTTP client: {0}")]
    BuildFailed(#[source] reqwest::Error),
    // Add more variants as needed, e.g.:
    // #[error("Request failed: {0}")]
    // RequestFailed(#[from] reqwest::Error),
    //
    // #[error("Invalid header value")]
    // InvalidHeader(#[from] reqwest::header::InvalidHeaderValue),
}

// Allow converting reqwest::Error directly into HttpClientError::BuildFailed.
impl From<reqwest::Error> for HttpClientError {
    fn from(err: reqwest::Error) -> Self {
        HttpClientError::BuildFailed(err)
    }
}
