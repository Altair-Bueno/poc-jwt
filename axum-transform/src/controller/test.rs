use crate::auth::JWTAuthentication;
use crate::test::*;
use axum::http::Request;
use axum::{body::HttpBody, headers::ContentType, http::request::Builder, Extension, Router};
use hyper::{
    header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE},
    Body, StatusCode,
};
use jsonwebtoken::{DecodingKey, Validation};
use rstest::*;
use speculoos::assert_that;

#[fixture]
fn router(key: DecodingKey, validation: Validation) -> Router {
    let authentication = JWTAuthentication::new(key, validation);
    super::router().layer(Extension(authentication))
}

#[fixture]
fn request() -> Builder {
    Request::builder()
        .header(AUTHORIZATION, format!("Bearer {SAMPLE_TOKEN}"))
        .header(CONTENT_TYPE, ContentType::json().to_string())
        .header(ACCEPT, ContentType::json().to_string())
}

#[rstest]
#[case::root_post(
    "/",
    "POST",
    r#"{"transformation": "capitalize", "data": "Hello world"}"#
)]
#[tokio::test]
pub async fn check_if_routes_are_available(
    #[case] route: &'static str,
    #[case] method: &'static str,
    #[case] body: &'static str,
    router: Router,
    request: Builder,
) {
    let request = request
        .method(method)
        .uri(route)
        .body(Body::from(body))
        .unwrap();

    let response = tower::ServiceExt::oneshot(router, request).await.unwrap();
    let status = response.status();

    assert_that!(status).is_equal_to(StatusCode::OK);
}
