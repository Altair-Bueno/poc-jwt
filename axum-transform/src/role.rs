use enum_utils::FromStr;

#[derive(Debug, Clone, Copy, Hash, FromStr, PartialEq, Eq)]
pub enum Role {
    ADMIN,
    USER,
    ANALYST,
}
