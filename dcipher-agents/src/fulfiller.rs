//! Module containing various traits to define the behaviour of fulfillers.

mod failure;
pub(crate) mod ticker;

pub use failure::RetryStrategy;

use futures_util::future::BoxFuture;
use std::hash::Hash;

/// Base trait defining a type that can be used to sign request and submit fulfilled requests in a transaction.
pub trait Fulfiller {
    /// Type of request processed by the fulfiller.
    type Request: Identifier + Send + Sync + 'static;

    /// Type of signed request processed by the fulfiller.
    type SignedRequest: Identifier + Send + Sync + 'static;

    /// A [`RequestSigningRegistry`] used to obtain a [`SignedRequest`](Self::SignedRequest)
    /// from a [`Request`](Self::Request).
    type RequestSigningRegistry: RequestSigningRegistry<Request = Self::Request, SignedRequest = Self::SignedRequest>;

    /// A [`TransactionFulfiller`] used to register the [`SignedRequest`](Self::SignedRequest) by
    /// generally sending a transaction to a blockchain.
    type TransactionFulfiller: TransactionFulfiller<SignedRequest = Self::SignedRequest>;

    /// A [`RequestChannel`] that can be used by other components to request the fulfillment of a
    /// [`Request`](Self::Request).
    type RequestChannel: RequestChannel;

    /// Handle to stop the fulfiller.
    type Stop: Stopper;
}

/// A [`Fulfiller`] that attempts to submit transactions every tick.
pub trait TickerBasedFulfiller: Fulfiller {
    /// Executes the [`Fulfiller`] in a background task that attempts to fulfil requests every tick.
    /// Returns a handle that is used to stop the execution of the fulfiller, and a channel that can
    /// be used to send requests to the fulfiller.
    fn run(self, ticker: impl Ticker) -> (Self::Stop, Self::RequestChannel);
}

/// Types implementing [`Identifier`] provide a field that can be used as an identifier.
pub trait Identifier {
    type Id: std::fmt::Display + Ord + Eq + Hash + ToOwned + Send + Sync + 'static;

    /// Returns a reference to a field that can be used as an identifier
    fn id(&self) -> &Self::Id;
}

/// A channel that can be used to register requests with the fulfiller.
pub trait RequestChannel {
    type Request;

    /// Register requests.
    fn register_requests<I>(&mut self, requests: I)
    where
        I: IntoIterator,
        I::Item: Into<Self::Request>;
}

/// Aborts the execution of a task.
pub trait Stopper {
    fn stop(self) -> impl Future<Output = ()> + Send;
}

/// Ticker used to provide fulfillment frequency to the fulfiller.
pub trait Ticker: Send + Sync + 'static {
    fn tick(&mut self) -> impl Future<Output = ()> + Send;
}

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

/// Final stage of the fulfiller responsible to submit signed requests, typically to a blockchain.
pub trait TransactionFulfiller: Send + Sync + 'static {
    type SignedRequest;

    type Error: std::error::Error + Send + Sync + 'static;

    fn fulfil_decryption_requests<'lt_self, 'lt_sr, I>(
        &'lt_self self,
        requests: I,
    ) -> BoxFuture<'lt_self, Vec<Result<(), Self::Error>>>
    where
        I: IntoIterator<Item = &'lt_sr Self::SignedRequest> + Send + 'lt_self,
        I::IntoIter: Send;
}
