use crate::model::{RequestId, Trade};
use alloy::primitives::TxHash;
use anyhow::Context;
use config::timeout::TimeoutConfig;
use moka::future::Cache;
use onlyswaps_client::client::OnlySwapsClient;
use onlyswaps_client::client::solver::OnlySwapsTrade;
use onlyswaps_client::config::token::TokenTag;
use tokio::time::timeout;

pub(crate) struct TradeExecutor {
    client: OnlySwapsClient,
}

impl TradeExecutor {
    pub fn new(client: OnlySwapsClient) -> Self {
        Self { client }
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

            // and finally execute the trade with a timeout
            match timeout(
                timeout_config.request_timeout,
                execute_trade(&trade, &self.client),
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

async fn execute_trade(trade: &Trade, client: &OnlySwapsClient) -> anyhow::Result<TxHash> {
    let src_chain_id = trade
        .src_chain_id
        .try_into()
        .context("failed to cast chain id")?;
    let dest_chain_id = trade
        .dest_chain_id
        .try_into()
        .context("failed to cast chain id")?;
    let source_address = client
        .config()
        .get_address(src_chain_id)
        .context("source address not found")?;

    client
        .approve_spending_and_wait(
            dest_chain_id,
            TokenTag::Other(trade.token_out_addr.to_string().into()),
            trade.amount_out,
            None,
        )
        .await
        .context("failed to approve spending")?;

    let receipt = client
        .relay_tokens(
            trade
                .to_owned()
                .try_into()
                .context("failed to cast trade")?,
            source_address,
        )
        .await
        .context("error submitting swap")?;

    Ok(receipt.transaction_hash)
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
