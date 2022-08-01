use axum::{http::StatusCode, response::IntoResponse, Json};

pub mod auth;
use serde::{Serialize, Serializer};

#[derive(Serialize, Clone, Debug)]
pub struct ErrorResponse {
    message: String,
    #[serde(serialize_with = "serialize_status_code")]
    code: StatusCode,
}

fn serialize_status_code<S>(status_code: &StatusCode, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    status_code.as_u16().serialize(serializer)
}

impl IntoResponse for ErrorResponse {
    fn into_response(self) -> axum::response::Response {
        let code = self.code;

        (code, Json(self)).into_response()
    }
}
