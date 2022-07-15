use std::{collections::HashSet, path::Path, sync::Arc};

use axum::{
    async_trait,
    extract::{FromRequest, RequestParts},
    headers::{authorization::Bearer, Authorization},
    TypedHeader,
};
use jsonwebtoken::{DecodingKey, Validation};
use serde::{Deserialize, Serialize};
use tokio::fs::read;
use tracing::error;

use crate::{
    error::{AuthenticationError, ConfigError, RequestError},
    role::Role,
};

#[derive(Clone)]
pub struct PublicKey {
    key: Arc<DecodingKey>,
}
impl PublicKey {
    pub async fn new(path: impl AsRef<Path>) -> Result<PublicKey, ConfigError> {
        let bytes = read(path).await?;
        let decoding = DecodingKey::from_rsa_pem(bytes.as_slice())?;
        let key = Arc::new(decoding);
        Ok(PublicKey { key })
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    iss: String,
    sub: String,
    exp: usize,
    iat: usize,
    roles: HashSet<String>,
}

#[derive(Debug)]
pub struct JWTAuth {
    subject: String,
    roles: HashSet<Role>,
}

impl From<Claims> for JWTAuth {
    fn from(claims: Claims) -> Self {
        let Claims { sub, roles, .. } = claims;
        let roles = roles.into_iter().filter_map(|x| x.parse().ok()).collect();
        JWTAuth {
            subject: sub,
            roles,
        }
    }
}

#[async_trait]
impl<B> FromRequest<B> for JWTAuth
where
    B: Send,
{
    type Rejection = RequestError;

    async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        authorise(req).await.map_err(Into::into)
    }
}

async fn authorise<B: Send>(req: &mut RequestParts<B>) -> Result<JWTAuth, AuthenticationError> {
    let TypedHeader(Authorization(bearer)) =
        TypedHeader::<Authorization<Bearer>>::from_request(req).await?;

    let content = req.extensions().get::<PublicKey>();
    let PublicKey { key } = match content {
        Some(x)=>x,
        None => {
            error!("Unreachable statement. Missing Public Key");
            panic!();
        },
    };
    let token = bearer.token();
    let validation = Validation::new(jsonwebtoken::Algorithm::RS256);
    let claims: Claims = jsonwebtoken::decode(token, key, &validation)?.claims;

    Ok(claims.into())
}
