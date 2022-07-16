use jsonwebtoken::{DecodingKey, Validation};
use rstest::fixture;


pub const SAMPLE_TOKEN:&str ="eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiIiLCJpc3MiOiJKb2huIERvZSIsImV4cCI6MCwiaWF0IjowLCJyb2xlcyI6W119.ZQInJT7TSylY9RvZ9nBSfTRO3H-Qbhtn9DuAyWvDV-k";

#[fixture]
pub fn key() -> DecodingKey {
    DecodingKey::from_base64_secret("YXNkZmE=").unwrap()
}

#[fixture]
pub fn validation() -> Validation {
    let mut validation = Validation::default();
    validation.insecure_disable_signature_validation();
    validation.validate_exp = false;
    validation
}