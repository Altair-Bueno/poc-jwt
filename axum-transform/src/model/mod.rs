use chrono::{DateTime, Utc};
use enum_utils::FromStr;
use serde::{Deserialize, Serialize};

#[derive(FromStr, Debug, Serialize, Deserialize)]
pub enum Transformation {
    capitalize
}

#[derive(Deserialize, Serialize)]
pub struct TransformRequest {
    pub transformation: Transformation,
    pub data: String
}

#[derive(Deserialize, Serialize)]
pub struct TransformResponse {
    pub took: usize,
    pub data: String
}
