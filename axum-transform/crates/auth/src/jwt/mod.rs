mod claims;
use axum::{
    body::BoxBody,
    http::{header::AUTHORIZATION, Request, Response},
    response::IntoResponse,
};
use claims::Claims;
use error::{auth::AuthenticationError, ErrorResponse};
use jsonwebtoken::{decode, DecodingKey, Validation};
use std::sync::Arc;
use tower_http::auth::AuthorizeRequest;

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

impl<B> AuthorizeRequest<B> for JwtAuthentication {
    type ResponseBody = BoxBody;
    fn authorize(&mut self, request: &mut Request<B>) -> Result<(), Response<Self::ResponseBody>> {
        authorise(self, request)
            .map_err(ErrorResponse::from)
            .map_err(IntoResponse::into_response)
    }
}

fn authorise<B>(
    jwtauth: &JwtAuthentication,
    request: &mut Request<B>,
) -> Result<(), AuthenticationError> {
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
    Ok(())
}
