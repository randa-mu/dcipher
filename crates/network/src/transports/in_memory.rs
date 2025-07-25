//! In memory transport that is primarily designed for use in tests.

use crate::{PartyIdentifier, ReceivedMessage, Recipient, Transport, TransportSender};
use futures_util::stream::BoxStream;
use futures_util::StreamExt;
use std::collections::VecDeque;
use tokio::sync::broadcast;
use tokio_stream::wrappers::errors::BroadcastStreamRecvError;
use tokio_stream::wrappers::BroadcastStream;

#[derive(thiserror::Error, Debug)]
#[error("memory transport error")]
pub enum MemoryTransportError {
    TokioSender,
}

/// Used to obtain a [`Transport`] for each node.
pub struct MemoryNetwork;

#[derive(Clone, Debug)]
struct BusMemoryMessage<ID: PartyIdentifier, M> {
    sender: ID,
    recipient: Recipient<ID>,
    m: M,
}

pub struct BusMemoryTransport<ID: PartyIdentifier, M = Vec<u8>> {
    id: ID,
    tx_channel: broadcast::Sender<BusMemoryMessage<ID, M>>,
    rx_channel: Option<broadcast::Receiver<BusMemoryMessage<ID, M>>>,
}

impl<ID, M> Clone for BusMemoryTransport<ID, M>
where
    ID: PartyIdentifier,
    M: Clone,
{
    fn clone(&self) -> Self {
        Self {
            id: self.id,
            tx_channel: self.tx_channel.clone(),
            rx_channel: self.rx_channel.as_ref().map(|rx| rx.resubscribe()),
        }
    }
}

#[derive(Clone)]
pub struct BusMemorySender<ID: PartyIdentifier, M> {
    id: ID,
    tx_channel: broadcast::Sender<BusMemoryMessage<ID, M>>,
}

impl MemoryNetwork {
    /// Get n individual transports
    pub fn get_transports<ID: PartyIdentifier>(
        ids: impl IntoIterator<Item = ID>,
    ) -> VecDeque<BusMemoryTransport<ID>> {
        let (tx, _) = broadcast::channel(4096);

        ids.into_iter()
            .map(|id| BusMemoryTransport {
                id,
                tx_channel: tx.clone(),
                rx_channel: Some(tx.subscribe()),
            })
            .collect::<VecDeque<_>>()
    }
}

impl<ID: PartyIdentifier> Transport for BusMemoryTransport<ID, Vec<u8>> {
    type Error = BroadcastStreamRecvError;
    type Identity = ID;
    type ReceiveMessageStream =
        BoxStream<'static, Result<ReceivedMessage<ID, Vec<u8>>, Self::Error>>;
    type Sender = BusMemorySender<ID, Vec<u8>>;

    fn sender(&mut self) -> Option<Self::Sender> {
        Some(BusMemorySender {
            id: self.id,
            tx_channel: self.tx_channel.clone(),
        })
    }

    fn receiver_stream(&mut self) -> Option<Self::ReceiveMessageStream> {
        let id = self.id;
        Some(
            BroadcastStream::new(self.rx_channel.take()?)
                .filter_map(move |res| async move {
                    let msg = match res {
                        Ok(msg) => msg,
                        Err(e) => return Some(Err(e)),
                    };

                    let received = ReceivedMessage::new(msg.sender, msg.m, msg.recipient.into());
                    match msg.recipient {
                        Recipient::AllIncludingSelf => Some(Ok(received)), // always yield to stream
                        Recipient::All => (msg.sender != id).then_some(Ok(received)), // ignore if broadcast sent by self
                        Recipient::Single(i) => (i == id).then_some(Ok(received)), // only if sent directly
                    }
                })
                .boxed(),
        )
    }
}

impl<ID: PartyIdentifier> TransportSender for BusMemorySender<ID, Vec<u8>> {
    type Identity = ID;
    type Error = MemoryTransportError;

    async fn send(&self, msg: Vec<u8>, to: Recipient<Self::Identity>) -> Result<(), Self::Error> {
        self.tx_channel
            .send(BusMemoryMessage {
                sender: self.id,
                recipient: to,
                m: msg,
            })
            .map_err(|_| MemoryTransportError::TokioSender)?;
        Ok(())
    }
}
