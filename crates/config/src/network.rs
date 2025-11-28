use crate::keys::Libp2pKeyWrapper;
use alloy::primitives::FixedBytes;
use libp2p::Multiaddr;
use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use url::Url;

#[serde_as]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NetworkConfig {
    pub chain_id: u64,
    pub rpc_url: Url,
    pub router_address: FixedBytes<20>,
    #[serde(default = "default_should_write")]
    pub should_write: bool,
    #[serde(default = "default_tx_gas_buffer")]
    pub tx_gas_buffer: u16,
    #[serde(default = "default_tx_gas_price_buffer")]
    pub tx_gas_price_buffer: u16,
    #[serde(with = "humantime_serde")]
    pub reregistration_delay: Option<std::time::Duration>,
}

#[serde_as]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Libp2pConfig {
    pub secret_key: Libp2pKeyWrapper,
    pub multiaddr: Multiaddr,
}

fn default_should_write() -> bool {
    false
}

/// 20 percent extra gas to the limit by default
fn default_tx_gas_buffer() -> u16 {
    120
}

/// no extra gas to the price by default
fn default_tx_gas_price_buffer() -> u16 {
    100
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json as json;

    // handy fixtures (valid 32-byte key and 20-byte address)
    const ADDRESS_20: &str = "0x2222222222222222222222222222222222222222"; // 20 bytes

    #[test]
    fn deserialize_all_explicit_fields() {
        let cfg: NetworkConfig = json::from_str(&format!(
            r#"{{
            "chain_id": 84532,
            "rpc_url": "wss://example.org",
            "router_address": "{ADDRESS_20}",
            "should_write": false,
            "request_timeout": "45s"
        }}"#
        ))
        .unwrap();

        assert_eq!(cfg.chain_id, 84532);
        assert_eq!(cfg.rpc_url, Url::parse("wss://example.org").unwrap());
        assert!(!cfg.should_write);

        // sanity: lengths
        assert_eq!(cfg.router_address.len(), 20);
    }

    #[test]
    fn missing_optional_fields_use_defaults() {
        // omit should_write and timeout
        let cfg: NetworkConfig = json::from_str(&format!(
            r#"{{
            "chain_id": 1,
            "rpc_url": "wss://example.org",
            "router_address": "{ADDRESS_20}"
        }}"#
        ))
        .unwrap();

        assert_eq!(cfg.should_write, default_should_write());
    }

    #[test]
    fn null_does_not_trigger_defaults_for_bool_or_duration() {
        // ensure we don't silently accept nulls
        let err1 = json::from_str::<NetworkConfig>(&format!(
            r#"{{
            "chain_id": 1,
            "rpc_url": "wss://example.org",
            "router_address": "{ADDRESS_20}",
            "should_write": null
        }}"#
        ))
        .unwrap_err();

        let err2 = json::from_str::<NetworkConfig>(&format!(
            r#"{{
            "chain_id": 1,
            "rpc_url": "wss://example.org",
            "router_address": "{ADDRESS_20}",
            "should_write": null
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
    fn missing_required_fields_error() {
        // omit chain_id
        let err = json::from_str::<NetworkConfig>(&format!(
            r#"{{
            "rpc_url": "wss://example.org",
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
