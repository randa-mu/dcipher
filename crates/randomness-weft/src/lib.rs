//! Various functions used by the randomness agent.

use alloy::providers::Provider;
use anyhow::anyhow;
use randomness_warp::RandomnessAgent;
use enc_core::fulfiller::{RequestChannel, Ticker};
use enc_core::signature_sender::{SignatureRequest, contracts::{SignatureSender, SignatureSender::SignatureRequested}};
use futures::Stream;
use futures_util::StreamExt;
use std::future::Future;
use std::sync::Arc;

/// Scheme ID of the randomness scheme
pub const RANDOMNESS_SCHEME_ID: &str = "BN254";

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
    signature_sender_contract: SignatureSender::SignatureSenderInstance<P>,
) -> anyhow::Result<()>
where
    F: RequestChannel<Request = SignatureRequest>,
    P: Provider + Clone + 'static,
{
    let mut events_stream = create_events_stream(signature_sender_contract.clone()).await?;
    loop {
        match events_stream.next().await {
            Some(ChainEvent::NewBlock(new_block)) => {
                tracing::debug!(block_number = new_block, "ChainEvent::NewBlock");
                agent.handle_new_block(new_block.into()).await;

                // Tick the fulfiller every block
                ticker.0.notify_one();
            }
            Some(ChainEvent::SignatureRequested(request)) => {
                tracing::info!(request_id = %request.requestID, "ChainEvent::SignatureRequested");
                agent.handle_signature_requested(request).await;
            }
            None => Err(anyhow!("events stream ended prematurely"))?,
        };
    }
}

async fn create_events_stream<P>(
    signature_sender_contract: SignatureSender::SignatureSenderInstance<P>,
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
    let new_blocks_stream = signature_sender_contract
        .provider()
        .subscribe_blocks()
        .await?
        .into_stream();

    // Transform each stream into a stream of events
    let signature_requested_stream =
        signature_requested_stream.map(|(req, _)| ChainEvent::SignatureRequested(req));
    let new_blocks_stream = new_blocks_stream.map(|h| ChainEvent::NewBlock(h.number));
    let events_stream = futures::stream::select(signature_requested_stream, new_blocks_stream);

    Ok(events_stream)
}

impl Ticker for NotifyTicker {
    fn tick(&mut self) -> impl Future<Output = ()> + Send {
        self.0.notified()
    }
}
