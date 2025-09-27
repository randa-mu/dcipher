use crate::chain_state::NetworkBus;
use crate::chain_state_pending::Verification;
use alloy::primitives::FixedBytes;
use alloy::providers::DynProvider;
use generated::onlyswaps::router::IRouter::SwapRequestParameters;
use generated::onlyswaps::router::Router::getSwapRequestReceiptReturn;
use std::sync::Arc;

pub struct ChainStateResolver {
    chain: Arc<NetworkBus<DynProvider>>,
}

#[derive(Debug, Clone)]
pub struct ChainState {
    pub transfer_receipt: getSwapRequestReceiptReturn,
    pub swap_params: SwapRequestParameters,
}

impl ChainStateResolver {
    pub fn new(chain: Arc<NetworkBus<DynProvider>>) -> Self {
        Self { chain }
    }

    pub async fn resolve_state(
        &self,
        verification_job: &Verification<FixedBytes<32>>,
    ) -> anyhow::Result<ChainState> {
        let transfer_receipt = self
            .chain
            .fetch_swap_receipt(verification_job.chain_id, verification_job.request_id)
            .await?;
        tracing::trace!("swap receipt received from dest chain");

        let swap_params = self
            .chain
            .fetch_swap_params(
                transfer_receipt.srcChainId.try_into()?,
                verification_job.request_id,
            )
            .await?;
        tracing::trace!("swap params received from src chain");

        Ok(ChainState {
            transfer_receipt,
            swap_params,
        })
    }
}
