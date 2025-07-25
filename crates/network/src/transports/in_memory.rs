//! In memory transport that is primarily designed for use in tests.

use crate::{ReceivedMessage, Recipient, Transport, TransportSender};
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
struct BusMemoryMessage<M> {
    sender: u16,
    recipient: Recipient<u16>,
    m: M,
}

pub struct BusMemoryTransport<M = Vec<u8>> {
    id: u16,
    tx_channel: broadcast::Sender<BusMemoryMessage<M>>,
    rx_channel: Option<broadcast::Receiver<BusMemoryMessage<M>>>, // need mutex for interior mutability + Sync
}

impl<M> Clone for BusMemoryTransport<M>
where
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
pub struct BusMemorySender<M> {
    id: u16,
    tx_channel: broadcast::Sender<BusMemoryMessage<M>>,
}

impl MemoryNetwork {
    /// Get n individual transports
    pub fn get_transports(ids: impl IntoIterator<Item = u16>) -> VecDeque<BusMemoryTransport> {
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

impl Transport for BusMemoryTransport<Vec<u8>> {
    type Error = BroadcastStreamRecvError;
    type Identity = u16;
    type ReceiveMessageStream =
        BoxStream<'static, Result<ReceivedMessage<u16, Vec<u8>>, Self::Error>>;
    type Sender = BusMemorySender<Vec<u8>>;

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

impl TransportSender for BusMemorySender<Vec<u8>> {
    type Identity = u16;
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
