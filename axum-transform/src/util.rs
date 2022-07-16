use figment::{
    providers::{Env, Format, Toml},
    Figment,
};
use jsonwebtoken::{DecodingKey, Validation};
use tokio::fs::read;

use crate::{
    config::{Config, JWT, Loader},
    error::ConfigError,
};

type Result<T> = std::result::Result<T, ConfigError>;

pub fn load_config() -> Result<Config> {
    let config = Figment::new()
        .merge(Toml::file("axum.toml"))
        .merge(Env::prefixed("AXUM_").split("_"))
        .extract()?;

    Ok(config)
}

pub async fn load_decoding_key(
    Config {
        jwt: JWT { publickey, kind, .. },
        ..
    }: &Config,
) -> Result<DecodingKey> {
    let content = read(publickey).await?;
    let slice = content.as_slice();
    let key = Loader::from(kind)(slice)?;

    Ok(key)
}

pub async fn load_validation(Config { jwt, .. }: &Config) -> Result<Validation> {
    let JWT { algorithm, .. } = jwt;
    Ok(Validation::new(algorithm.clone()))
}

pub fn init_logger() {
    tracing_subscriber::fmt::init()
}
