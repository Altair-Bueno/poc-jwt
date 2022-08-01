use std::net::SocketAddr;

mod app;
mod config;
mod util;
use app::app;
use eyre::Result;
use tracing::{error, info};
use util::*;

use crate::config::Config;

#[tokio::main]
pub async fn main() {
    let result = run().await;
    if let Err(err) = result {
        error!(%err, "Application crash")
    } else {
        info!("Aplication exited sucessfully")
    }
}

async fn run() -> Result<()> {
    init_logger();

    let cwd = std::env::current_dir();
    let log_level = std::env::var("RUST_LOG").unwrap_or_else(|_| "INFO".into());
    info!(?cwd, %log_level,"Application start");

    let config = load_config()?;
    info!(?config, "Sucessfully loaded configuration");

    let app = app(&config).await?;

    let Config { hostname, port, .. } = config;
    axum::Server::bind(&SocketAddr::from((hostname, port)))
        .serve(app.into_make_service())
        .await?;

    Ok(())
}
