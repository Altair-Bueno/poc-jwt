use crate::{roles::Role, subject::Subject};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub iss: String,
    pub sub: String,
    pub exp: usize,
    pub iat: usize,
    pub roles: HashSet<Role>,
}

impl From<Claims> for Subject {
    fn from(claims: Claims) -> Self {
        let name = claims.sub;
        let roles = claims.roles;
        Subject { name, roles }
    }
}
