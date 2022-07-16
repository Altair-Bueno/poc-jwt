use std::{
    net::{IpAddr, Ipv4Addr},
    path::PathBuf,
};

use jsonwebtoken::{Algorithm, DecodingKey};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    #[serde(default = "default_hostname")]
    pub hostname: IpAddr,
    #[serde(default = "default_port")]
    pub port: u16,
    pub jwt: JWT,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JWT {
    pub publickey: PathBuf,
    #[serde(default = "default_algorithm")]
    pub algorithm: Algorithm,
    #[serde(default)]
    pub kind: LoadKind
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub enum LoadKind {
    #[default]
    RSAPem,
    ECPem,
}

pub type Loader = fn(&[u8]) -> Result<DecodingKey, jsonwebtoken::errors::Error>;
impl From<&LoadKind> for Loader {
    fn from(kind: &LoadKind) -> Self {
        match kind {
            LoadKind::RSAPem => DecodingKey::from_rsa_pem,
            LoadKind::ECPem => DecodingKey::from_ec_pem,
        }
    }
}


fn default_algorithm() -> Algorithm {
    Algorithm::RS256
}

fn default_hostname() -> IpAddr {
    IpAddr::V4(Ipv4Addr::LOCALHOST)
}

fn default_port() -> u16 {
    9100
}
