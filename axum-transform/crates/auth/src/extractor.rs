use axum::{
    async_trait,
    extract::{rejection::ExtensionRejection, FromRequest, RequestParts},
};

use crate::subject::Subject;
use error::{auth::AuthenticationError, ErrorResponse};

#[derive(Debug)]
pub struct Authorization(Subject);

#[async_trait]
impl<B: Send> FromRequest<B> for Authorization {
    type Rejection = ErrorResponse;

    async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        req.extensions()
            .get::<Subject>()
            .map(Clone::clone)
            .ok_or(ErrorResponse::from(AuthenticationError::SubjectNotFound))
            .map(Authorization)
    }
}
