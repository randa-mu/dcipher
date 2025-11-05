use crate::gasless::{Permit2RelayTokensDetails, permit2, permit2_relay_tokens_details};
use crate::model::{RequestId, Trade};
use crate::network::Network;
use crate::util::normalise_chain_id;
use alloy::primitives::{Address, TxHash};
use alloy::providers::Provider;
use alloy::signers::Signer;
use anyhow::{Context, anyhow};
use generated::onlyswaps::router::Router::{RouterErrors, RouterInstance};
use moka::future::Cache;
use std::collections::HashMap;
use std::time::Duration;
use tokio::time::timeout;

pub(crate) struct TradeExecutor<'a, P, S> {
    signer: S,
    own_address: Address,
    routers: HashMap<u64, ChainConfig<'a, P>>,
}

pub(crate) struct ChainConfig<'a, P> {
    router: &'a RouterInstance<P>,
    permit2_relayer_address: Address,
}

impl<'a, P, S> TradeExecutor<'a, P, S> {
    pub fn new(signer: S, networks: &'a HashMap<u64, Network<P>>) -> Self {
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
        }
    }
}

impl<'a, P, S> TradeExecutor<'a, P, S>
where
    P: Provider,
    S: Signer,
{
    pub async fn execute(&self, trades: Vec<Trade>, in_flight: &mut Cache<RequestId, ()>) {
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
                Duration::from_secs(10),
                execute_trade(
                    &trade,
                    config.router,
                    config.permit2_relayer_address,
                    self.own_address,
                    &self.signer,
                ),
            )
            .await
            {
                Ok(Ok(_)) => {
                    tracing::info!(
                        amount = ?trade.swap_amount,
                        src_chain_id = ?trade.src_chain_id,
                        dest_chain_id = ?trade.dest_chain_id,
                        request_id = %trade.request_id,
                        "successfully traded"
                    );
                }
                Ok(Err(e)) => {
                    tracing::error!(
                        amount = ?trade.swap_amount,
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
    router: &RouterInstance<impl Provider>,
    permit2_relayer_address: Address,
    own_addr: Address,
    signer: &S,
) -> anyhow::Result<TxHash>
where
    S: Signer,
{
    let Permit2RelayTokensDetails {
        message_hash,
        nonce: permit_nonce,
        deadline: permit_deadline,
    } = permit2_relay_tokens_details(trade, permit2_relayer_address)?;
    let permit2_signed_allowance = signer.sign_hash(&message_hash).await?;

    let tx = router
        .relayTokensPermit2(
            own_addr,
            trade.request_id,
            trade.sender_addr,
            trade.recipient_addr,
            trade.token_in_addr,
            trade.token_out_addr,
            trade.swap_amount,
            trade.src_chain_id,
            trade.nonce,
            permit_nonce,
            permit_deadline,
            permit2_signed_allowance.as_erc2098().into(),
        )
        .send()
        .await
        .map_err(|e| {
            // Try to decode it as a Router error
            if let Some(router_err) = e.as_decoded_interface_error::<RouterErrors>() {
                return anyhow!("router contract error: {router_err:?}");
            }
            // Try to decode it as a permit2 error
            if let Some(permit2_err) = permit2::decode_error(&e) {
                return anyhow!("permit2 error: {permit2_err:?}");
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

    tracing::info!("Successfully sent relayTokensPermit2 tx");

    Ok(receipt.transaction_hash)
}
