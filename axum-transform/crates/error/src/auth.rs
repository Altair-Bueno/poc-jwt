use super::ErrorResponse;
use axum::http::StatusCode;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AuthenticationError {
    #[error("Missing authentication header")]
    MissingAuthenticationHeader,
    #[error("Bad token encoding")]
    BadTokenEncoding,
    #[error("Missing bearer token")]
    MissingBearerToken,
    #[error("Invalid token: {0}")]
    InvalidToken(#[from] jsonwebtoken::errors::Error),
    #[error("Couldn't find subject")]
    SubjectNotFound,
}

impl From<AuthenticationError> for ErrorResponse {
    fn from(err: AuthenticationError) -> Self {
        let message = format!("{err}");
        let code = err.into();
        ErrorResponse { message, code }
    }
}

impl From<AuthenticationError> for StatusCode {
    fn from(err: AuthenticationError) -> Self {
        match err {
            AuthenticationError::BadTokenEncoding | AuthenticationError::MissingBearerToken => {
                StatusCode::BAD_REQUEST
            }
            AuthenticationError::MissingAuthenticationHeader
            | AuthenticationError::InvalidToken(_) => StatusCode::UNAUTHORIZED,
            AuthenticationError::SubjectNotFound => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}
