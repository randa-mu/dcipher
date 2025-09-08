use alloy::primitives::FixedBytes;
use serde::Deserialize;
use std::time::Duration;

#[derive(Deserialize, Debug, Clone)]
pub struct NetworkConfig {
    pub chain_id: u64,
    pub rpc_url: String,
    pub private_key: FixedBytes<32>,
    pub router_address: FixedBytes<20>,
    #[serde(default = "default_should_write")]
    pub should_write: bool,
    #[serde(with = "humantime_serde", default = "default_request_timeout")]
    pub request_timeout: Duration,
}

fn default_should_write() -> bool {
    true
}

fn default_request_timeout() -> Duration {
    Duration::from_secs(30)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json as json;

    // handy fixtures (valid 32-byte key and 20-byte address)
    const PRIVKEY_32: &str = "0x1111111111111111111111111111111111111111111111111111111111111111"; // 32 bytes
    const ADDRESS_20: &str = "0x2222222222222222222222222222222222222222"; // 20 bytes

    #[test]
    fn deserialize_all_explicit_fields() {
        let cfg: NetworkConfig = json::from_str(&format!(
            r#"{{
            "chain_id": 84532,
            "rpc_url": "wss://example.org",
            "private_key": "{PRIVKEY_32}",
            "router_address": "{ADDRESS_20}",
            "should_write": false,
            "request_timeout": "45s"
        }}"#
        ))
        .unwrap();

        assert_eq!(cfg.chain_id, 84532);
        assert_eq!(cfg.rpc_url, "wss://example.org");
        assert!(!cfg.should_write);
        assert_eq!(cfg.request_timeout, Duration::from_secs(45));

        // sanity: lengths
        assert_eq!(cfg.private_key.len(), 32);
        assert_eq!(cfg.router_address.len(), 20);
    }

    #[test]
    fn missing_optional_fields_use_defaults() {
        // omit should_write and timeout
        let cfg: NetworkConfig = json::from_str(&format!(
            r#"{{
            "chain_id": 1,
            "rpc_url": "wss://example.org",
            "private_key": "{PRIVKEY_32}",
            "router_address": "{ADDRESS_20}"
        }}"#
        ))
        .unwrap();

        assert_eq!(cfg.should_write, default_should_write());
        assert_eq!(cfg.request_timeout, default_request_timeout());
    }

    #[test]
    fn timeout_parses_humantime_strings() {
        let cfg: NetworkConfig = json::from_str(&format!(
            r#"{{
            "chain_id": 10,
            "rpc_url": "wss://example.org",
            "private_key": "{PRIVKEY_32}",
            "router_address": "{ADDRESS_20}",
            "request_timeout": "500ms"
        }}"#
        ))
        .unwrap();

        assert_eq!(cfg.request_timeout, Duration::from_millis(500));
    }

    #[test]
    fn null_does_not_trigger_defaults_for_bool_or_duration() {
        // ensure we don't silently accept nulls
        let err1 = json::from_str::<NetworkConfig>(&format!(
            r#"{{
            "chain_id": 1,
            "rpc_url": "wss://example.org",
            "private_key": "{PRIVKEY_32}",
            "router_address": "{ADDRESS_20}",
            "should_write": null
        }}"#
        ))
        .unwrap_err();

        let err2 = json::from_str::<NetworkConfig>(&format!(
            r#"{{
            "chain_id": 1,
            "rpc_url": "wss://example.org",
            "private_key": "{PRIVKEY_32}",
            "router_address": "{ADDRESS_20}",
            "request_timeout": null
        }}"#
        ))
        .unwrap_err();

        let msg1 = err1.to_string();
        let msg2 = err2.to_string();
        assert!(
            msg1.contains("null") || msg1.contains("invalid type"),
            "unexpected error: {msg1}"
        );
        assert!(
            msg2.contains("null") || msg2.contains("invalid type"),
            "unexpected error: {msg2}"
        );
    }

    #[test]
    fn numbers_for_timeout_are_rejected_by_humantime_serde() {
        // humantime_serde expects a string like "30s", not a bare number
        let err = json::from_str::<NetworkConfig>(&format!(
            r#"{{
            "chain_id": 1,
            "rpc_url": "wss://example.org",
            "private_key": "{PRIVKEY_32}",
            "router_address": "{ADDRESS_20}",
            "request_timeout": 30
        }}"#
        ))
        .unwrap_err();

        let msg = err.to_string();
        assert!(
            msg.contains("invalid type") || msg.contains("expected a string"),
            "unexpected error: {msg}"
        );
    }

    #[test]
    fn missing_required_fields_error() {
        // omit chain_id
        let err = json::from_str::<NetworkConfig>(&format!(
            r#"{{
            "rpc_url": "wss://example.org",
            "private_key": "{PRIVKEY_32}",
            "router_address": "{ADDRESS_20}"
        }}"#
        ))
        .unwrap_err();

        let msg = err.to_string();
        assert!(
            msg.contains("chain_id") || msg.contains("missing field"),
            "unexpected error: {msg}"
        );
    }
}
