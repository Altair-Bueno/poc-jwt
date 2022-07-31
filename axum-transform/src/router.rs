use crate::auth::JWTAuthentication;
use crate::config::Config;
use crate::controller;
use crate::error::ConfigError;
use crate::util::{load_decoding_key, load_validation};
use axum::{Extension, Router};
use tower::ServiceBuilder;
use tower_http::compression::CompressionLayer;
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;

pub async fn app(config: &Config) -> Result<Router, ConfigError> {
    let key = load_decoding_key(&config).await?;
    let validation = load_validation(&config).await?;

    let middleware = ServiceBuilder::new()
        // https://docs.rs/axum/0.5.13/axum/middleware/index.html#ordering
        .layer(Extension(JWTAuthentication::new(key, validation)))
        .layer(CompressionLayer::new())
        .layer(TraceLayer::new_for_http())
        .layer(CorsLayer::permissive());

    let controller = controller::router();
    let app = Router::new().nest("/", controller).layer(middleware);
    Ok(app)
}
