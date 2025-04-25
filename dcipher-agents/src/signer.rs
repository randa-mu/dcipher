use crate::fulfiller::Identifier;

pub mod threshold_signer;

/// Registry used to request signatures on requests.
pub trait RequestSigningRegistry: Send + Sync + 'static {
    type Request: Identifier;
    type SignedRequest: Identifier;

    /// Attempts to fetch signed requests from the signing registry if they are available.
    /// If not, the implementation should asynchronously fetch the request and prepare it for
    /// future calls.
    fn try_fetch_signed_requests<'lt_self, 'lt_r, 'lt_rr>(
        &'lt_self self,
        inputs: impl IntoIterator<Item = &'lt_r Self::Request> + 'lt_self,
    ) -> impl Iterator<Item = Option<Self::SignedRequest>> + 'lt_self
    where
        'lt_r: 'lt_self;
}
