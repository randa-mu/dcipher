use crate::eth::IRouter::SwapRequestParameters;
use crate::parsing::{TransferReceipt, reconcile_transfer_params};
use crate::pending::{RequestId, Verification};
use crate::util::normalise_chain_id;
use alloy::primitives::{Address, FixedBytes};
use alloy::sol_types::SolValue;
use async_trait::async_trait;
use dcipher_signer::bls::{AsyncThresholdSigner, BlsPairingSigner, BlsSigner};
use dcipher_signer::dsigner::{
    ApplicationArgs, BlsSignatureAlgorithm, BlsSignatureCurve, BlsSignatureHash,
    DSignerSchemeSigner, OnlySwapsVerifierArgs, SignatureAlgorithm, SignatureRequest,
};
use std::marker::PhantomData;
use std::sync::Arc;

pub struct OnlySwapsSigner<C, S> {
    chain: Arc<C>,
    signer: Arc<S>,
}

#[async_trait]
pub trait ChainService {
    async fn fetch_transfer_receipt(
        &self,
        chain_id: u64,
        request_id: FixedBytes<32>,
    ) -> anyhow::Result<TransferReceipt>;
    async fn fetch_transfer_params(
        &self,
        chain_id: u64,
        request_id: FixedBytes<32>,
    ) -> anyhow::Result<SwapRequestParameters>;

    async fn submit_verification(
        &self,
        chain_id: u64,
        verified_swap: &VerifiedSwap,
    ) -> anyhow::Result<()>;
}

#[derive(Clone, Debug)]
pub struct VerifiedSwap {
    pub request_id: FixedBytes<32>,
    pub solver: Address,
    pub signature: Vec<u8>,
}

#[async_trait]
pub trait Signer {
    async fn sign(&self, b: Vec<u8>, chain_id: u64) -> anyhow::Result<Vec<u8>>;
}

impl<C, S> OnlySwapsSigner<C, S> {
    pub fn new(chain: impl Into<Arc<C>>, signer: impl Into<Arc<S>>) -> Self {
        Self {
            chain: chain.into(),
            signer: signer.into(),
        }
    }
}

impl<C, S> OnlySwapsSigner<C, S>
where
    C: ChainService + Send + Sync,
    S: Signer + Send + Sync,
{
    pub async fn evaluate_and_send(
        &self,
        verification_job: &Verification<RequestId>,
    ) -> anyhow::Result<VerifiedSwap> {
        let transfer_receipt = self
            .chain
            .fetch_transfer_receipt(verification_job.chain_id, verification_job.request_id)
            .await?;
        tracing::trace!("transfer receipt received from dest chain");

        let transfer_params = self
            .chain
            .fetch_transfer_params(
                normalise_chain_id(transfer_receipt.src_chain_id),
                verification_job.request_id,
            )
            .await?;
        tracing::trace!("transfer params received from src chain");

        let solver = transfer_receipt.solver;
        let valid_transfer_params = reconcile_transfer_params(transfer_params, transfer_receipt)?;
        let src_chain_id = normalise_chain_id(valid_transfer_params.srcChainId);
        let m = create_message(valid_transfer_params, &solver);
        tracing::trace!("message for signing created");

        let signature = self.signer.sign(m, src_chain_id).await?;
        let verified_swap = VerifiedSwap {
            request_id: verification_job.request_id,
            signature,
            solver,
        };
        tracing::trace!("signing complete");

        self.chain
            .submit_verification(src_chain_id, &verified_swap)
            .await?;
        tracing::trace!("verification submitted successfully");

        Ok(verified_swap)
    }
}

pub fn create_message(params: SwapRequestParameters, solver: &Address) -> Vec<u8> {
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

pub struct DsignerWrapper<S: BlsSigner> {
    s: AsyncThresholdSigner<S>,
    _s: PhantomData<S>, // todo: remove it if signer is never required
}

impl<S: BlsSigner> DsignerWrapper<S> {
    pub fn new(s: AsyncThresholdSigner<S>) -> Self {
        Self { s, _s: PhantomData }
    }
}

#[async_trait]
impl Signer for DsignerWrapper<BlsPairingSigner<ark_bn254::Bn254>> {
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
        let point = self.s.async_sign(sig_request).await?;
        Ok(point.into())
    }
}

