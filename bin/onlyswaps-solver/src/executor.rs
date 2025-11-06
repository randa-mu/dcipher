use crate::gasless::{Permit2RelayTokensDetails, permit2_relay_tokens_details};
use crate::model::{RequestId, Trade};
use crate::network::Network;
use crate::profitability::{ErasedProfitabilityEstimator, ProfitabilityEstimator};
use crate::util::normalise_chain_id;
use alloy::primitives::{Address, TxHash};
use alloy::providers::Provider;
use alloy::signers::Signer;
use anyhow::{Context, anyhow};
use config::timeout::TimeoutConfig;
use generated::onlyswaps::errors_lib::ErrorsLib::ErrorsLibErrors;
use generated::onlyswaps::i_router::IRouter::{IRouterInstance, RelayTokensPermit2Params};
use generated::onlyswaps::ierc20_errors::IERC20Errors::IERC20ErrorsErrors as IERC20Errors;
use moka::future::Cache;
use std::collections::HashMap;
use tokio::time::timeout;

pub(crate) struct TradeExecutor<'a, P, S> {
    signer: S,
    own_address: Address,
    routers: HashMap<u64, ChainConfig<'a, P>>,
    profitability_estimator: ErasedProfitabilityEstimator,
}

pub(crate) struct ChainConfig<'a, P> {
    router: &'a IRouterInstance<P>,
    permit2_relayer_address: Address,
}

impl<'a, P, S> TradeExecutor<'a, P, S> {
    pub fn new(
        signer: S,
        networks: &'a HashMap<u64, Network<P>>,
        profitability_estimator: ErasedProfitabilityEstimator,
    ) -> Self {
        let routers = networks
            .iter()
            .map(|(chain_id, net)| {
                (
                    *chain_id,
                    ChainConfig {
                        router: &net.router,
                        permit2_relayer_address: net.permit2_relayer_address,
                    },
                )
            })
            .collect();

        let own_address = networks
            .iter()
            .next()
            .map(|(_, network)| network.own_addr)
            .expect("if we don't have a network by now, something is very wrong");

        Self {
            signer,
            routers,
            own_address,
            profitability_estimator,
        }
    }
}

impl<'a, P, S> TradeExecutor<'a, P, S>
where
    P: Provider,
    S: Signer,
{
    pub async fn execute(
        &self,
        trades: Vec<Trade>,
        in_flight: &mut Cache<RequestId, ()>,
        timeout_config: &TimeoutConfig,
    ) {
        for trade in trades {
            // first we add the trade to the cache so that we don't retry it in the next block
            // (before it's been finalised, potentially)
            in_flight.insert(trade.request_id, ()).await;

            // then we get the contract bindings for the destination chain
            let config = self
                .routers
                .get(&normalise_chain_id(trade.dest_chain_id))
                .expect("somehow didn't have a router binding for a solved trade");

            // and finally execute the trade with a timeout
            match timeout(
                timeout_config.request_timeout,
                execute_trade(
                    &trade,
                    config.router,
                    config.permit2_relayer_address,
                    self.own_address,
                    &self.signer,
                    &self.profitability_estimator,
                ),
            )
            .await
            {
                Ok(Ok(_)) => {
                    tracing::info!(
                        amount_in = ?trade.amount_in,
                        amount_out = ?trade.amount_out,
                        src_chain_id = ?trade.src_chain_id,
                        dest_chain_id = ?trade.dest_chain_id,
                        request_id = %trade.request_id,
                        "successfully traded"
                    );
                }
                Ok(Err(e)) => {
                    tracing::error!(
                        amount_in = ?trade.amount_in,
                        amount_out = ?trade.amount_out,
                        src_chain_id = ?trade.src_chain_id,
                        dest_chain_id = ?trade.dest_chain_id,
                        request_id = ?trade.request_id,
                        error = ?e,
                        "error trading",
                    );
                }
                Err(_) => {
                    tracing::error!(request_id = ?trade.request_id,"trade timed out");
                    in_flight.remove(&trade.request_id).await;
                }
            }
        }
    }
}

async fn execute_trade<S>(
    trade: &Trade,
    router: &IRouterInstance<impl Provider>,
    permit2_relayer_address: Address,
    own_addr: Address,
    signer: &S,
    profitability_estimator: &ErasedProfitabilityEstimator,
) -> anyhow::Result<TxHash>
where
    S: Signer,
{
    let Permit2RelayTokensDetails {
        message_hash,
        nonce: permit_nonce,
        deadline: permit_deadline,
    } = permit2_relay_tokens_details(trade, permit2_relayer_address, own_addr)?;
    let permit2_signed_allowance = signer.sign_hash(&message_hash).await?;

    let relay_tokens_call = router.relayTokensPermit2(RelayTokensPermit2Params {
        solver: own_addr,
        solverRefundAddress: own_addr,
        requestId: trade.request_id,
        sender: trade.sender_addr,
        recipient: trade.recipient_addr,
        tokenIn: trade.token_in_addr,
        tokenOut: trade.token_out_addr,
        amountOut: trade.amount_out,
        srcChainId: trade.src_chain_id,
        nonce: trade.nonce,
        permitNonce: permit_nonce,
        permitDeadline: permit_deadline,
        signature: permit2_signed_allowance.as_erc2098().into(),
        preHooks: trade.pre_hooks.to_vec(),
        postHooks: trade.post_hooks.to_vec(),
    });

    let gas = relay_tokens_call
        .clone()
        .estimate_gas()
        .await
        .map_err(decode_irouter_error)
        .context("gas estimation failed")?;

    // Currently, we cannot check for profitability earlier, due to the allowance check.
    let gas_cost = estimate_gas_cost(router.provider())
        .await
        .context("gas cost estimation failed")?;
    if !profitability_estimator
        .is_profitable(trade, gas, gas_cost)
        .await
        .context("failed to compute profitability of trade")?
    {
        tracing::warn!("Trade not profitable, refusing fulfillment");
        anyhow::bail!("trade not profitable");
    }

    let tx = relay_tokens_call
        .send()
        .await
        .map_err(decode_irouter_error)
        .context("failed to send relayTokens tx")?;

    // Fetch the receipt to get the tx status
    let receipt = tx.get_receipt().await.context("error submitting swap")?;
    if !receipt.status() {
        tracing::error!(?receipt, "error submitting swap: tx reverted");
        anyhow::bail!("error submitting swap: tx reverted");
    }

    Ok(receipt.transaction_hash)
}

fn decode_irouter_error(e: alloy::contract::Error) -> anyhow::Error {
    // Try to decode it as an IERC20 error
    if let Some(erc20_err) = e.as_decoded_interface_error::<IERC20Errors>() {
        return anyhow!("erc20 contract error: {erc20_err:?}");
    }
    // Try to decode it as a Router error
    if let Some(router_err) = e.as_decoded_interface_error::<ErrorsLibErrors>() {
        return anyhow!("router contract error: {router_err:?}");
    }
    e.into()
}

/// Get an upper bound estimation of the current gas cost from the provider
async fn estimate_gas_cost(provider: &impl Provider) -> anyhow::Result<u128> {
    let gas_cost = match provider.estimate_eip1559_fees().await {
        Ok(fees) => fees.max_fee_per_gas,
        Err(e) => {
            tracing::warn!(
                error = ?e,
                "Failed to estimate eip1559 fees, falling back to legacy estimation"
            );
            provider
                .get_gas_price()
                .await
                .context("failed to get gas price")?
        }
    };

    Ok(gas_cost)
}
