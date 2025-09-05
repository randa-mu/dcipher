use clap::Parser;
use serde::{Deserialize, Serialize};
use std::net::Ipv4Addr;

#[derive(Parser, Serialize, Deserialize, Debug, Clone)]
pub struct SharedConfig {
    /// The address to host the health-check HTTP server
    pub healthcheck_listen_addr: Ipv4Addr,

    /// The port to host the health-check HTTP server
    pub healthcheck_port: u16,
}

impl Default for SharedConfig {
    fn default() -> Self {
        Self {
            healthcheck_port: 8080,
            healthcheck_listen_addr: Ipv4Addr::new(0, 0, 0, 0),
        }
    }
}
