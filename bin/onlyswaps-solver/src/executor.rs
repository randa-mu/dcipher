use crate::model::{RequestId, Trade};
use crate::network::Network;
use crate::util::normalise_chain_id;
use alloy::primitives::{Address, TxHash};
use alloy::providers::Provider;
use generated::onlyswaps::erc20_faucet_token::ERC20FaucetToken::ERC20FaucetTokenInstance;
use generated::onlyswaps::router::Router::RouterInstance;
use moka::sync::Cache;
use std::collections::HashMap;

pub(crate) struct TradeExecutor<'a, P> {
    own_address: Address,
    routers: HashMap<u64, &'a RouterInstance<P>>,
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
    pub async fn execute(&self, trades: Vec<Trade>, in_flight: &mut Cache<RequestId, ()>) {
        for trade in trades {
            // first we add the trade to the cache so that we don't retry it in the next block
            // (before it's been finalised, potentially)
            in_flight.insert(trade.request_id, ());

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

            // in theory, we shouldn't need to wait until the next block because txs will be processed in nonce order
            // but for whatever reason this doesn't seem to be the case :(
            let approve: anyhow::Result<TxHash> = async {
                let tx = token
                    .approve(*router.address(), trade.swap_amount)
                    .send()
                    .await?;
                let receipt = tx.watch().await?;
                Ok(receipt)
            }
            .await;
            match approve {
                Ok(_) => {}
                Err(e) => {
                    tracing::error!(e = ?e,"error approving trade");
                }
            }

            // actually send the funds via the router contract
            let relay: anyhow::Result<TxHash> = async {
                let tx = router
                    .relayTokens(
                        self.own_address,
                        trade.request_id.into(),
                        trade.sender_addr,
                        trade.recipient_addr,
                        trade.token_in_addr,
                        trade.token_out_addr,
                        trade.swap_amount,
                        trade.src_chain_id,
                        trade.nonce,
                    )
                    .send()
                    .await?;
                let receipt = tx.watch().await?;
                Ok(receipt)
            }
            .await;
            match relay {
                Ok(_) => tracing::info!(
                    amount = ?trade.swap_amount,
                    chain_id = ?trade.dest_chain_id,
                    "successfully traded",
                ),
                Err(e) => tracing::error!(
                    amount = ?trade.swap_amount,
                    chain_id = ?trade.dest_chain_id,
                    error = ?e,
                    "error trading",
                ),
            }
        }
    }
}