impl<C, S> Clone for OnlySwapsSigner<C, S>
where
    C: ChainService,
    S: Signer,
{
    fn clone(&self) -> Self {
        Self {
            chain: self.chain.clone(),
            signer: self.signer.clone(),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::eth::IRouter::SwapRequestParameters;
    use crate::parsing::TransferReceipt;
    use crate::pending::Verification;
    use crate::signing::{ChainService, DsignerWrapper, OnlySwapsSigner, Signer, VerifiedSwap};
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
            tokenOut: Address::from(U160::from(3)),
            amountOut: U256::from(10),
            srcChainId: U256::from(2),
            verificationFee: U256::from(3),
            solverFee: U256::from(2),
            nonce: U256::from(1),
            executed: true,
            requestedAt: U256::from(123456),
        };

        let transfer_receipt = TransferReceipt {
            chain_id: U256::from(destination_chain_id),
            request_id,
            src_chain_id: U256::from(2),
            token: Address::from(U160::from(3)),
            fulfilled: true,
            solver: Address::from(U160::from(4)),
            recipient: Address::from(U160::from(5)),
            amount_out: U256::from(10),
            fulfilled_at: U256::from(8),
        };

        let service = StubbedChainService::new(transfer_receipt, transfer_params);
        let onlyswaps = OnlySwapsSigner::new(service, StubbedSigner {});

        onlyswaps
            .evaluate_and_send(&Verification {
                chain_id: 1,
                request_id,
            })
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn contract_errors_propagate() {
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

        let transfer_receipt = TransferReceipt {
            chain_id: U256::from(destination_chain_id),
            request_id,
            src_chain_id: U256::from(2),
            token: Address::from(U160::from(3)),
            fulfilled: true,

            recipient: Address::from(U160::from(5)),
            solver: Address::from(U160::from(4)),
            amount_out: U256::from(5), // amount - swapFee - solverFee
            fulfilled_at: U256::from(6),
        };

        let service =
            StubbedChainService::error(transfer_receipt, transfer_params, "oh shit".to_string());
        let stub_signer = StubbedSigner {};
        let onlyswaps = OnlySwapsSigner::new(service, stub_signer);
        let result = onlyswaps
            .evaluate_and_send(&Verification {
                chain_id: 1,
                request_id,
            })
            .await;

        assert_that!(result.is_err());
        assert_that!(result.unwrap_err().to_string()).is_equal_to("oh shit".to_string());
    }

    struct StubbedChainService {
        receipt: TransferReceipt,
        params: SwapRequestParameters,
        error: Option<String>,
    }

    impl StubbedChainService {
        fn new(receipt: TransferReceipt, params: SwapRequestParameters) -> Self {
            Self {
                receipt,
                params,
                error: None,
            }
        }

        fn error(receipt: TransferReceipt, params: SwapRequestParameters, error: String) -> Self {
            Self {
                receipt,
                params,
                error: Some(error),
            }
        }
    }

    #[async_trait]
    impl ChainService for StubbedChainService {
        async fn fetch_transfer_receipt(
            &self,
            _: u64,
            _: FixedBytes<32>,
        ) -> anyhow::Result<TransferReceipt> {
            if let Some(e) = &self.error {
                anyhow::bail!(e.to_string());
            }
            Ok(self.receipt.clone())
        }

        async fn fetch_transfer_params(
            &self,
            _: u64,
            _: FixedBytes<32>,
        ) -> anyhow::Result<SwapRequestParameters> {
            if let Some(e) = &self.error {
                anyhow::bail!(e.to_string());
            }
            Ok(self.params.clone())
        }

        async fn submit_verification(&self, _: u64, _: &VerifiedSwap) -> anyhow::Result<()> {
            Ok(())
        }
    }

    struct StubbedSigner {}

    #[async_trait]
    impl Signer for StubbedSigner {
        async fn sign(&self, _: Vec<u8>, _: u64) -> anyhow::Result<Vec<u8>> {
            Ok(vec![0x1, 0x2, 0x3, 0x4])
        }
    }

    use ark_ec::{AffineRepr, CurveGroup};
    use futures::future::try_join_all;

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

        let s1 = DsignerWrapper::new(ch1);
        let s2 = DsignerWrapper::new(ch2);
        let s3 = DsignerWrapper::new(ch3);
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
