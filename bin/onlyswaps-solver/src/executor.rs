use crate::model::{RequestId, Trade};
use crate::network::Network;
use crate::util::normalise_chain_id;
use alloy::primitives::{Address, TxHash};
use alloy::providers::Provider;
use anyhow::{Context, anyhow};
use config::timeout::TimeoutConfig;
use generated::onlyswaps::erc20_faucet_token::ERC20FaucetToken::ERC20FaucetTokenInstance;
use generated::onlyswaps::errors_lib::ErrorsLib::ErrorsLibErrors;
use generated::onlyswaps::i_router::IRouter::IRouterInstance;
use generated::onlyswaps::ierc20_errors::IERC20Errors::IERC20ErrorsErrors as IERC20Errors;
use moka::future::Cache;
use std::collections::HashMap;
use tokio::time::timeout;

pub(crate) struct TradeExecutor<'a, P> {
    own_address: Address,
    routers: HashMap<u64, &'a IRouterInstance<P>>,
    tokens: HashMap<u64, &'a Vec<ERC20FaucetTokenInstance<P>>>,
}

impl<'a, P: Provider> TradeExecutor<'a, P> {
    pub fn new(networks: &'a HashMap<u64, Network<P>>) -> Self {
        let routers = networks
            .iter()
            .map(|(chain_id, net)| (*chain_id, &net.router))
            .collect();

        let tokens = networks
            .iter()
            .map(|(chain_id, net)| (*chain_id, &net.tokens))
            .collect();

        let own_address = networks
            .iter()
            .next()
            .map(|(_, network)| network.own_addr)
            .expect("if we don't have a network by now, something is very wrong");

        Self {
            routers,
            tokens,
            own_address,
        }
    }
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
            let router = self
                .routers
                .get(&normalise_chain_id(trade.dest_chain_id))
                .expect("somehow didn't have a router binding for a solved trade");
            let token = self
                .tokens
                .get(&normalise_chain_id(trade.dest_chain_id))
                .expect("somehow didn't have a token binding for a solved trade")
                .iter()
                .find(|contract| contract.address() == &trade.token_out_addr)
                .expect("somehow didn't have a token contract binding for a solved trade");

            // and finally execute the trade with a timeout
            match timeout(
                timeout_config.request_timeout,
                execute_trade(&trade, router, token, self.own_address),
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

async fn execute_trade(
    trade: &Trade,
    router: &IRouterInstance<impl Provider>,
    token: &ERC20FaucetTokenInstance<impl Provider>,
    own_addr: Address,
) -> anyhow::Result<TxHash> {
    // in theory, we shouldn't need to wait until the next block because txs will be processed in nonce order
    // but for whatever reason this doesn't seem to be the case :(
    let tx = token
        .approve(*router.address(), trade.amount_out)
        .send()
        .await
        .map_err(|e| {
            // Try to decode it as an IERC20 error
            if let Some(erc20_err) = e.as_decoded_interface_error::<IERC20Errors>() {
                return anyhow!("erc20 contract error: {erc20_err:?}");
            }
            e.into()
        })
        .context("error approving funds")?;
    tx.watch().await.context("error approving funds")?;

    let tx = router
        .relayTokens(
            own_addr,
            trade.request_id,
            trade.sender_addr,
            trade.recipient_addr,
            trade.token_in_addr,
            trade.token_out_addr,
            trade.amount_out,
            trade.src_chain_id,
            trade.nonce,
            trade.pre_hooks.to_vec(),
            trade.post_hooks.to_vec(),
        )
        .send()
        .await
        .map_err(|e| {
            // Try to decode it as an IERC20 error
            if let Some(erc20_err) = e.as_decoded_interface_error::<IERC20Errors>() {
                return anyhow!("erc20 contract error: {erc20_err:?}");
            }
            // Try to decode it as a Router error
            if let Some(router_err) = e.as_decoded_interface_error::<ErrorsLibErrors>() {
                return anyhow!("router contract error: {router_err:?}");
            }
            e.into()
        })
        .context("error submitting swap")?;

    // Fetch the receipt to get the tx status
    let receipt = tx.get_receipt().await.context("error submitting swap")?;
    if !receipt.status() {
        tracing::error!(?receipt, "error submitting swap: tx reverted");
        anyhow::bail!("error submitting swap: tx reverted");
    }

    Ok(receipt.transaction_hash)
}
