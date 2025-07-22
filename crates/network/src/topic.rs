//! Topic based transport.

pub mod dispatcher;

use crate::{PartyIdentifier, Transport};

pub trait Topic: AsRef<[u8]> {}

impl<T: AsRef<[u8]>> Topic for T {}

pub trait TopicBasedTransport: Clone {
    type Transport: Transport<Identity = Self::Identity> + Send + 'static;
    type Identity: PartyIdentifier + Send + Sync + 'static;

    /// Get an instance of [`Transport`] used to send and receive messages on a specific topic.
    fn get_transport_for<T>(&self, topic: T) -> Option<Self::Transport>
    where
        T: Topic;
}

/// Blanket implementation for arc'ed topic based transport
impl<TBT: TopicBasedTransport> TopicBasedTransport for std::sync::Arc<TBT> {
    type Transport = TBT::Transport;
    type Identity = TBT::Identity;

    fn get_transport_for<T>(&self, topic: T) -> Option<Self::Transport>
    where
        T: Topic,
    {
        self.as_ref().get_transport_for(topic)
    }
}
