//! Functions of the onlyswaps-client used by solvers.

use crate::client::{OnlySwapsClient, OnlySwapsClientError};
use alloy::primitives::{Address, B256, U256};
use alloy::rpc::types::TransactionReceipt;
use generated::onlyswaps::i_router::IRouter::{Hook, IRouterInstance};

#[derive(Clone, Debug)]
pub struct OnlySwapsTrade {
    pub token_in_addr: Address,
    pub token_out_addr: Address,
    pub src_chain_id: u64,
    pub dest_chain_id: u64,
    pub sender_addr: Address,
    pub recipient_addr: Address,
    pub amount_out: U256,
    pub nonce: U256,
    pub request_id: B256,
    pub pre_hooks: Vec<Hook>,
    pub post_hooks: Vec<Hook>,
}

impl OnlySwapsClient {
    /// Estimate the gas limit of a relayTokens call.
    pub async fn estimate_relay_tokens(
        &self,
        trade: OnlySwapsTrade,
        refund_addr: Address,
    ) -> Result<u64, OnlySwapsClientError> {
        let chain_config = self
            .config
            .get_chain_config(trade.dest_chain_id)
            .ok_or(OnlySwapsClientError::UnsupportedChain(trade.dest_chain_id))?;

        let provider = self
            .config
            .get_ethereum_provider(trade.dest_chain_id)
            .ok_or(OnlySwapsClientError::MissingProvider(trade.dest_chain_id))?;
        let router = IRouterInstance::new(chain_config.router_address, provider);

        let gas_estimate = router
            .relayTokens(
                refund_addr,
                trade.request_id,
                trade.sender_addr,
                trade.recipient_addr,
                trade.token_in_addr,
                trade.token_out_addr,
                trade.amount_out,
                U256::from(trade.src_chain_id),
                trade.nonce,
                trade.pre_hooks,
                trade.post_hooks,
            )
            .estimate_gas()
            .await
            .map_err(|e| (e, "failed to estimate gas for relayTokens"))?;

        Ok(gas_estimate)
    }

    /// Call the router's relayTokens function.
    /// Tokens must be approved to be spent by the router prior to calling this function.
    pub async fn relay_tokens(
        &self,
        trade: OnlySwapsTrade,
        refund_addr: Address,
    ) -> Result<TransactionReceipt, OnlySwapsClientError> {
        let chain_config = self
            .config
            .get_chain_config(trade.dest_chain_id)
            .ok_or(OnlySwapsClientError::UnsupportedChain(trade.dest_chain_id))?;

        let provider = self
            .config
            .get_ethereum_provider(trade.dest_chain_id)
            .ok_or(OnlySwapsClientError::MissingProvider(trade.dest_chain_id))?;
        let router = IRouterInstance::new(chain_config.router_address, provider);

        let tx_hash = router
            .relayTokens(
                refund_addr,
                trade.request_id,
                trade.sender_addr,
                trade.recipient_addr,
                trade.token_in_addr,
                trade.token_out_addr,
                trade.amount_out,
                U256::from(trade.src_chain_id),
                trade.nonce,
                trade.pre_hooks,
                trade.post_hooks,
            )
            .send()
            .await
            .map_err(|e| (e, "failed to send approve tx"))?
            .with_required_confirmations(chain_config.required_confirmations)
            .with_timeout(Some(chain_config.timeout))
            .watch()
            .await?;

        // Fetch the receipt to get the tx status
        let receipt = super::backoff::get_receipt(tx_hash, router.provider()).await?;
        if !receipt.status() {
            tracing::error!(?receipt, "error submitting relayTokens: tx reverted");
            return Err(OnlySwapsClientError::RelayTokensReverted);
        }

        Ok(receipt)
    }
}
