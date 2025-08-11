//! Various traits and implementations used to sign messages.

#[cfg(feature = "bls")]
pub mod bls;

/// An asynchronous signer is used to generate a signature asynchronously.
pub trait AsynchronousSigner<M> {
    type Error: std::error::Error + Send + Sync + 'static;

    type Signature;

    /// Obtain a future that resolves into a signature.
    fn async_sign(&self, m: M)
    -> impl Future<Output = Result<Self::Signature, Self::Error>> + Send;
}
