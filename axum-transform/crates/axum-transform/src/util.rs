use crate::config::Config;
use eyre::Result;
use figment::{
    providers::{Env, Format, Toml},
    Figment,
};

pub fn load_config() -> Result<Config> {
    let config = Figment::new()
        .merge(Toml::file("axum.toml"))
        .merge(Env::prefixed("AXUM_").split("_"))
        .extract()?;
    Ok(config)
}

pub fn init_logger() {
    tracing_subscriber::fmt::init()
}
