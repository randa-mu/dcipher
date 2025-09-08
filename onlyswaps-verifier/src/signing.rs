use crate::eth::IRouter::SwapRequestParameters;
use crate::parsing::{TransferReceipt, reconcile_transfer_params};
use crate::pending::{RequestId, Verification};
use crate::util::normalise_chain_id;
use alloy::primitives::{Address, FixedBytes};
use alloy::sol_types::SolValue;
use ark_serialize::{CanonicalSerialize, Compress};
use async_trait::async_trait;
use dcipher_signer::threshold_signer::AsyncThresholdSigner;
use dcipher_signer::{AsynchronousSigner, BN254SignatureOnG1Signer, BlsSigner};

pub struct OnlySwapsSigner<'a, C, S> {
    chain: &'a C,
    signer: &'a S,
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
    async fn sign(&self, b: Vec<u8>) -> anyhow::Result<Vec<u8>>;
}

impl<'a, C, S> OnlySwapsSigner<'a, C, S> {
    pub fn new(chain: &'a C, signer: &'a S) -> Self {
        Self { chain, signer }
    }
}

impl<C, S> OnlySwapsSigner<'_, C, S>
where
    C: ChainService,
    S: Signer,
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
        let m = create_message(valid_transfer_params);
        tracing::trace!("message for signing created");

        let signature = self.signer.sign(m).await?;
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

pub fn create_message(params: SwapRequestParameters) -> Vec<u8> {
    (
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
}

impl<S: BlsSigner> DsignerWrapper<S> {
    pub fn new(s: AsyncThresholdSigner<S>) -> Self {
        Self { s }
    }
}

#[async_trait]
impl Signer for DsignerWrapper<BN254SignatureOnG1Signer> {
    async fn sign(&self, message: Vec<u8>) -> anyhow::Result<Vec<u8>> {
        let point = self.s.async_sign(message).await?;
        let mut bytes = Vec::with_capacity(point.serialized_size(Compress::No));
        point.serialize_with_mode(&mut bytes, Compress::No)?;
        Ok(bytes)
    }
}

#[cfg(test)]
mod test {
    use crate::eth::IRouter::SwapRequestParameters;
    use crate::parsing::TransferReceipt;
    use crate::pending::Verification;
    use crate::signing::{ChainService, OnlySwapsSigner, Signer, VerifiedSwap};
    use alloy::primitives::{Address, FixedBytes, U160, U256};
    use async_trait::async_trait;
    use speculoos::assert_that;

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
        let onlyswaps = OnlySwapsSigner::new(&service, &StubbedSigner {});

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
        let onlyswaps = OnlySwapsSigner::new(&service, &stub_signer);
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
        async fn sign(&self, _: Vec<u8>) -> anyhow::Result<Vec<u8>> {
            Ok(vec![0x1, 0x2, 0x3, 0x4])
        }
    }
}
