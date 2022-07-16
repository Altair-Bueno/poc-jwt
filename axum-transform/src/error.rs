use axum::{
    extract::rejection::TypedHeaderRejection, http::StatusCode, response::IntoResponse, Json,
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use thiserror::Error;

pub type RequestResult<T> = std::result::Result<T, RequestError>;

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error(transparent)]
    IOerror(#[from] std::io::Error),
    #[error(transparent)]
    KeyError(#[from] jsonwebtoken::errors::Error),
    #[error(transparent)]
    FigmentError(#[from] figment::Error),
    #[error(transparent)]
    ServerError(#[from] hyper::Error),
}

#[derive(Debug, Error)]
pub enum AuthenticationError {
    #[error("Missing authentication header")]
    MissingAuthenticationHeader(#[from] TypedHeaderRejection),
    #[error("Invalid token: {0}")]
    InvalidToken(#[from] jsonwebtoken::errors::Error),
}

#[derive(Debug, Error)]
pub enum RequestError {
    #[error(transparent)]
    Unauthorised(#[from] AuthenticationError),
    #[error("{0}")]
    InternalServerError(&'static str),
}

#[derive(Debug, Serialize, Deserialize)]
struct Payload {
    timestamp: DateTime<Utc>,
    message: String,
}

impl IntoResponse for RequestError {
    fn into_response(self) -> axum::response::Response {
        let code = match self {
            RequestError::Unauthorised(_) => StatusCode::UNAUTHORIZED,
            RequestError::InternalServerError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        };
        let message = self.to_string();
        let timestamp = Utc::now();
        let payload = Json(Payload { timestamp, message });
        (code, payload).into_response()
    }
}
