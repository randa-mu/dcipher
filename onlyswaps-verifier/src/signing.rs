use crate::eth::IRouter::TransferParams;
use crate::parsing::{TransferReceipt, reconcile_transfer_params};
use crate::pending::{RequestId, Verification};
use crate::util::normalise_chain_id;
use alloy::primitives::FixedBytes;
use alloy::sol_types::SolValue;
use async_trait::async_trait;

pub struct OnlySwapsSigner<C, S> {
    chain: C,
    signer: S,
}

#[async_trait]
pub trait ChainService {
    async fn fetch_transfer_receipt(
        &self,
        chain_id: u64,
        request_id: FixedBytes<32>,
    ) -> eyre::Result<TransferReceipt>;
    async fn fetch_transfer_params(
        &self,
        chain_id: u64,
        request_id: FixedBytes<32>,
    ) -> eyre::Result<TransferParams>;
}
pub trait Signer {
    fn sign(&self, b: Vec<u8>) -> Vec<u8>;
}

impl<C, S> OnlySwapsSigner<C, S> {
    pub fn new(chain: C, signer: S) -> Self {
        Self { chain, signer }
    }
}

impl<C: ChainService, S: Signer> OnlySwapsSigner<C, S> {
    pub async fn try_sign(&self, verification_job: Verification<RequestId>) -> eyre::Result<Vec<u8>> {
        let transfer_receipt = self
            .chain
            .fetch_transfer_receipt(verification_job.chain_id, verification_job.request_id)
            .await?;

        let transfer_params = self
            .chain
            .fetch_transfer_params(
                normalise_chain_id(transfer_receipt.src_chain_id),
                verification_job.request_id,
            )
            .await?;

        let valid_transfer_params = reconcile_transfer_params(transfer_params, transfer_receipt)?;
        let m = create_message(valid_transfer_params);
        Ok(self.signer.sign(m))
    }
}

pub fn create_message(params: TransferParams) -> Vec<u8> {
    (
        params.sender,
        params.recipient,
        params.token,
        params.amount,
        params.srcChainId,
        params.dstChainId,
        params.swapFee,
        params.solverFee,
        params.nonce,
        params.executed,
    )
        .abi_encode()
}

#[cfg(test)]
mod test {
    use crate::eth::IRouter::TransferParams;
    use crate::parsing::TransferReceipt;
    use crate::pending::Verification;
    use crate::signing::{ChainService, OnlySwapsSigner, Signer};
    use alloy::primitives::{Address, FixedBytes, U160, U256};
    use async_trait::async_trait;
    use eyre::eyre;
    use speculoos::assert_that;

    #[tokio::test]
    async fn matching_receipt_and_params_create_valid_signature() {
        let destination_chain_id = 1;
        let request_id = FixedBytes::from(U256::from(1));
        let transfer_params = TransferParams {
            dstChainId: U256::from(destination_chain_id),
            sender: Address::from(U160::from(3)),
            recipient: Address::from(U160::from(3)),
            token: Address::from(U160::from(3)),
            amount: U256::from(10),
            srcChainId: U256::from(2),
            swapFee: U256::from(3),
            solverFee: U256::from(2),
            nonce: U256::from(1),
            executed: true,
        };

        let transfer_receipt = TransferReceipt {
            chain_id: U256::from(destination_chain_id),
            request_id,
            src_chain_id: U256::from(2),
            token: Address::from(U160::from(3)),
            fulfilled: true,
            solver: Address::from(U160::from(4)),
            recipient: Address::from(U160::from(5)),
            amount_out: U256::from(5), // amount - swapFee - solverFee
            fulfilled_at: U256::from(8),
        };

        let service = StubbedChainService::new(transfer_receipt, transfer_params);
        let onlyswaps = OnlySwapsSigner::new(service, StubbedSigner {});

        onlyswaps
            .try_sign(Verification {
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
        let transfer_params = TransferParams {
            dstChainId: U256::from(destination_chain_id),
            sender: Address::from(U160::from(3)),
            recipient: Address::from(U160::from(3)),
            token: Address::from(U160::from(3)),
            amount: U256::from(10),
            srcChainId: U256::from(2),
            swapFee: U256::from(3),
            solverFee: U256::from(2),
            nonce: U256::from(1),
            executed: true,
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

        let service = StubbedChainService::error(transfer_receipt, transfer_params, "oh shit".to_string());
        let onlyswaps = OnlySwapsSigner::new(service, StubbedSigner {});
        let result = onlyswaps
            .try_sign(Verification {
                chain_id: 1,
                request_id,
            })
            .await;

        assert_that!(result.is_err());
        assert_that!(result.unwrap_err().to_string()).is_equal_to("oh shit".to_string());
    }

    struct StubbedChainService {
        receipt: TransferReceipt,
        params: TransferParams,
        error: Option<String>,
    }

    impl StubbedChainService {
        fn new(receipt: TransferReceipt, params: TransferParams) -> Self {
            Self { receipt, params, error: None }
        }

        fn error(receipt: TransferReceipt, params: TransferParams, error: String) -> Self {
            Self { receipt, params, error: Some(error)}
        }
    }

    #[async_trait]
    impl ChainService for StubbedChainService {
        async fn fetch_transfer_receipt(
            &self,
            _: u64,
            _: FixedBytes<32>,
        ) -> eyre::Result<TransferReceipt> {
            if let Some(e) = &self.error {
                return Err(eyre!(e.to_string()));
            }
            Ok(self.receipt.clone())
        }

        async fn fetch_transfer_params(
            &self,
            _: u64,
            _: FixedBytes<32>,
        ) -> eyre::Result<TransferParams> {
            if let Some(e) = &self.error {
                return Err(eyre!(e.to_string()));
            }
            Ok(self.params.clone())
        }
    }

    struct StubbedSigner {}

    impl Signer for StubbedSigner {
        fn sign(&self, b: Vec<u8>) -> Vec<u8> {
            vec![0x1, 0x2, 0x3, 0x4]
        }
    }
}
