use crate::config::AppConfig;
use crate::signing::{ChainService, DsignerWrapper, OnlySwapsSigner};
use anyhow::anyhow;
use dcipher_network::transports::in_memory::MemoryNetwork;
use dcipher_signer::BN254SignatureOnG1Signer;
use dcipher_signer::threshold_signer::ThresholdSigner;

pub(crate) fn create_bn254_signer<C: ChainService>(
    config: &AppConfig,
    network_bus: C,
) -> anyhow::Result<OnlySwapsSigner<C, DsignerWrapper<BN254SignatureOnG1Signer>>> {
    let bls_secret_key = &config.secret_key;
    let suite = BN254SignatureOnG1Signer::new(
        bls_secret_key.secret_key.clone().into(),
        b"BN254G1_XMD:KECCAK-256_SVDW_RO_H1_".to_vec(),
    );

    let signer = ThresholdSigner::new(
        suite,
        bls_secret_key.n.get(),
        bls_secret_key.t.get(),
        bls_secret_key.node_id.get(),
        config.group.nodes.iter().map(|n| n.bls_pk).collect(),
    );

    let transport = MemoryNetwork::get_transports(1..=1)
        .pop_front()
        .ok_or(anyhow!("failed to get transport"))?;

    let (_, threshold_signer) = signer.run(transport);
    let dsigner = DsignerWrapper::new(threshold_signer);

    Ok(OnlySwapsSigner::new(network_bus, dsigner))
}
