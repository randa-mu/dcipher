//! Various traits and implementations used to sign messages.

/// bls requires at least a curve, and a hash function
#[cfg(all(
    feature = "bls",
    any(feature = "bn254", feature = "bls12-381"),
    any(feature = "sha2", feature = "sha3")
))]
pub mod bls;
#[cfg(feature = "dsigner")]
pub mod dsigner;

/// An asynchronous signer is used to generate a signature asynchronously.
pub trait AsynchronousSigner<M> {
    type Error: std::error::Error + Send + Sync + 'static;

    type Signature;

    /// Obtain a future that resolves into a signature.
    fn async_sign(&self, m: M)
    -> impl Future<Output = Result<Self::Signature, Self::Error>> + Send;
}
