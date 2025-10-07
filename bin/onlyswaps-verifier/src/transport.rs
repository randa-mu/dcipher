use ark_ec::AffineRepr;
use config::keys::Libp2pKeyWrapper;
use config::signing::CommitteeConfig;
use dcipher_network::transports::libp2p::Libp2pNodeConfig;

pub(crate) fn create_libp2p_transport<G: AffineRepr>(
    libp2p_secret_key: &Libp2pKeyWrapper,
    committee_config: &CommitteeConfig<G>,
) -> anyhow::Result<Libp2pNodeConfig<u16>> {
    let (addrs, peer_ids, node_ids) = committee_config
        .members
        .iter()
        .map(|node| (node.address.clone(), node.peer_id, node.member_id.get()))
        .collect();

    Ok(Libp2pNodeConfig::new(
        libp2p_secret_key.clone().0,
        committee_config.member_id.get(),
        addrs,
        peer_ids,
        node_ids,
    ))
}

#[cfg(test)]
mod test {
    use crate::transport::create_libp2p_transport;
    use ark_bn254::G2Affine;
    use config::keys::{Bn254SecretKey, Libp2pKeyWrapper};
    use config::signing::{CommitteeConfig, MemberConfig};
    use libp2p::PeerId;
    use std::num::NonZeroU16;
    use std::str::FromStr;

    #[test]
    fn test_builds_with_valid_config() -> anyhow::Result<()> {
        let libp2p_secret = Libp2pKeyWrapper::from_str(
            "CAESQMbvGFHfIUOQv29mlUTngwhFk6zHdhwOaXUL4SEfVdEu8FgoWAuQzZ4ixgscoH2sCdszAqkLB3tI34LOivHd+WM=",
        )?;
        let committee_config = CommitteeConfig {
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
        };

        let _ = create_libp2p_transport(&libp2p_secret, &committee_config)?;

        Ok(())
    }
}
