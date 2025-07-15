//! Implementation of the transport traits for libp2p.

use crate::transports::SendMessage;
use crate::{ReceivedMessage, Recipient, Transport, TransportSender};
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};
use tokio_stream::wrappers::UnboundedReceiverStream;

pub struct Libp2pTransport {
    receive_incoming: Option<UnboundedReceiver<ReceivedMessage<u16>>>, // multi-producer, single-consumer
    send_outgoing: Libp2pSender,
}

impl Libp2pTransport {
    pub(super) fn new(
        receive_incoming: UnboundedReceiver<ReceivedMessage<u16>>,
        send_outgoing: UnboundedSender<SendMessage<u16>>,
    ) -> Self {
        Self {
            receive_incoming: Some(receive_incoming),
            send_outgoing: Libp2pSender(send_outgoing),
        }
    }
}

#[derive(Clone)]
pub struct Libp2pSender(UnboundedSender<SendMessage<u16>>);

impl Transport for Libp2pTransport {
    type Identity = u16;
    type ReceiveMessageStream = UnboundedReceiverStream<ReceivedMessage<Self::Identity>>;
    type Sender = Libp2pSender;

    fn sender(&mut self) -> Option<Self::Sender> {
        Some(self.send_outgoing.clone())
    }

    fn receiver_stream(&mut self) -> Option<Self::ReceiveMessageStream> {
        self.receive_incoming
            .take()
            .map(UnboundedReceiverStream::new)
    }
}

#[derive(thiserror::Error, Debug)]
pub enum Libp2pSenderError {
    #[error("channel closed, cannot send new messages")]
    ChannelClosed,
}

impl TransportSender for Libp2pSender {
    type Identity = u16;
    type Error = Libp2pSenderError;

    async fn send(&self, msg: Vec<u8>, to: Recipient<Self::Identity>) -> Result<(), Self::Error> {
        self.0
            .send(SendMessage { msg, to })
            .map_err(|_| Libp2pSenderError::ChannelClosed)
    }
}
