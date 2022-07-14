use std::{
    net::{IpAddr, Ipv4Addr},
    path::PathBuf,
};

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    #[serde(default = "default_hostname")]
    pub hostname: IpAddr,
    #[serde(default = "default_port")]
    pub port: u16,
    pub public_key: PathBuf,
}

fn default_hostname() -> IpAddr {
    IpAddr::V4(Ipv4Addr::LOCALHOST)
}

fn default_port() -> u16 {
    9100
}
