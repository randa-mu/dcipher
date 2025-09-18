use config::file::AppConfig;
use dcipher_network::transports::libp2p::Libp2pNodeConfig;

pub(crate) fn create_libp2p_transport(config: &AppConfig) -> anyhow::Result<Libp2pNodeConfig<u16>> {
    let key = config.libp2p.secret_key.clone();
    let (addrs, peer_ids, node_ids) = config
        .committee
        .members
        .iter()
        .map(|node| (node.address.clone(), node.peer_id, node.member_id.get()))
        .collect();

    Ok(Libp2pNodeConfig::new(
        key.into(),
        config.committee.member_id.get(),
        addrs,
        peer_ids,
        node_ids,
    ))
}

#[cfg(test)]
mod test {
    use crate::transport::create_libp2p_transport;
    use alloy::primitives::{FixedBytes, U160, U256};
    use alloy::transports::http::reqwest::Url;
    use ark_bn254::G2Affine;
    use config::agent::AgentConfig;
    use config::file::{AppConfig, Libp2pConfig};
    use config::keys::{Bn254SecretKey, Libp2pKeyWrapper};
    use config::network::NetworkConfig;
    use config::signing::{CommitteeConfig, MemberConfig};
    use libp2p::PeerId;
    use std::num::NonZeroU16;
    use std::str::FromStr;
    use std::time::Duration;

    #[test]
    fn test_builds_with_valid_config() -> anyhow::Result<()> {
        let config = AppConfig {
            agent: AgentConfig {
                healthcheck_listen_addr: "0.0.0.0".parse()?,
                healthcheck_port: 8080,
                log_level: "debug".to_string(),
                log_json: true,
            },
            networks: vec![NetworkConfig {
                chain_id: 1,
                rpc_url: Url::parse("wss://example.com")?,
                router_address: FixedBytes::from(U160::from(1)),
                private_key: FixedBytes::from(U256::from(1)),
                should_write: false,
                request_timeout: Duration::from_secs(1),
            }],
            libp2p: Libp2pConfig {
                secret_key: Libp2pKeyWrapper::from_str(
                    "CAESQMbvGFHfIUOQv29mlUTngwhFk6zHdhwOaXUL4SEfVdEu8FgoWAuQzZ4ixgscoH2sCdszAqkLB3tI34LOivHd+WM=",
                )?,
                multiaddr: "/ip4/127.0.0.1/tcp/8080".parse()?,
            },
            committee: CommitteeConfig {
                member_id: NonZeroU16::new(1).unwrap(),
                secret_key: Bn254SecretKey::from_str(
                    "0x2800cafe7d54bcc5cc21d37a2e4e67a49654fc7ddf16bf616e15091962426f8d",
                )?,
                t: NonZeroU16::new(1).unwrap(),
                n: NonZeroU16::new(1).unwrap(),
                members: vec![MemberConfig {
                    member_id: NonZeroU16::new(1).unwrap(),
                    address: "/ip4/127.0.0.1/tcp/8080".parse()?,
                    peer_id: PeerId::random(),
                    bls_pk: G2Affine::default(),
                }],
            },
        };

        let _ = create_libp2p_transport(&config)?;

        Ok(())
    }
}
