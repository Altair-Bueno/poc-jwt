#![allow(dead_code)]

use std::net::SocketAddr;

use axum::routing::post;
use axum::{Extension, Router};
use tower::ServiceBuilder;
use tower_http::compression::CompressionLayer;
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;
use tracing::error;
use tracing::info;

use config::Config;
use error::ConfigError;
use util::load_config;

use crate::auth::JWTAuthentication;
use crate::util::{init_logger, load_decoding_key, load_validation};

mod auth;
mod config;
mod controller;
mod error;
mod model;
mod role;
mod util;

#[tokio::main]
async fn main() {
    if let Err(error) = run().await {
        error!(%error, "Application crash");
    } else {
        info!("Application exit");
    }
}

async fn run() -> Result<(), ConfigError> {
    init_logger();

    let config = load_config()?;
    info!(config = ?config, "Loaded config");

    let Config { hostname, port, .. } = config;

    let key = load_decoding_key(&config).await?;
    let validation = load_validation(&config).await?;
    let middleware = ServiceBuilder::new()
        .layer(TraceLayer::new_for_http())
        .layer(CompressionLayer::new())
        .layer(CorsLayer::permissive())
        .layer(Extension(JWTAuthentication::new(key, validation)));

    let app = Router::new()
        .route("/", post(controller::transform))
        .layer(middleware);

    let socket_addr = SocketAddr::from((hostname, port));
    axum::Server::bind(&socket_addr)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}
