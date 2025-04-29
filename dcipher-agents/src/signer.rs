//! Various traits and implementations used to sign messages.

use crate::ibe_helper::{PairingIbeCipherSuite, PairingIbeSigner};
use ark_ec::AffineRepr;
use std::convert::Infallible;

pub mod threshold_signer;

/// An asynchronous signer is used to generate a signature asynchronously.
pub trait AsynchronousSigner<M> {
    type Error: std::error::Error + Send + Sync + 'static;

    type Signature;

    /// Obtain a future that resolves into a signature.
    fn async_sign(&self, m: M)
    -> impl Future<Output = Result<Self::Signature, Self::Error>> + Send;
}

/// A BLS verifier defines the groups used by an instantiation of a BLS signature scheme and a
/// verification function.
pub trait BlsVerifier {
    type SignatureGroup: AffineRepr;
    type PublicKeyGroup: AffineRepr;

    /// Outputs true if the signature is valid under the specified message and public key.
    fn verify(
        &self,
        m: impl AsRef<[u8]>,
        signature: Self::SignatureGroup,
        public_key: Self::PublicKeyGroup,
    ) -> bool;
}

/// A BLS signer extends the [`BlsVerifier`] trait by providing a signature function.
pub trait BlsSigner: BlsVerifier {
    type Error: std::error::Error + Send + Sync + 'static;

    /// Sign a message using the signer's private key.
    fn sign(&self, m: impl AsRef<[u8]>) -> Result<Self::SignatureGroup, Self::Error>;
}

/// Blanket implementation of a BLS verifier for all [`PairingIbeCipherSuite`].
impl<CS> BlsVerifier for CS
where
    CS: PairingIbeCipherSuite,
{
    type SignatureGroup = CS::IdentityGroup;
    type PublicKeyGroup = CS::PublicKeyGroup;

    fn verify(
        &self,
        m: impl AsRef<[u8]>,
        signature: Self::SignatureGroup,
        public_key: Self::PublicKeyGroup,
    ) -> bool {
        self.verify_decryption_key(m.as_ref(), signature, public_key)
    }
}

/// Blanket implementation of a BLS verifier for all [`PairingIbeSigner`].
impl<CS> BlsSigner for CS
where
    CS: PairingIbeSigner,
{
    type Error = Infallible;

    fn sign(&self, m: impl AsRef<[u8]>) -> Result<Self::SignatureGroup, Self::Error> {
        let identity = self.h1(m.as_ref());
        Ok(PairingIbeSigner::decryption_key(self, identity))
    }
}
