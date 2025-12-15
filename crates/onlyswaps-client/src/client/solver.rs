//! Functions of the onlyswaps-client used by solvers.

#[cfg(feature = "permit2")]
mod gasless;

use crate::client::{OnlySwapsClient, OnlySwapsClientError};
use alloy::primitives::{Address, B256, U256};
use alloy::rpc::types::TransactionReceipt;
use futures_util::FutureExt;
use futures_util::future::BoxFuture;
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
        let chain = self
            .config
            .get_chain(trade.dest_chain_id)
            .ok_or(OnlySwapsClientError::UnsupportedChain(trade.dest_chain_id))?;

        let provider = self
            .config
            .get_ethereum_provider(trade.dest_chain_id)
            .ok_or(OnlySwapsClientError::MissingProvider(trade.dest_chain_id))?;
        let router = IRouterInstance::new(chain.config.router_address, provider);

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
    ) -> Result<SendableTransaction<'_>, OnlySwapsClientError> {
        let chain = self
            .config
            .get_chain(trade.dest_chain_id)
            .ok_or(OnlySwapsClientError::UnsupportedChain(trade.dest_chain_id))?;

        let provider = self
            .config
            .get_ethereum_provider(trade.dest_chain_id)
            .ok_or(OnlySwapsClientError::MissingProvider(trade.dest_chain_id))?;
        let router = IRouterInstance::new(chain.config.router_address, provider.to_owned());

        let call = router
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
            .with_cloned_provider();

        let gas_estimate = call
            .clone()
            .estimate_gas()
            .await
            .map_err(|e| (e, "failed to estimate gas for relayTokens"))?;

        let send_fut = async move {
            let tx_hash = call
                .send()
                .await
                .map_err(|e| (e, "failed to send relayTokens tx"))?
                .with_required_confirmations(chain.config.required_confirmations)
                .with_timeout(Some(chain.config.timeout))
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
        .boxed();

        Ok(SendableTransaction {
            gas_estimate,
            send_fut,
        })
    }

    /// Fetch the permit2 addresses used by relayer contracts
    #[cfg(feature = "permit2")]
    pub async fn fetch_permit2_addresses(
        &self,
        relayers: impl IntoIterator<Item = (u64, Address)>,
    ) -> Result<impl Iterator<Item = (u64, Address)>, OnlySwapsClientError> {
        use generated::onlyswaps::permit2_relayer::Permit2Relayer::Permit2RelayerInstance;

        let permit2_addresses = futures_util::future::try_join_all(relayers.into_iter().map(
            async |(id, relayer_addr)| -> Result<_, OnlySwapsClientError> {
                let provider = self
                    .config
                    .get_ethereum_provider(id)
                    .ok_or(OnlySwapsClientError::UnsupportedChain(id))?;

                let permit2_addr = Permit2RelayerInstance::new(relayer_addr, provider)
                    .PERMIT2()
                    .call()
                    .await
                    .map_err(|e| (e, "failed to execute PERMIT2 RPC static call"))?;
                Ok((id, permit2_addr))
            },
        ))
        .await?;

        Ok(permit2_addresses.into_iter())
    }

    /// Call the router's relayTokens function.
    /// Tokens must be approved to be spent by the router prior to calling this function.
    #[cfg(feature = "permit2")]
    pub async fn relay_tokens_permit2(
        &self,
        trade: &OnlySwapsTrade,
        refund_addr: Address,
        permit2_addr: Address,
        permit2_relayer_addr: Address,
        signer: &impl alloy::signers::Signer,
    ) -> Result<SendableTransaction<'_>, OnlySwapsClientError> {
        use crate::client::solver::gasless::{
            Permit2RelayTokensDetails, permit2_relay_tokens_details,
        };
        use futures_util::FutureExt;
        use generated::onlyswaps::i_router::IRouter::RelayTokensPermit2Params;

        let chain = self
            .config
            .get_chain(trade.dest_chain_id)
            .ok_or(OnlySwapsClientError::UnsupportedChain(trade.dest_chain_id))?;

        let provider = self
            .config
            .get_ethereum_provider(trade.dest_chain_id)
            .ok_or(OnlySwapsClientError::MissingProvider(trade.dest_chain_id))?;
        let router = IRouterInstance::new(chain.config.router_address, provider.to_owned());
        let own_addr = signer.address();

        let Permit2RelayTokensDetails {
            message_hash,
            nonce: permit_nonce,
            deadline: permit_deadline,
        } = permit2_relay_tokens_details(
            trade,
            permit2_relayer_addr,
            own_addr,
            Some(permit2_addr),
        )?;
        let permit2_signed_allowance = signer.sign_hash(&message_hash).await?;
        let call = router
            .relayTokensPermit2(RelayTokensPermit2Params {
                solver: own_addr,
                solverRefundAddress: refund_addr,
                requestId: trade.request_id,
                sender: trade.sender_addr,
                recipient: trade.recipient_addr,
                tokenIn: trade.token_in_addr,
                tokenOut: trade.token_out_addr,
                amountOut: trade.amount_out,
                srcChainId: U256::from(trade.src_chain_id),
                nonce: trade.nonce,
                permitNonce: permit_nonce,
                permitDeadline: permit_deadline,
                signature: permit2_signed_allowance.as_erc2098().into(),
                preHooks: trade.pre_hooks.to_vec(),
                postHooks: trade.post_hooks.to_vec(),
            })
            .with_cloned_provider();

        let gas_estimate = call
            .clone()
            .estimate_gas()
            .await
            .map_err(|e| (e, "failed to estimate gas for relayTokens"))?;

        let send_fut = async move {
            let tx_hash = call
                .send()
                .await
                .map_err(|e| (e, "failed to send relayTokensPermit2 tx"))?
                .with_required_confirmations(chain.config.required_confirmations)
                .with_timeout(Some(chain.config.timeout))
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
        .boxed();

        Ok(SendableTransaction {
            gas_estimate,
            send_fut,
        })
    }
}

pub struct SendableTransaction<'a> {
    gas_estimate: u64,
    send_fut: BoxFuture<'a, Result<TransactionReceipt, OnlySwapsClientError>>,
}

impl SendableTransaction<'_> {
    pub fn gas_estimate(&self) -> u64 {
        self.gas_estimate
    }

    pub async fn send(self) -> Result<TransactionReceipt, OnlySwapsClientError> {
        self.send_fut.await
    }
}
