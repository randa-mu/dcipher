use crate::config::AppConfig;
use alloy::primitives::{Address, FixedBytes, U256};
use alloy::sol_types::SolValue;
use anyhow::anyhow;
use async_trait::async_trait;
use dcipher_network::transports::libp2p::Libp2pNodeConfig;
use dcipher_signer::bls::{AsyncThresholdSigner, BlsPairingSigner, BlsSigner, BlsThresholdSigner};
use dcipher_signer::dsigner::{
    ApplicationArgs, BlsSignatureAlgorithm, BlsSignatureCurve, BlsSignatureHash,
    DSignerSchemeSigner, OnlySwapsVerifierArgs, SignatureAlgorithm, SignatureRequest,
};
use generated::onlyswaps::router::IRouter::SwapRequestParameters;
use std::collections::HashMap;
use std::sync::Arc;

pub struct OnlySwapsSigner<S> {
    signer: Arc<S>,
}

#[derive(Clone, Debug)]
pub struct SignedVerification {
    pub src_chain_id: U256,
    pub request_id: FixedBytes<32>,
    pub solver: Address,
    pub signature: Vec<u8>,
}

#[async_trait]
pub trait Signer {
    async fn sign(&self, b: Vec<u8>, chain_id: u64) -> anyhow::Result<Vec<u8>>;
}

impl<S> OnlySwapsSigner<S> {
    pub fn new(signer: impl Into<Arc<S>>) -> Self {
        Self {
            signer: signer.into(),
        }
    }
}

impl<S> OnlySwapsSigner<S>
where
    S: Signer + Send + Sync,
{
    pub async fn sign(
        &self,
        solver: &Address,
        valid_transfer_params: &SwapRequestParameters,
    ) -> anyhow::Result<Vec<u8>> {
        let src_chain_id = valid_transfer_params.srcChainId.try_into()?;
        let m = create_message(valid_transfer_params, solver);
        let signature = self.signer.sign(m, src_chain_id).await?;
        tracing::trace!("signing successful");

        Ok(signature)
    }
}

pub fn create_message(params: &SwapRequestParameters, solver: &Address) -> Vec<u8> {
    (
        solver,
        params.sender,
        params.recipient,
        params.tokenIn,
        params.tokenOut,
        params.amountOut,
        params.srcChainId,
        params.dstChainId,
        params.nonce,
    )
        .abi_encode()
}

pub struct NetworkedSigner<S: BlsSigner> {
    threshold_signer: AsyncThresholdSigner<S>,
}

impl NetworkedSigner<BlsPairingSigner<ark_bn254::Bn254>> {
    pub(crate) fn new(
        config: &AppConfig,
        libp2p_node: Libp2pNodeConfig<u16>,
    ) -> anyhow::Result<NetworkedSigner<BlsPairingSigner<ark_bn254::Bn254>>> {
        let pairing_signer =
            BlsPairingSigner::<ark_bn254::Bn254>::new(config.committee_config.secret_key.clone().0);
        let signer = BlsThresholdSigner::new(
            pairing_signer,
            config.committee_config.n.get(),
            config.committee_config.signing_threshold.get(),
            config.committee_config.member_id.get(),
            HashMap::default(), // no keys on g1
            config
                .committee_config
                .members
                .iter()
                .map(|n| (n.member_id.get(), n.bls_pk))
                .collect(),
        );

        let transport = libp2p_node
            .run(config.listen_addr.clone())?
            .get_transport()
            .ok_or(anyhow!("failed to get libp2p transport"))?;
        let (_, threshold_signer) = signer.run(transport);

        Ok(Self { threshold_signer })
    }
}

#[async_trait]
impl Signer for NetworkedSigner<BlsPairingSigner<ark_bn254::Bn254>> {
    async fn sign(&self, message: Vec<u8>, chain_id: u64) -> anyhow::Result<Vec<u8>> {
        // Sign a message using a dst in the following format:
        //  swap-v1-BN254G1_XMD:KECCAK-256_SVDW_RO_0x0000000000000000000000000000000000000000000000000000000000014a34_
        let sig_request = SignatureRequest {
            m: message.into(),
            args: ApplicationArgs::OnlySwapsVerifier(OnlySwapsVerifierArgs { chain_id }),
            alg: SignatureAlgorithm::Bls(BlsSignatureAlgorithm {
                curve: BlsSignatureCurve::Bn254G1,
                hash: BlsSignatureHash::Keccak256,
                compression: false,
            }),
        };
        let point = self.threshold_signer.async_sign(sig_request).await?;
        Ok(point.into())
    }
}

