use std::collections::HashSet;

use axum::extract::{FromRequest, RequestParts};
use axum::headers::authorization::Bearer;
use axum::headers::Authorization;
use axum::{async_trait, Extension, TypedHeader};
use getset::Getters;

use crate::error::{AuthenticationError, RequestError};
use crate::role::Role;

use crate::auth::Claims;

use super::JWTAuthentication;

#[derive(Debug, Getters)]
pub struct Authentication {
    subject: String,
    roles: HashSet<Role>,
}

impl From<Claims> for Authentication {
    fn from(claims: Claims) -> Self {
        let Claims { sub, roles, .. } = claims;
        let roles = roles.into_iter().filter_map(|x| x.parse().ok()).collect();
        Self {
            subject: sub,
            roles,
        }
    }
}

#[async_trait]
impl<B> FromRequest<B> for Authentication
where
    B: Send,
{
    type Rejection = RequestError;

    async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        let Extension(auth): Extension<JWTAuthentication> = Extension::from_request(req)
            .await
            .map_err(|_| RequestError::InternalServerError("Missing authentication layer"))?;
        authorise(&auth, req).await.map_err(Into::into)
    }
}

async fn authorise<B: Send>(
    jwt_auth: &JWTAuthentication,
    req: &mut RequestParts<B>,
) -> Result<Authentication, AuthenticationError> {
    let validation = jwt_auth.validation();
    let key = jwt_auth.key();
    let TypedHeader(Authorization(bearer)) =
        TypedHeader::<Authorization<Bearer>>::from_request(req).await?;

    let token = bearer.token();
    let claims: Claims = jsonwebtoken::decode(token, key, validation)?.claims;

    Ok(claims.into())
}
