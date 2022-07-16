use figment::{
    providers::{Env, Format, Toml},
    Figment,
};
use jsonwebtoken::{DecodingKey, Validation};
use tokio::fs::read;

use crate::{config::Config, error::ConfigError};

type Result<T> = std::result::Result<T, ConfigError>;

pub fn load_config() -> Result<Config> {
    let config = Figment::new()
        .merge(Toml::file("axum.toml"))
        .merge(Env::prefixed("AXUM_"))
        .extract()?;

    Ok(config)
}

pub async fn load_decoding_key(Config { public_key, .. }: &Config) -> Result<DecodingKey> {
    let content = read(public_key).await?;
    let key = DecodingKey::from_rsa_pem(content.as_slice())?;

    Ok(key)
}

pub async fn load_validation(Config { .. }: &Config) -> Result<Validation> {
    Ok(Validation::new(jsonwebtoken::Algorithm::RS256))
}
