use ark_ec::pairing::Pairing;
use digest::{core_api::BlockSizeUser, DynDigest};

mod bn254;

pub trait CustomPairingHashToCurve: Pairing {
    /// Hashes a message to an element of the G1 group using a custom DST and
    /// hash function.
    fn hash_to_g1_custom<H: DynDigest + BlockSizeUser + Default + Clone>(
        message: &[u8],
        dst: &[u8],
    ) -> Self::G1;

    /// Hashes a message to an element of the G2 group using a custom DST and
    /// hash function.
    fn hash_to_g2_custom<H: DynDigest + BlockSizeUser + Default + Clone>(
        message: &[u8],
        dst: &[u8],
    ) -> Self::G2;
}

/// Provides an implementation of hash to curve on G1 and G2 for elliptic curve pairings.
pub trait PairingHashToCurve: CustomPairingHashToCurve {
    const G1_CIPHERSUITE: &'static str;
    const G2_CIPHERSUITE: &'static str;
    type DefaultInnerHash: DynDigest + BlockSizeUser + Default + Clone;

    /// Hashes a message to an element of the G1 group using the hash function
    /// defined in the trait implementation.
    fn hash_to_g1(message: &[u8], dst: &[u8]) -> Self::G1 {
        Self::hash_to_g1_custom::<Self::DefaultInnerHash>(message, dst)
    }

    /// Hashes a message to an element of the G2 group using the hash function
    /// defined in the trait implementation.
    fn hash_to_g2(message: &[u8], dst: &[u8]) -> Self::G2 {
        Self::hash_to_g2_custom::<Self::DefaultInnerHash>(message, dst)
    }
}
