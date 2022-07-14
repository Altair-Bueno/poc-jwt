use axum::{
    extract::rejection::TypedHeaderRejection, http::StatusCode, Json, response::IntoResponse,
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use thiserror::Error;

pub type RequestResult<T> = std::result::Result<T, RequestError>;

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("{0}")]
    IOerror(#[from] std::io::Error),
    #[error("{0}")]
    KeyError(#[from] jsonwebtoken::errors::Error),
    #[error("{0}")]
    FigmentError(#[from] figment::Error),
    #[error("{0}")]
    ServerError(#[from] hyper::Error),
}

#[derive(Debug, Error)]
pub enum AuthenticationError {
    #[error("Missing authentication header")]
    MissingAuthenticationHeader(#[from] TypedHeaderRejection),
    #[error("Invalid token")]
    InvalidToken(#[from] jsonwebtoken::errors::Error),
    #[error("Missing decriptionKey")]
    MissingDecriptionKey,
}

#[derive(Debug, Error)]
pub enum RequestError {
    #[error("{0}")]
    Unauthorised(#[from] AuthenticationError),
}

#[derive(Debug, Serialize, Deserialize)]
struct Payload {
    timestamp: DateTime<Utc>,
    message: String,
}

impl IntoResponse for RequestError {
    fn into_response(self) -> axum::response::Response {
        let (code, message) = match self {
            RequestError::Unauthorised(x) => (StatusCode::UNAUTHORIZED, format!("{x}")),
        };

        let timestamp = Utc::now();
        let payload = Json(Payload { timestamp, message });
        (code, payload).into_response()
    }
}
