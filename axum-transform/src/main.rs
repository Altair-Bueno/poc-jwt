#![allow(dead_code)]

use std::net::SocketAddr;

use axum::routing::post;
use axum::{Extension, Router};
use tower::ServiceBuilder;
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;
use tracing::error;
use tracing::info;

use auth::PublicKey;
use config::Config;
use error::ConfigError;
use util::load_config;

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
    tracing_subscriber::fmt().init();

    let config = load_config()?;
    info!(config = ?config, "Loaded config");

    let Config {
        hostname,
        port,
        public_key,
    } = config;

    let public_key = PublicKey::new(public_key).await?;

    let app = Router::new().route("/", post(controller::transform)).layer(
        ServiceBuilder::new()
            .layer(TraceLayer::new_for_http())
            .layer(CorsLayer::permissive())
            .layer(Extension(public_key)),
    );

    let socket_addr = SocketAddr::from((hostname, port));
    axum::Server::bind(&socket_addr)
        .serve(app.into_make_service())
        .await?;
    Ok(())
}
