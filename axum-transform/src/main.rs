#![allow(dead_code)]

use std::net::SocketAddr;

use tracing::error;
use tracing::info;

use config::Config;
use error::ConfigError;
use util::load_config;

use crate::router::app;
use crate::util::init_logger;

mod auth;
mod config;
mod controller;
mod error;
mod model;
mod role;
mod router;
#[cfg(test)]
mod test;
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

    let app = app(&config).await?;

    let Config { hostname, port, .. } = config;
    axum::Server::bind(&SocketAddr::from((hostname, port)))
        .serve(app.into_make_service())
        .await?;

    Ok(())
}
