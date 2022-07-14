use figment::{
    Figment,
    providers::{Env, Format, Toml},
};

use crate::{config::Config, error::ConfigError};

type Result<T> = std::result::Result<T, ConfigError>;

pub fn load_config() -> Result<Config> {
    let config = Figment::new()
        .merge(Toml::file("axum.toml"))
        .merge(Env::prefixed("AXUM_"))
        .extract()?;

    Ok(config)
}
