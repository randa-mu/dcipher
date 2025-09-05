use crate::config::AppConfig;
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
    use crate::config::{AppConfig, Libp2pConfig, NetworkConfig};
    use crate::transport::create_libp2p_transport;
    use alloy::primitives::{FixedBytes, U160, U256};
    use ark_bn254::g2::G2Affine;
    use config::shared::SharedConfig;
    use config::signing::{CommitteeConfig, MemberConfig};
    use libp2p::PeerId;
    use serde_keys::{Bn254SecretKey, Libp2pKeyWrapper};
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
