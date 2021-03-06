use axum::{Router, http::request::Builder, Extension, headers::ContentType, body::HttpBody};
use hyper::{StatusCode, header::{AUTHORIZATION, CONTENT_TYPE, ACCEPT}, Body};
use jsonwebtoken::{DecodingKey, Validation};
use rstest::*;
use axum::http::Request;
use speculoos::assert_that;
use crate::auth::JWTAuthentication;
use crate::test::*;

#[fixture]
fn router(
    key: DecodingKey,
    validation: Validation,
) -> Router {
    let authentication = JWTAuthentication::new(key, validation);
    super::router()
        .layer(Extension(authentication))
}

#[fixture]
fn request(
) -> Builder {
    Request::builder()
        .header(AUTHORIZATION, format!("Bearer {SAMPLE_TOKEN}"))
        .header(CONTENT_TYPE, ContentType::json().to_string())
        .header(ACCEPT, ContentType::json().to_string())
}

#[rstest]
#[case::root_post("/", "POST", r#"{"transformation": "capitalize", "data": "Hello world"}"#)]
#[tokio::test]
pub async fn check_if_routes_are_available(
    #[case] route:&'static str,
    #[case] method: &'static str,
    #[case] body:&'static str,
    router: Router,
    request: Builder
) {
    let request = request
        .method(method)
        .uri(route)
        .body(Body::from(body))
        .unwrap();
    
    let response = tower::ServiceExt::oneshot(router, request).await.unwrap();
    let status = response.status();
    print!("{:?}", hyper::body::to_bytes(response.into_body()).await.unwrap());

    assert_that!(status)
        .is_equal_to(StatusCode::OK);
}