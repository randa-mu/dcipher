//! Various functions used by the randomness agent.

use alloy::providers::Provider;
use anyhow::anyhow;
use dcipher_agents::agents::randomness::RandomnessAgent;
use dcipher_agents::fulfiller::{RequestChannel, Ticker};
use dcipher_agents::signature_sender::SignatureRequest;
use dcipher_agents::utils::block_poller;
use futures::Stream;
use futures_util::StreamExt;
use generated::randomness::signature_sender::SignatureSender::{
    SignatureRequested, SignatureSenderInstance,
};
use std::future::Future;
use std::sync::Arc;

/// Scheme ID of the randomness schemes
pub const BN254_RANDOMNESS_SCHEME_ID: &str = "BN254";
pub const BLS12_381_RANDOMNESS_SCHEME_ID: &str = "BLS12381";
pub const BLS12_381_COMPRESSED_RANDOMNESS_SCHEME_ID: &str = "BLS12381Compressed";

/// Ticker using tokio's Notify
#[derive(Clone, Default)]
pub struct NotifyTicker(Arc<tokio::sync::Notify>);

impl NotifyTicker {
    pub fn ticker(&self) -> Arc<tokio::sync::Notify> {
        self.0.clone()
    }
}

/// Chain events used by the randomness agent.
enum ChainEvent {
    NewBlock(u64),
    SignatureRequested(SignatureRequested),
}

/// Run the randomness agent
pub async fn run_agent<F, P>(
    agent: &mut RandomnessAgent<F, P>,
    ticker: NotifyTicker,
    signature_sender_contract: SignatureSenderInstance<P>,
    block_poll_interval: std::time::Duration,
    ticker_interval: std::time::Duration,
) -> anyhow::Result<()>
where
    F: RequestChannel<Request = SignatureRequest>,
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
        create_events_stream(signature_sender_contract.clone(), block_poll_interval).await?;
    let events_loop = async move {
        loop {
            match events_stream.next().await {
                Some(ChainEvent::NewBlock(new_block)) => {
                    tracing::debug!(block_number = new_block, "ChainEvent::NewBlock");
                    agent.handle_new_block(new_block.into()).await;
                }
                Some(ChainEvent::SignatureRequested(request)) => {
                    tracing::info!(request_id = %request.requestID, "ChainEvent::SignatureRequested");
                    agent.handle_signature_requested(request).await;
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
    signature_sender_contract: SignatureSenderInstance<P>,
    block_poll_interval: std::time::Duration,
) -> anyhow::Result<impl Stream<Item = ChainEvent>>
where
    P: Provider + Clone + 'static,
{
    // Create a stream for SignatureRequested events and new blocks
    let signature_requested_stream = signature_sender_contract
        .SignatureRequested_filter()
        .subscribe()
        .await?
        .into_stream()
        .flat_map(futures::stream::iter);
    let new_blocks_stream = block_poller(
        signature_sender_contract.provider().to_owned(),
        block_poll_interval,
    )
    .await;

    // Transform each stream into a stream of events
    let signature_requested_stream =
        signature_requested_stream.map(|(req, _)| ChainEvent::SignatureRequested(req));
    let new_blocks_stream = new_blocks_stream.map(ChainEvent::NewBlock);
    let events_stream = futures::stream::select(signature_requested_stream, new_blocks_stream);

    Ok(events_stream)
}

impl Ticker for NotifyTicker {
    fn tick(&mut self) -> impl Future<Output = ()> + Send {
        self.0.notified()
    }
}
