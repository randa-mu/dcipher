use crate::config::AppConfig;
use crate::signing::DsignerWrapper;
use anyhow::anyhow;
use dcipher_network::transports::libp2p::Libp2pNodeConfig;
use dcipher_signer::bls::{BlsPairingSigner, BlsThresholdSigner};
use std::collections::HashMap;

pub(crate) fn create_bn254_signer(
    config: &AppConfig,
    libp2p_node: Libp2pNodeConfig<u16>,
) -> anyhow::Result<DsignerWrapper<BlsPairingSigner<ark_bn254::Bn254>>> {
    let bls_secret_key = &config.committee.secret_key;
    let signer = BlsPairingSigner::<ark_bn254::Bn254>::new(bls_secret_key.clone().into());

    let signer = BlsThresholdSigner::new(
        signer,
        config.committee.n.get(),
        config.committee.t.get(),
        config.committee.member_id.get(),
        HashMap::default(), // no keys on g1
        config
            .committee
            .members
            .iter()
            .map(|n| (n.member_id.get(), n.bls_pk))
            .collect(),
    );

    let transport = libp2p_node
        .run(config.libp2p.multiaddr.clone())?
        .get_transport()
        .ok_or(anyhow!("failed to get libp2p transport"))?;
    let (_, threshold_signer) = signer.run(transport);

    Ok(DsignerWrapper::new(threshold_signer))
}
