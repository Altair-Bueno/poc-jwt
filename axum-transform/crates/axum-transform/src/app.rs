use std::sync::Arc;

use crate::config::{Config, Loader, Jwt};
use auth::jwt::JwtAuthentication;
use axum::{Extension, Router};
use eyre::Result;
use jsonwebtoken::{DecodingKey, Validation};
use services::transform::{StdTransform, TransformService};
use tokio::fs::read;
use tower::ServiceBuilder;
use tower_http::{
    auth::AsyncRequireAuthorizationLayer, compression::CompressionLayer, cors::CorsLayer,
    trace::TraceLayer,
};

async fn load_decoding_key(config: &Config) -> Result<DecodingKey> {
    let Jwt {
        publickey, kind, ..
    } = &config.jwt;
    let content = read(publickey).await?;
    let slice = content.as_slice();
    let key = Loader::from(kind)(slice)?;

    Ok(key)
}

async fn load_validation(Config { jwt, .. }: &Config) -> Result<Validation> {
    let Jwt { algorithm, .. } = jwt;
    Ok(Validation::new(*algorithm))
}

pub async fn app(config: &Config) -> Result<Router> {
    let decodingkey = load_decoding_key(config).await?;
    let validation = load_validation(config).await?;

    let async_require_authorization = JwtAuthentication::new(decodingkey, validation);

    let transform = Arc::new(StdTransform::new()) as TransformService;
    let middleware = ServiceBuilder::new()
        // https://docs.rs/axum/0.5.13/axum/middleware/index.html#ordering
        .layer(Extension(transform))
        .layer(TraceLayer::new_for_http())
        .layer(CompressionLayer::new())
        .layer(AsyncRequireAuthorizationLayer::new(
            async_require_authorization,
        ))
        .layer(CorsLayer::permissive());
    let controller = controller::router();
    let app = Router::new().nest("/", controller).layer(middleware);
    Ok(app)
}
