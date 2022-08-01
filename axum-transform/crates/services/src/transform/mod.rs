use std::{fmt::Debug, sync::Arc};

use async_trait::async_trait;
use chrono::Utc;
use models::transform::*;

#[cfg(test)]
mod test;
#[cfg(test)]
use mockall::automock;

#[cfg_attr(test, automock)]
#[async_trait]
pub trait Transform: Debug {
    async fn transform(&self, request: TransformRequest) -> TransformResponse;
}

pub type TransformService = Arc<dyn Transform + Send + Sync>;

#[derive(Clone, Debug, Default)]
pub struct StdTransform;

impl StdTransform {
    pub fn new() -> Self {
        Default::default()
    }
}

#[async_trait]
impl Transform for StdTransform {
    async fn transform(&self, request: TransformRequest) -> TransformResponse {
        let start = Utc::now();
        let TransformRequest {
            transformation,
            data,
        } = request;
        let data = match transformation {
            Transformation::Capitalize => data.to_uppercase(),
        };
        let took = start.signed_duration_since(Utc::now()).num_milliseconds() as _;

        TransformResponse { took, data }
    }
}
