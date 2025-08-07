//! Implementation of [`Transport`] that forwards sent messages to a [`MessageWriter`].

use crate::transports::replayable::writer::{InMemoryWriter, MessageWriter};
use crate::{Recipient, Transport, TransportSender};

/// [`Transport`] forwarding sent messages to a writer.
pub struct TransportWriter<W, T> {
    writer: W,
    transport: T,
}

#[derive(thiserror::Error, Debug)]
pub enum TransportWriterError {
    #[error("underlying writer error")]
    Writer(Box<dyn std::error::Error + Send + Sync + 'static>),
}

impl<W, T> TransportWriter<W, T> {
    /// Create a new [`TransportWriter`].
    pub fn new(writer: W, transport: T) -> Self {
        Self { writer, transport }
    }

    /// Obtain a reference to the inner [`MessageWriter`].
    pub fn writer(&self) -> &W {
        &self.writer
    }
}

impl<T, M> TransportWriter<InMemoryWriter<T::Identity, M>, T>
where
    T: Transport,
{
    /// Create a new [`TransportWriter`] with an [`InMemoryWriter`].
    pub fn new_in_memory(transport: T) -> Self {
        Self {
            writer: Default::default(),
            transport,
        }
    }
}

impl<W, _Transport> Transport for TransportWriter<W, _Transport>
where
    W: Clone + 'static,
    _Transport: Transport,
    TransportWriterSender<W, _Transport::Sender>: TransportSender<Identity = _Transport::Identity>,
{
    type Error = _Transport::Error;
    type Identity = _Transport::Identity;
    type ReceiveMessageStream = _Transport::ReceiveMessageStream;
    type Sender = TransportWriterSender<W, _Transport::Sender>;

    fn sender(&mut self) -> Option<Self::Sender> {
        self.transport.sender().map(|ts| TransportWriterSender {
            writer: self.writer.clone(),
            transport_sender: ts,
        })
    }

    fn receiver_stream(&mut self) -> Option<Self::ReceiveMessageStream> {
        self.transport.receiver_stream()
    }
}

/// A [`TransportSender`] that forwards messages to a [`MessageWriter`].
#[derive(Clone)]
pub struct TransportWriterSender<W, TS> {
    writer: W,
    transport_sender: TS,
}

impl<W, _TransportSender> TransportSender for TransportWriterSender<W, _TransportSender>
where
    W: MessageWriter<_TransportSender::Identity, Vec<u8>>,
    _TransportSender: TransportSender,
{
    type Identity = _TransportSender::Identity;
    type Error = _TransportSender::Error;

    async fn send(&self, msg: Vec<u8>, to: Recipient<Self::Identity>) -> Result<(), Self::Error> {
        if let Err(e) = self.writer.write(&msg, to).await {
            // trace the error, do not return it as a transport error
            tracing::error!(error = ?e, "Failed to write sent message");
        }

        self.transport_sender.send(msg.clone(), to).await
    }
}
