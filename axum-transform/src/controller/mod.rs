use axum::Json;
use chrono::Utc;

use crate::{
    auth::JWTAuth,
    error::RequestResult,
    model::{TransformRequest, TransformResponse, Transformation},
};

pub async fn transform(
    _: JWTAuth,
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
