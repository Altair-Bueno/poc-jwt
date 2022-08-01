mod claims;
use axum::{
    body::BoxBody,
    http::{header::AUTHORIZATION, Request, Response},
    response::IntoResponse,
};
use claims::Claims;
use error::{auth::AuthenticationError, ErrorResponse};
use futures_util::future::BoxFuture;
use jsonwebtoken::{decode, DecodingKey, Validation};
use std::sync::Arc;
use tower_http::auth::AsyncAuthorizeRequest;

use crate::subject::Subject;

#[derive(Clone)]
pub struct JwtAuthentication {
    inner: Arc<(DecodingKey, Validation)>,
}

impl JwtAuthentication {
    pub fn new(decodingkey: DecodingKey, validation: Validation) -> JwtAuthentication {
        let inner = Arc::new((decodingkey, validation));
        JwtAuthentication { inner }
    }
}

impl<B> AsyncAuthorizeRequest<B> for JwtAuthentication
where
    B: Send + Sync + 'static,
{
    type RequestBody = B;
    type ResponseBody = BoxBody;
    type Future = BoxFuture<'static, Result<Request<B>, Response<Self::ResponseBody>>>;

    fn authorize(&mut self, request: Request<B>) -> Self::Future {
        let clone = self.clone();
        Box::pin(async move {
            authorise(&clone, request)
                .await
                .map_err(ErrorResponse::from)
                .map_err(IntoResponse::into_response)
        })
    }
}

async fn authorise<B>(
    jwtauth: &JwtAuthentication,
    mut request: Request<B>,
) -> Result<Request<B>, AuthenticationError> {
    let token = request
        .headers()
        .get(AUTHORIZATION)
        .ok_or(AuthenticationError::MissingAuthenticationHeader)?
        .to_str()
        .map_err(|_| AuthenticationError::BadTokenEncoding)?
        .strip_prefix("Bearer ")
        .ok_or(AuthenticationError::MissingAuthenticationHeader)?;

    let key = &jwtauth.inner.0;
    let validation = &jwtauth.inner.1;

    let claims: Claims = decode(token, key, validation)?.claims;
    let subject = Subject::from(claims);

    let _ignore = request.extensions_mut().insert(subject);

    Ok(request)
}
