use crate::model::{RequestId, Trade};
use crate::network::Network;
use crate::profitability::{ErasedProfitabilityEstimator, ProfitabilityEstimator};
use crate::util::normalise_chain_id;
use alloy::primitives::{Address, TxHash};
use alloy::providers::Provider;
use alloy::signers::Signer;
use anyhow::Context;
use config::timeout::TimeoutConfig;
use generated::onlyswaps::i_router::IRouter::IRouterInstance;
use generated::onlyswaps::permit2_relayer::Permit2Relayer::Permit2RelayerInstance;
use moka::future::Cache;
use onlyswaps_client::client::OnlySwapsClient;
use onlyswaps_client::client::solver::OnlySwapsTrade;
use std::collections::HashMap;
use tokio::time::timeout;

pub(crate) struct TradeExecutor<'a, P, S> {
    client: OnlySwapsClient,
    signer: S,
    own_address: Address,
    configs: HashMap<u64, ChainConfig<'a, P>>,
    profitability_estimator: ErasedProfitabilityEstimator,
}

pub(crate) struct ChainConfig<'a, P> {
    router: &'a IRouterInstance<P>,
    permit2_relayer_address: Address,
    permit2_addr: Address,
}

impl<'a, P, S> TradeExecutor<'a, P, S>
where
    P: Provider,
{
    pub async fn new(
        signer: S,
        client: OnlySwapsClient,
        networks: &'a HashMap<u64, Network<P>>,
        profitability_estimator: ErasedProfitabilityEstimator,
    ) -> anyhow::Result<Self> {
        let permit2_addresses: HashMap<_, _> =
            fetch_permit2_addresses(networks.iter()).await?.collect();
        let configs = networks
            .iter()
            .map(|(chain_id, net)| -> anyhow::Result<_> {
                let permit2_addr = *permit2_addresses.get(chain_id).with_context(|| {
                    format!("failed to get permit2 address of chain {chain_id}")
                })?;
                Ok((
                    *chain_id,
                    ChainConfig {
                        router: &net.router,
                        permit2_relayer_address: net.permit2_relayer_address,
                        permit2_addr,
                    },
                ))
            })
            .collect::<anyhow::Result<_>>()?;

        let own_address = networks
            .iter()
            .next()
            .map(|(_, network)| network.own_addr)
            .expect("if we don't have a network by now, something is very wrong");

        Ok(Self {
            client,
            signer,
            configs,
            own_address,
            profitability_estimator,
        })
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
                .configs
                .get(&normalise_chain_id(trade.dest_chain_id))
                .expect("somehow didn't have a router binding for a solved trade");

            // and finally execute the trade with a timeout
            match timeout(
                timeout_config.request_timeout,
                self.execute_trade(trade.clone(), config),
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

    async fn execute_trade<'aa>(
        &self,
        trade: Trade,
        chain_config: &ChainConfig<'aa, P>,
    ) -> anyhow::Result<TxHash> {
        let sendable_tx = self
            .client
            .relay_tokens_permit2(
                &trade.clone().try_into().context("invalid trade")?,
                self.own_address,
                chain_config.permit2_addr,
                chain_config.permit2_relayer_address,
                &self.signer,
            )
            .await
            .context("failed to obtain sendable permit2 tx")?;

        let gas_cost = estimate_gas_cost(chain_config.router.provider())
            .await
            .context("gas cost estimation failed")?;
        if !self
            .profitability_estimator
            .is_profitable(&trade, sendable_tx.gas_estimate(), gas_cost)
            .await
            .context("failed to compute profitability of trade")?
        {
            tracing::warn!("Trade not profitable, refusing fulfillment");
            anyhow::bail!("trade not profitable");
        }

        let receipt = sendable_tx
            .send()
            .await
            .context("failed to send permit2 tx")?;
        Ok(receipt.transaction_hash)
    }
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

impl TryFrom<Trade> for OnlySwapsTrade {
    type Error = anyhow::Error;

    fn try_from(value: Trade) -> Result<Self, Self::Error> {
        Ok(Self {
            token_in_addr: value.token_in_addr,
            token_out_addr: value.token_out_addr,
            src_chain_id: value.src_chain_id.try_into()?,
            dest_chain_id: value.dest_chain_id.try_into()?,
            sender_addr: value.sender_addr,
            recipient_addr: value.recipient_addr,
            amount_out: value.amount_out,
            nonce: value.nonce,
            request_id: value.request_id,
            pre_hooks: value.pre_hooks,
            post_hooks: value.post_hooks,
        })
    }
}

pub async fn fetch_permit2_addresses<'a, P>(
    networks: impl IntoIterator<Item = (&'a u64, &'a Network<P>)>,
) -> anyhow::Result<impl Iterator<Item = (u64, Address)>>
where
    P: Provider + 'a,
{
    let permit2_addresses =
        futures::future::try_join_all(networks.into_iter().map(async |(&id, c)| {
            Permit2RelayerInstance::new(c.permit2_relayer_address, c.router.provider())
                .PERMIT2()
                .call()
                .await
                .map(|addr| (id, addr))
        }))
        .await
        .context("failed to get permit2 addresses")?;

    Ok(permit2_addresses.into_iter())
}
