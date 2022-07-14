use axum::Json;
use chrono::{Duration, Utc};
use tokio::time::Instant;

use crate::{
    auth::JWTAuth,
    error::RequestResult,
    model::{Transformation, TransformRequest, TransformResponse},
};

pub async fn transform(
    _: JWTAuth,
    Json(request): Json<TransformRequest>,
) -> RequestResult<Json<TransformResponse>> {
    let took = Utc::now();
    let TransformRequest{ transformation, data } = request;

    let data = match transformation {
        Transformation::capitalize => data.to_uppercase(),
    };

    let took = took.signed_duration_since(Utc::now()).num_milliseconds() as _;
    Ok(Json(TransformResponse { took, data }))
}
