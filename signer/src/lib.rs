//! Various traits and implementations used to sign messages.

use ark_ec::pairing::Pairing;
use ark_ec::{AffineRepr, CurveGroup};
use ark_ff::Zero;
use pairing_utils::hash_to_curve::CustomPairingHashToCurve;
use std::convert::Infallible;
use std::ops::Neg;

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

/// Concrete implementation of a [`BlsSigner`] on the BN254 curve w/ signatures on G1.
#[derive(Clone)]
pub struct BN254SignatureOnG1Signer {
    sk: ark_bn254::Fr,
    dst: Vec<u8>,
}

impl BN254SignatureOnG1Signer {
    pub fn new(sk: ark_bn254::Fr, dst: Vec<u8>) -> Self {
        Self { sk, dst }
    }
}

impl BlsVerifier for BN254SignatureOnG1Signer {
    type SignatureGroup = ark_bn254::G1Affine;
    type PublicKeyGroup = ark_bn254::G2Affine;

    fn verify(
        &self,
        m: impl AsRef<[u8]>,
        signature: Self::SignatureGroup,
        public_key: Self::PublicKeyGroup,
    ) -> bool {
        if !signature.is_on_curve()
            || !signature.is_in_correct_subgroup_assuming_on_curve()
            || signature.is_zero()
        {
            return false;
        }

        let m = ark_bn254::Bn254::hash_to_g1_custom::<sha3::Keccak256>(m.as_ref(), &self.dst);
        ark_bn254::Bn254::multi_pairing(
            [m.neg(), signature.into()],
            [public_key, Self::PublicKeyGroup::generator()],
        )
        .is_zero()
    }
}

impl BlsSigner for BN254SignatureOnG1Signer {
    type Error = Infallible;

    fn sign(&self, m: impl AsRef<[u8]>) -> Result<Self::SignatureGroup, Self::Error> {
        let m = ark_bn254::Bn254::hash_to_g1_custom::<sha3::Keccak256>(m.as_ref(), &self.dst);
        let sig = m * self.sk;
        Ok(sig.into_affine())
    }
}
