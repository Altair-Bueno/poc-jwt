use enum_utils::FromStr;

#[derive(Debug, Clone, Copy, Hash, FromStr, PartialEq, Eq)]
pub enum Role {
    #[enumeration(rename = "ADMIN")]
    Admin,
    #[enumeration(rename = "USER")]
    User,
    #[enumeration(rename = "ANALYST")]
    Analyst,
}
