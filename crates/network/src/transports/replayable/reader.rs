//! A replayable implementation of [`Transport`].

use crate::{MessageType, PartyIdentifier, ReceivedMessage, Recipient, Transport, TransportSender};
use futures_util::StreamExt;
use futures_util::stream::BoxStream;
use itertools::Itertools;
use serde::{Deserialize, Serialize};
use std::convert::Infallible;
use std::marker::PhantomData;

/// An entry used to store a message that was sent.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct InMemoryEntry<ID: PartyIdentifier, M> {
    pub timestamp: i64,
    pub sender: ID,
    pub message_type: MessageType,
    pub msg: M,
}

/// A [`Transport`] that can be replayed.
pub struct InMemoryReaderTransport<ID: PartyIdentifier, M> {
    entries: Vec<InMemoryEntry<ID, M>>,
}

impl<ID: PartyIdentifier, M> InMemoryReaderTransport<ID, M> {
    pub fn new(entries: Vec<InMemoryEntry<ID, M>>) -> Self {
        Self { entries }
    }
}

#[cfg(feature = "writer")]
mod writer_impl {
    use super::*;
    use crate::transports::replayable::writer;

    impl<ID: PartyIdentifier, M> InMemoryReaderTransport<ID, M> {
        /// Create an [`InMemoryReaderTransport`] from a collection of broadcasts and direct messages.
        pub fn from_entries(
            broadcasts: impl IntoIterator<Item = (ID, Vec<writer::InMemoryEntry<ID, M>>)>,
            directs: impl IntoIterator<Item = (ID, Vec<writer::InMemoryEntry<ID, M>>)>,
        ) -> Self {
            let map_writer_entry_to_reader =
                move |sender, message_type, entry: writer::InMemoryEntry<ID, M>| InMemoryEntry {
                    timestamp: entry.timestamp,
                    sender,
                    message_type,
                    msg: entry.msg,
                };

            // Flatten each entry, and convert to the expected format
            // O(n) *
            let broadcasts = broadcasts.into_iter().flat_map(|(sender, entries)| {
                // O(N)
                entries.into_iter().map(move |entry| {
                    map_writer_entry_to_reader(sender, MessageType::Broadcast, entry)
                })
            });
            // O(n) *
            let directs = directs.into_iter().flat_map(|(sender, entries)| {
                // O(N)
                entries.into_iter().map(move |entry| {
                    map_writer_entry_to_reader(sender, MessageType::Broadcast, entry)
                })
            });

            // Sort by timestamp
            let sorted = broadcasts
                .into_iter()
                .chain(directs)
                .sorted_by(|a, b| a.timestamp.cmp(&b.timestamp));

            Self {
                entries: sorted.collect(),
            }
        }
    }
}

#[derive(Clone)]
pub struct NopSender<ID>(PhantomData<fn(ID) -> ID>);

impl<ID> Default for NopSender<ID> {
    fn default() -> Self {
        Self(PhantomData)
    }
}

impl<ID> Transport for InMemoryReaderTransport<ID, Vec<u8>>
where
    ID: PartyIdentifier,
{
    type Error = Infallible;
    type Identity = ID;
    type ReceiveMessageStream =
        BoxStream<'static, Result<ReceivedMessage<Self::Identity, Vec<u8>>, Self::Error>>;
    type Sender = NopSender<ID>;

    fn sender(&mut self) -> Option<Self::Sender> {
        Some(NopSender::default())
    }

    fn receiver_stream(&mut self) -> Option<Self::ReceiveMessageStream> {
        let sorted_entries = self
            .entries
            .drain(..)
            .sorted_by(|a, b| a.timestamp.cmp(&b.timestamp));

        let stream = futures_util::stream::iter(sorted_entries)
            .map(|entry| {
                Ok(ReceivedMessage::new(
                    entry.sender,
                    entry.msg,
                    entry.message_type,
                ))
            })
            .chain(futures_util::stream::pending());

        Some(stream.boxed())
    }
}

impl<ID> TransportSender for NopSender<ID>
where
    ID: PartyIdentifier,
{
    type Identity = ID;
    type Error = Infallible;

    async fn send(&self, _msg: Vec<u8>, _to: Recipient<Self::Identity>) -> Result<(), Self::Error> {
        Ok(())
    }
}
