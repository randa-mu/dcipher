pub mod hbacss0;
pub(crate) mod multi_acss;

use crate::helpers::PartyId;
use crate::rand::AdkgRngInstance;
use ark_ec::CurveGroup;
use dcipher_network::topic::TopicBasedTransport;
use std::future::Future;
use std::sync::Arc;
use tokio::sync::oneshot;
use tokio_util::sync::CancellationToken;

/// Trait used to store the configuration required by a ACSS protocol and create new instances.
/// Note that when executing parallel instances, the transports must have a domain separation,
/// i.e., only send / receive messages for this specific instance.
pub trait AcssConfig<'a, CG: CurveGroup, ID>: Send + Sync + 'static {
    type Output: Clone + Sized + Send + Sync;
    type Error: std::error::Error + Send + Sync + 'static;

    fn new_instance<T>(
        self: &Arc<Self>,
        transport: T,
    ) -> Result<impl Acss<CG, ID, Output = Self::Output, Error = Self::Error> + 'a, Self::Error>
    where
        T: TopicBasedTransport<Identity = ID>,
    {
        self.new_instance_with_prefix("".to_owned(), transport)
    }

    fn new_instance_with_prefix<T>(
        self: &Arc<Self>,
        topic_prefix: String,
        transport: T,
    ) -> Result<impl Acss<CG, ID, Output = Self::Output, Error = Self::Error> + 'a, Self::Error>
    where
        T: TopicBasedTransport<Identity = ID>;
}

pub trait Acss<CG, ID>: Send
where
    CG: CurveGroup,
{
    type Error: std::error::Error + Send + Sync + 'static;
    type Output;

    /// Start the reliable broadcast by sending a proposal for message m.
    fn deal<RNG>(
        self,
        s: &CG::ScalarField,
        cancel: CancellationToken,
        output: oneshot::Sender<Self::Output>,
        rng: &mut RNG,
    ) -> impl Future<Output = Result<(), Self::Error>> + Send
    where
        RNG: AdkgRngInstance;

    /// Listen for a reliable broadcast proposal and interact with other nodes to output a message.
    fn get_share<RNG>(
        self,
        expected_broadcaster: PartyId,
        cancel: CancellationToken,
        output: oneshot::Sender<Self::Output>,
        rng: &mut RNG,
    ) -> impl Future<Output = Result<(), Self::Error>> + Send
    where
        RNG: AdkgRngInstance;
}
