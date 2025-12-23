//! Various functions used by the blocklock agent.

use alloy::providers::Provider;
use anyhow::anyhow;
use dcipher_agents::agents::blocklock::agent::BlocklockAgent;
use dcipher_agents::decryption_sender::DecryptionRequest;
use dcipher_agents::fulfiller::{RequestChannel, Ticker};
use dcipher_agents::utils::block_poller;
use futures::Stream;
use futures_util::StreamExt;
use generated::blocklock::decryption_sender::DecryptionSender;
use generated::blocklock::decryption_sender::DecryptionSender::DecryptionRequested;
use std::future::Future;
use std::sync::Arc;

/// Scheme ID of the blocklock scheme
pub const BN254_BLOCKLOCK_SCHEME_ID: &str = "BN254-BLS-BLOCKLOCK";

/// Ticker using tokio's Notify
#[derive(Clone, Default)]
pub struct NotifyTicker(Arc<tokio::sync::Notify>);

impl NotifyTicker {
    pub fn ticker(&self) -> Arc<tokio::sync::Notify> {
        self.0.clone()
    }
}

/// Chain events used by the blocklock agent.
enum ChainEvent {
    NewBlock(u64),
    DecryptionRequested(DecryptionRequested),
}

/// Run the blocklock agent
pub async fn run_agent<F, P>(
    agent: &mut BlocklockAgent<F, P>,
    ticker: NotifyTicker,
    decryption_sender_contract: DecryptionSender::DecryptionSenderInstance<P>,
    block_poll_interval: std::time::Duration,
    ticker_interval: std::time::Duration,
) -> anyhow::Result<()>
where
    F: RequestChannel<Request = DecryptionRequest>,
    P: Provider + Clone + 'static,
{
    let ticker_fut = {
        let mut interval = tokio::time::interval(ticker_interval);
        async move {
            loop {
                interval.tick().await;
                ticker.0.notify_one();
            }
        }
    };

    let mut events_stream =
        create_events_stream(decryption_sender_contract.clone(), block_poll_interval).await?;
    let events_loop = async move {
        loop {
            match events_stream.next().await {
                Some(ChainEvent::NewBlock(new_block)) => {
                    tracing::debug!(block_number = new_block, "ChainEvent::NewBlock");

                    // Update the blocklock state
                    agent.handle_new_block(new_block.into()).await;
                }
                Some(ChainEvent::DecryptionRequested(request)) => {
                    tracing::info!(request_id = %request.requestId, "ChainEvent::DecryptionRequested");
                    agent.handle_decryption_requested(request).await;
                }
                None => Err(anyhow!("events stream ended prematurely"))?,
            };
        }
    };

    tokio::select! {
        out = events_loop => out,
        _ = ticker_fut => unreachable!("ticker fut is always pending"),
    }
}

async fn create_events_stream<P>(
    decryption_sender_contract: DecryptionSender::DecryptionSenderInstance<P>,
    block_poll_interval: std::time::Duration,
) -> anyhow::Result<impl Stream<Item = ChainEvent>>
where
    P: Provider + Clone + 'static,
{
    // Create a stream for DecryptionRequested and new blocks
    let decryption_requested_stream = decryption_sender_contract
        .DecryptionRequested_filter()
        .subscribe()
        .await?
        .into_stream()
        .flat_map(futures::stream::iter);
    let new_blocks_stream = block_poller(
        decryption_sender_contract.provider().to_owned(),
        block_poll_interval,
    )
    .await;
    // Transform each stream into a stream of events
    let decryption_requested_stream =
        decryption_requested_stream.map(|(req, _)| ChainEvent::DecryptionRequested(req));
    let new_blocks_stream = new_blocks_stream.map(ChainEvent::NewBlock);
    let events_stream = futures::stream::select(decryption_requested_stream, new_blocks_stream);

    Ok(events_stream)
}

impl Ticker for NotifyTicker {
    fn tick(&mut self) -> impl Future<Output = ()> + Send {
        self.0.notified()
    }
}
