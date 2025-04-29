//! Various functions used by the blocklock agent.

use alloy::providers::Provider;
use anyhow::anyhow;
use dcipher_agents::agents::blocklock::agent::BlocklockAgent;
use dcipher_agents::decryption_sender::DecryptionRequest;
use dcipher_agents::decryption_sender::contracts::DecryptionSender;
use dcipher_agents::decryption_sender::contracts::DecryptionSender::DecryptionRequested;
use dcipher_agents::fulfiller::{RequestChannel, Ticker};
use futures::Stream;
use futures_util::StreamExt;
use std::future::Future;
use std::sync::Arc;

/// Scheme ID of the blocklock scheme
pub const BLOCKLOCK_SCHEME_ID: &str = "BN254-BLS-BLOCKLOCK";

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
) -> anyhow::Result<()>
where
    F: RequestChannel<Request = DecryptionRequest>,
    P: Provider + Clone + 'static,
{
    let mut events_stream = create_events_stream(decryption_sender_contract.clone()).await?;
    loop {
        match events_stream.next().await {
            Some(ChainEvent::NewBlock(new_block)) => {
                tracing::info!(block_number = new_block, "ChainEvent::NewBlock");

                // Update the blocklock state
                agent.handle_new_block(new_block.into()).await;

                // Tick the decryption fulfiller to try to satisfy requests every new block
                ticker.0.notify_one();
            }
            Some(ChainEvent::DecryptionRequested(request)) => {
                tracing::info!(request_id = %request.requestID, "ChainEvent::DecryptionRequested");
                agent.handle_decryption_requested(request).await;
            }
            None => Err(anyhow!("events stream ended prematurely"))?,
        };
    }
}

async fn create_events_stream<P>(
    decryption_sender_contract: DecryptionSender::DecryptionSenderInstance<P>,
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
    let new_blocks_stream = decryption_sender_contract
        .provider()
        .subscribe_blocks()
        .await?
        .into_stream();

    // Transform each stream into a stream of events
    let decryption_requested_stream =
        decryption_requested_stream.map(|(req, _)| ChainEvent::DecryptionRequested(req));
    let new_blocks_stream = new_blocks_stream.map(|h| ChainEvent::NewBlock(h.number));
    let events_stream = futures::stream::select(decryption_requested_stream, new_blocks_stream);

    Ok(events_stream)
}

impl Ticker for NotifyTicker {
    fn tick(&mut self) -> impl Future<Output = ()> + Send {
        self.0.notified()
    }
}
