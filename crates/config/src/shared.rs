use clap::Parser;
use serde::{Deserialize, Serialize};
use std::net::Ipv4Addr;

#[derive(Parser, Serialize, Deserialize, Debug, Clone)]
pub struct SharedConfig {
    /// The address to host the health-check HTTP server
    pub healthcheck_listen_addr: Ipv4Addr,

    /// The port to host the health-check HTTP server
    pub healthcheck_port: u16,

    /// the tracing log level to output
    pub log_level: String,

    // whether to format the logs as json
    pub log_json: bool,
}

impl Default for SharedConfig {
    fn default() -> Self {
        Self {
            healthcheck_port: 8080,
            healthcheck_listen_addr: Ipv4Addr::new(0, 0, 0, 0),
            log_level: "debug".to_string(),
            log_json: true,
        }
    }
}
