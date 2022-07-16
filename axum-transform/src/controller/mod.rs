#[cfg(test)]
mod test;
use axum::{routing::post, Json, Router};
use chrono::Utc;

use crate::{
    auth::extractor::Authentication,
    error::RequestResult,
    model::{TransformRequest, TransformResponse, Transformation},
};

pub fn router() -> Router {
    Router::new().route("/", post(transform))
}

async fn transform(
    _: Authentication,
    Json(request): Json<TransformRequest>,
) -> RequestResult<Json<TransformResponse>> {
    let took = Utc::now();
    let TransformRequest {
        transformation,
        data,
    } = request;

    let data = match transformation {
        Transformation::Capitalize => data.to_uppercase(),
    };

    let took = took.signed_duration_since(Utc::now()).num_milliseconds() as _;
    Ok(Json(TransformResponse { took, data }))
}
