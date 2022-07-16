use serde::{Deserialize, Serialize};
use std::{collections::HashSet, sync::Arc};

pub mod extractor;
use jsonwebtoken::{DecodingKey, Validation};

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    iss: String,
    sub: String,
    exp: usize,
    iat: usize,
    roles: HashSet<String>,
}

#[derive(Clone)]
pub struct JWTAuthentication {
    inner: Arc<Inner>,
}

struct Inner {
    key: DecodingKey,
    validation: Validation,
}

impl From<(DecodingKey, Validation)> for JWTAuthentication {
    fn from((key, validation): (DecodingKey, Validation)) -> Self {
        Self::new(key, validation)
    }
}

impl JWTAuthentication {
    pub fn new(key: DecodingKey, validation: Validation) -> Self {
        let inner = Inner { key, validation };
        let inner = Arc::new(inner);
        Self { inner }
    }

    pub fn key(&self) -> &DecodingKey {
        &self.inner.key
    }
    pub fn validation(&self) -> &Validation {
        &self.inner.validation
    }
}