impl<S> Clone for OnlySwapsSigner<S>
where
    S: Signer,
{
    fn clone(&self) -> Self {
        Self {
            signer: self.signer.clone(),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::signing::{NetworkedSigner, OnlySwapsSigner, Signer};
    use alloy::primitives::{Address, FixedBytes, U160, U256};
    use ark_bn254::Fr;
    use ark_ff::MontFp;
    use async_trait::async_trait;
    use dcipher_network::transports::in_memory::MemoryNetwork;
    use dcipher_signer::bls::{BlsPairingSigner, BlsThresholdSigner};
    use speculoos::assert_that;
    use std::collections::HashMap;

    #[tokio::test]
    async fn matching_receipt_and_params_create_valid_signature() {
        let destination_chain_id = 1;
        let request_id = FixedBytes::from(U256::from(1));
        let transfer_params = SwapRequestParameters {
            dstChainId: U256::from(destination_chain_id),
            sender: Address::from(U160::from(3)),
            recipient: Address::from(U160::from(5)),
            tokenIn: Address::from(U160::from(3)),
            tokenOut: Address::from(U160::from(4)),
            amountOut: U256::from(10),
            srcChainId: U256::from(2),
            verificationFee: U256::from(3),
            solverFee: U256::from(2),
            nonce: U256::from(1),
            executed: true,
            requestedAt: U256::from(123456),
        };

        let transfer_receipt = getSwapRequestReceiptReturn {
            dstChainId: U256::from(destination_chain_id),
            requestId: request_id,
            srcChainId: U256::from(2),
            tokenIn: Address::from(U160::from(3)),
            tokenOut: Address::from(U160::from(4)),
            fulfilled: true,
            solver: Address::from(U160::from(4)),
            recipient: Address::from(U160::from(5)),
            amountOut: U256::from(10),
            fulfilledAt: U256::from(8),
        };

        let onlyswaps = OnlySwapsSigner::new(StubbedSigner::default());

        onlyswaps
            .sign(&transfer_receipt.solver, &transfer_params)
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn signing_errors_propagate() {
        let destination_chain_id = 1;
        let request_id = FixedBytes::from(U256::from(1));
        let transfer_params = SwapRequestParameters {
            dstChainId: U256::from(destination_chain_id),
            sender: Address::from(U160::from(3)),
            recipient: Address::from(U160::from(3)),
            tokenIn: Address::from(U160::from(3)),
            tokenOut: Address::from(U160::from(3)),
            amountOut: U256::from(10),
            srcChainId: U256::from(2),
            verificationFee: U256::from(3),
            solverFee: U256::from(2),
            nonce: U256::from(1),
            executed: true,
            requestedAt: U256::from(123456),
        };

        let transfer_receipt = getSwapRequestReceiptReturn {
            dstChainId: U256::from(destination_chain_id),
            requestId: request_id,
            srcChainId: U256::from(2),
            tokenIn: Address::from(U160::from(3)),
            tokenOut: Address::from(U160::from(4)),
            fulfilled: true,
            recipient: Address::from(U160::from(5)),
            solver: Address::from(U160::from(4)),
            amountOut: U256::from(5), // amount - swapFee - solverFee
            fulfilledAt: U256::from(6),
        };

        let stub_signer = StubbedSigner { should_error: true };
        let onlyswaps = OnlySwapsSigner::new(stub_signer);
        let result = onlyswaps
            .sign(&transfer_receipt.solver, &transfer_params)
            .await;

        assert_that!(result.is_err());
        assert_that!(result.unwrap_err().to_string()).is_equal_to("boom!".to_string());
    }

    #[derive(Default)]
    struct StubbedSigner {
        should_error: bool,
    }

    #[async_trait]
    impl Signer for StubbedSigner {
        async fn sign(&self, _: Vec<u8>, _: u64) -> anyhow::Result<Vec<u8>> {
            if self.should_error {
                anyhow::bail!("boom!")
            }
            Ok(vec![0x1, 0x2, 0x3, 0x4])
        }
    }

    use ark_ec::{AffineRepr, CurveGroup};
    use futures::future::try_join_all;
    use generated::onlyswaps::router::IRouter::SwapRequestParameters;
    use generated::onlyswaps::router::Router::getSwapRequestReceiptReturn;

    #[tokio::test]
    async fn in_memory_test() -> anyhow::Result<()> {
        let n = 3;
        let t = 2;
        let g1 = ark_bn254::G1Affine::generator();
        let g2 = ark_bn254::G2Affine::generator();

        let _sk: Fr =
            MontFp!("7685086713915354683875500702831995067084988389812060097318430034144315778947");
        let sk1: Fr =
            MontFp!("5840327440053394277204603653048962762290958051681898697354171413163183818203");
        let sk2: Fr =
            MontFp!("3995568166191433870533706603265930457496927713551737297389912792182051857459");
        let sk3: Fr =
            MontFp!("2150808892329473463862809553482898152702897375421575897425654171200919896715");

        let pks_g2 = vec![g2 * sk1, g2 * sk2, g2 * sk3];
        let pks_g2 = pks_g2
            .into_iter()
            .enumerate()
            .map(|(i, pki)| (i as u16 + 1, pki.into_affine()))
            .collect::<HashMap<_, _>>();

        let pks_g1 = vec![g1 * sk1, g1 * sk2, g1 * sk3];
        let pks_g1 = pks_g1
            .into_iter()
            .enumerate()
            .map(|(i, pki)| (i as u16 + 1, pki.into_affine()))
            .collect::<HashMap<_, _>>();

        let cs1 = BlsPairingSigner::new_bn254(sk1);
        let cs2 = BlsPairingSigner::new_bn254(sk2);
        let cs3 = BlsPairingSigner::new_bn254(sk3);

        // Get transports
        let mut transports = MemoryNetwork::get_transports(1..=3);

        // Start three threshold signers
        let (_, ch1) =
            BlsThresholdSigner::new(cs1.clone(), n, t, 1, pks_g1.clone(), pks_g2.clone())
                .run(transports.pop_front().unwrap());
        let (_, ch2) = BlsThresholdSigner::new(cs2, n, t, 2, pks_g1.clone(), pks_g2.clone())
            .run(transports.pop_front().unwrap());
        let (_, ch3) = BlsThresholdSigner::new(cs3, n, t, 3, pks_g1.clone(), pks_g2.clone())
            .run(transports.pop_front().unwrap());

        let s1 = NetworkedSigner {
            threshold_signer: ch1,
        };
        let s2 = NetworkedSigner {
            threshold_signer: ch2,
        };
        let s3 = NetworkedSigner {
            threshold_signer: ch3,
        };
        let m = b"hello world";

        let futs = vec![
            s1.sign(m.to_vec(), 1),
            s2.sign(m.to_vec(), 1),
            s3.sign(m.to_vec(), 1),
        ];
        try_join_all(futs).await?;

        Ok(())
    }
}
