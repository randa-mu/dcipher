use clap::Parser;
use serde::{Deserialize, Serialize};
use std::net::Ipv4Addr;

#[derive(Parser, Serialize, Deserialize, Debug, Clone)]
pub struct AgentConfig {
    /// The address to host the health-check HTTP server
    #[serde(default = "default_healthcheck_listen_addr")]
    pub healthcheck_listen_addr: Ipv4Addr,

    /// The port to host the health-check HTTP server
    #[serde(default = "default_healthcheck_port")]
    pub healthcheck_port: u16,

    /// the tracing log level to output
    #[serde(default = "default_log_level")]
    pub log_level: String,

    // whether to format the logs as json
    #[serde(default = "default_log_json")]
    pub log_json: bool,
}

impl Default for AgentConfig {
    fn default() -> Self {
        Self {
            healthcheck_port: default_healthcheck_port(),
            healthcheck_listen_addr: default_healthcheck_listen_addr(),
            log_level: default_log_level(),
            log_json: default_log_json(),
        }
    }
}
fn default_healthcheck_listen_addr() -> Ipv4Addr {
    Ipv4Addr::new(0, 0, 0, 0)
}

fn default_healthcheck_port() -> u16 {
    8080
}

fn default_log_level() -> String {
    "info".to_string()
}
fn default_log_json() -> bool {
    true
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json as json;

    #[test]
    fn defaults_from_empty_object() {
        let cfg: AgentConfig = json::from_str("{}").unwrap();

        assert_eq!(
            cfg.healthcheck_listen_addr,
            default_healthcheck_listen_addr()
        );
        assert_eq!(cfg.healthcheck_port, default_healthcheck_port());
        assert_eq!(cfg.log_level, default_log_level());
        assert_eq!(cfg.log_json, default_log_json());
    }

    #[test]
    fn defaults_match_impl_default() {
        let from_empty: AgentConfig = json::from_str("{}").unwrap();
        let from_default = AgentConfig::default();
        assert_eq!(
            from_empty.healthcheck_listen_addr,
            from_default.healthcheck_listen_addr
        );
        assert_eq!(from_empty.healthcheck_port, from_default.healthcheck_port);
        assert_eq!(from_empty.log_level, from_default.log_level);
        assert_eq!(from_empty.log_json, from_default.log_json);
    }

    #[test]
    fn partial_object_uses_defaults_for_missing_fields() {
        // Provide only one field; others should use defaults.
        let cfg: AgentConfig = json::from_str(r#"{"log_level":"debug"}"#).unwrap();

        assert_eq!(cfg.log_level, "debug");
        assert_eq!(
            cfg.healthcheck_listen_addr,
            default_healthcheck_listen_addr()
        );
        assert_eq!(cfg.healthcheck_port, default_healthcheck_port());
        assert_eq!(cfg.log_json, default_log_json());
    }

    #[test]
    fn explicit_values_override_defaults() {
        let cfg: AgentConfig = json::from_str(
            r#"{
                "healthcheck_listen_addr":"127.0.0.1",
                "healthcheck_port": 9000,
                "log_level":"trace",
                "log_json": false
            }"#,
        )
        .unwrap();

        assert_eq!(cfg.healthcheck_listen_addr, Ipv4Addr::new(127, 0, 0, 1));
        assert_eq!(cfg.healthcheck_port, 9000);
        assert_eq!(cfg.log_level, "trace");
        assert!(!cfg.log_json);
    }

    #[test]
    fn null_is_not_treated_as_missing() {
        // Serde defaults apply when the field is *absent*, not when it's null.
        // This test ensures we don't silently accept nulls.
        let err = json::from_str::<AgentConfig>(r#"{"log_level": null}"#).unwrap_err();
        let msg = err.to_string();
        assert!(
            msg.contains("invalid type") || msg.contains("null"),
            "unexpected error: {msg}"
        );
    }
}
