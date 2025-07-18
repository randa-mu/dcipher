//! Topic based transport.

pub mod dispatcher;

use crate::{PartyIdentifier, Transport};

pub trait Topic: AsRef<[u8]> {}

impl<T: AsRef<[u8]>> Topic for T {}

pub trait TopicBasedTransport: Clone {
    type Identity: PartyIdentifier + Send + Sync + 'static;

    /// Get an instance of [`Transport`] used to send and receive messages on a specific topic.
    fn get_transport_for<T>(&self, topic: T) -> Option<impl Transport<Identity = Self::Identity>>
    where
        T: Topic;
}
