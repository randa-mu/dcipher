//! Implementation of the transport traits for libp2p.

use crate::transports::{SendBroadcastMessage, SendDirectMessage, TransportAction};
use crate::{PartyIdentifier, ReceivedMessage, Recipient, Transport, TransportSender};
use futures_util::StreamExt;
use futures_util::stream::BoxStream;
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};
use tokio_stream::wrappers::UnboundedReceiverStream;

/// An implementation of [`Transport`] with libp2p communications.
pub struct Libp2pTransport<I: PartyIdentifier> {
    receive_incoming: Option<UnboundedReceiver<ReceivedMessage<I>>>, // multi-producer, single-consumer
    send_outgoing: Libp2pSender<I>,
}

impl<I: PartyIdentifier> Libp2pTransport<I> {
    pub(super) fn new(
        receive_incoming: UnboundedReceiver<ReceivedMessage<I>>,
        send_outgoing: UnboundedSender<TransportAction<I>>,
    ) -> Self {
        Self {
            receive_incoming: Some(receive_incoming),
            send_outgoing: Libp2pSender(send_outgoing),
        }
    }
}

#[derive(Clone)]
pub struct Libp2pSender<I: PartyIdentifier>(UnboundedSender<TransportAction<I>>);

impl<I> Transport for Libp2pTransport<I>
where
    I: PartyIdentifier + Send + Sync + 'static,
{
    type Error = Libp2pTransportError;
    type Identity = I;
    type ReceiveMessageStream =
        BoxStream<'static, Result<ReceivedMessage<Self::Identity>, Self::Error>>;
    type Sender = Libp2pSender<I>;

    fn sender(&mut self) -> Option<Self::Sender> {
        Some(self.send_outgoing.clone())
    }

    fn receiver_stream(&mut self) -> Option<Self::ReceiveMessageStream> {
        let receiver = self.receive_incoming.take()?;
        Some(UnboundedReceiverStream::new(receiver).map(Ok).boxed())
    }
}

#[derive(thiserror::Error, Debug)]
pub enum Libp2pTransportError {
    #[error("channel closed, cannot send new messages")]
    ChannelClosed,
}

impl<I> TransportSender for Libp2pSender<I>
where
    I: PartyIdentifier + Send + Sync + 'static,
{
    type Identity = I;
    type Error = Libp2pTransportError;

    async fn send(&self, msg: Vec<u8>, to: Recipient<Self::Identity>) -> Result<(), Self::Error> {
        let action = match to {
            Recipient::All => SendBroadcastMessage::new(msg, false).into(),
            Recipient::AllIncludingSelf => SendBroadcastMessage::new(msg, true).into(),
            Recipient::Single(to) => SendDirectMessage::new(to, msg).into(),
        };

        self.0
            .send(action)
            .map_err(|_| Libp2pTransportError::ChannelClosed)
    }
}
