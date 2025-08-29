use crate::config::AppConfig;
use dcipher_network::transports::libp2p::Libp2pNodeConfig;
use libp2p::identity::Keypair;

pub(crate) fn create_libp2p_transport(config: &AppConfig) -> anyhow::Result<Libp2pNodeConfig<u16>> {
    let key = Keypair::from_protobuf_encoding(config.libp2p.secret_key.as_slice())?;
    let (addrs, peer_ids, node_ids) = config
        .group
        .nodes
        .iter()
        .map(|node| (node.address.clone(), node.peer_id, node.node_id.get()))
        .collect();

    Ok(Libp2pNodeConfig::new(
        key,
        config.secret_key.node_id.get(),
        addrs,
        peer_ids,
        node_ids,
    ))
}

#[cfg(test)]
mod test {
    use crate::config::{AppConfig, Libp2pConfig, NetworkConfig};
    use crate::transport::create_libp2p_transport;
    use alloy::primitives::{FixedBytes, U160, U256};
    use ark_bn254::g2::G2Affine;
    use base64::Engine;
    use base64::engine::general_purpose::STANDARD;
    use config::shared::SharedConfig;
    use config::signing::{GroupConfig, NodeConfig, SecretKeyConfig};
    use libp2p::PeerId;
    use serde_keys::Bn254SecretKey;
    use std::num::NonZeroU16;
    use std::str::FromStr;

    #[test]
    fn test_builds_with_valid_config() -> anyhow::Result<()> {
        let config = AppConfig {
            agent: SharedConfig {
                healthcheck_listen_addr: "0.0.0.0".parse()?,
                healthcheck_port: 8080,
            },
            networks: vec![NetworkConfig {
                chain_id: 1,
                rpc_url: "wss://example.com".to_string(),
                router_address: FixedBytes::from(U160::from(1)),
                private_key: FixedBytes::from(U256::from(1)),
                should_write: false,
            }],
            libp2p: Libp2pConfig {
                secret_key: STANDARD.decode("CAESQMbvGFHfIUOQv29mlUTngwhFk6zHdhwOaXUL4SEfVdEu8FgoWAuQzZ4ixgscoH2sCdszAqkLB3tI34LOivHd+WM=")?,
                multiaddr: "/ip4/127.0.0.1/tcp/8080".parse()?,
            },
            secret_key: SecretKeyConfig {
                node_id: NonZeroU16::new(1).unwrap(),
                secret_key: Bn254SecretKey::from_str("0x2800cafe7d54bcc5cc21d37a2e4e67a49654fc7ddf16bf616e15091962426f8d")?,
                t: NonZeroU16::new(1).unwrap(),
                n: NonZeroU16::new(1).unwrap(),
            },
            group: GroupConfig {
                nodes: vec![NodeConfig {
                    node_id: NonZeroU16::new(1).unwrap(),
                    address: "/ip4/127.0.0.1/tcp/8080".parse()?,
                    peer_id: PeerId::random(),
                    bls_pk: G2Affine::default(),
                }]
            },
        };

        let _ = create_libp2p_transport(&config)?;

        Ok(())
    }
}
