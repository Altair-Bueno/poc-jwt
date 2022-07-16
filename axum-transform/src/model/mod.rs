use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum Transformation {
    #[serde(rename = "capitalize")]
    Capitalize,
}

#[derive(Deserialize, Serialize)]
pub struct TransformRequest {
    pub transformation: Transformation,
    pub data: String,
}

#[derive(Deserialize, Serialize)]
pub struct TransformResponse {
    pub took: usize,
    pub data: String,
}
