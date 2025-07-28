use ark_ec::{pairing::Pairing, CurveGroup};
use digest::{core_api::BlockSizeUser, DynDigest};

#[cfg(any(feature = "bn254", feature = "bls12-381"))]
mod bn254_bls12_381;

/// Custom trait to provide hash to curve implementations for ark's CurveGroups
pub trait CustomHashToCurve: CurveGroup {
    /// Hashes a message to an element of the curve group using a custom DST and
    /// hash function.     
    fn hash_to_curve_custom<H: DynDigest + BlockSizeUser + Default + Clone>(
        message: &[u8],
        dst: &[u8],
    ) -> Self;
}

pub trait HashToCurve: CustomHashToCurve {
    const CIPHERSUITE: &'static str;
    type DefaultInnerHash: DynDigest + BlockSizeUser + Default + Clone;

    /// Hashes a message to an element of the curve group using a custom DST and
    /// hash function.     
    fn hash_to_curve(message: &[u8], dst: &[u8]) -> Self {
        Self::hash_to_curve_custom::<Self::DefaultInnerHash>(message, dst)
    }
}

pub trait CustomPairingHashToCurve: Pairing
where
    Self::G1: CustomHashToCurve,
    Self::G2: CustomHashToCurve,
{
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
pub trait PairingHashToCurve: CustomPairingHashToCurve
where
    Self::G1: HashToCurve,
    Self::G2: HashToCurve,
{
    const G1_CIPHERSUITE: &'static str = <Self::G1 as HashToCurve>::CIPHERSUITE;
    const G2_CIPHERSUITE: &'static str = <Self::G2 as HashToCurve>::CIPHERSUITE;

    /// Hashes a message to an element of the G1 group using the hash function
    /// defined in the trait implementation.
    fn hash_to_g1(message: &[u8], dst: &[u8]) -> Self::G1 {
        Self::hash_to_g1_custom::<<Self::G1 as HashToCurve>::DefaultInnerHash>(message, dst)
    }

    /// Hashes a message to an element of the G2 group using the hash function
    /// defined in the trait implementation.
    fn hash_to_g2(message: &[u8], dst: &[u8]) -> Self::G2 {
        Self::hash_to_g2_custom::<<Self::G2 as HashToCurve>::DefaultInnerHash>(message, dst)
    }
}
