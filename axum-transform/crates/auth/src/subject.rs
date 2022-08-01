use std::collections::HashSet;

use crate::roles::Role;

#[derive(Debug, Clone)]
pub struct Subject {
    pub name: String,
    pub roles: HashSet<Role>,
}
