use auth::extractor::Authorization;
use axum::{routing::post, Extension, Json, Router};
use models::transform::TransformRequest;
use models::transform::TransformResponse;
use services::transform::TransformService;

pub fn router() -> Router {
    Router::new().route("/", post(transform))
}

async fn transform(
    Json(request): Json<TransformRequest>,
    Extension(service): Extension<TransformService>,
    _: Authorization,
) -> Json<TransformResponse> {
    let result = service.transform(request).await;
    Json(result)
}
