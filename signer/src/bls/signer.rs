//! Implementation of a [`BlsVerifier`] and [`BlsSigner`] for any pairing-friendly curve.

use super::*;
use ark_ec::pairing::Pairing;
use ark_ec::short_weierstrass::{Affine, SWCurveConfig};
use ark_ec::{AffineRepr, PrimeGroup};
use ark_std::Zero;
use digest::FixedOutputReset;
use digest::core_api::BlockSizeUser;
use std::ops::Neg;

/// Concrete implementation of a [`BlsSigner`] on any pairing-friendly curve w/ signatures on G1 or G2.
#[derive(Clone)]
pub struct BlsPairingSigner<E: Pairing> {
    sk: E::ScalarField,
}

impl<E: Pairing> BlsPairingSigner<E> {
    pub fn new(sk: E::ScalarField) -> Self {
        Self { sk }
    }
}

#[cfg(feature = "bn254")]
impl BlsPairingSigner<ark_bn254::Bn254> {
    pub fn new_bn254(sk: ark_bn254::Fr) -> Self {
        Self::new(sk)
    }
}

#[cfg(feature = "bls12-381")]
impl BlsPairingSigner<ark_bls12_381::Bls12_381> {
    pub fn new_bls12_381(sk: ark_bls12_381::Fr) -> Self {
        Self::new(sk)
    }
}

impl<E> BlsVerifier for BlsPairingSigner<E>
where
    E: Pairing,
    E::G1: NamedCurveGroup + CustomHashToCurve,
    E::G1Affine: Into<Affine<<E::G1Affine as AffineRepr>::Config>> + Into<E::G1>,
    <E::G1Affine as AffineRepr>::Config: SWCurveConfig,
    E::G2: NamedCurveGroup + CustomHashToCurve,
    E::G2Affine: Into<Affine<<E::G2Affine as AffineRepr>::Config>> + Into<E::G2>,
    <E::G2Affine as AffineRepr>::Config: SWCurveConfig,
{
    type E = E;

    fn verify_g1<H: FixedOutputReset + BlockSizeUser + Default + Clone>(
        &self,
        m: impl AsRef<[u8]>,
        dst: impl AsRef<[u8]>,
        signature: <Self::E as Pairing>::G1Affine,
        public_key: <Self::E as Pairing>::G2Affine,
    ) -> bool {
        let affine_signature: Affine<_> = signature.into();
        if !affine_signature.is_on_curve()
            || !affine_signature.is_in_correct_subgroup_assuming_on_curve()
            || affine_signature.is_zero()
        {
            return false;
        }

        let m = <Self::E as Pairing>::G1::hash_to_curve_custom::<H>(m.as_ref(), dst.as_ref());
        <Self::E as Pairing>::multi_pairing(
            [m.neg(), signature.into()],
            [public_key, <Self::E as Pairing>::G2::generator().into()],
        )
        .is_zero()
    }

    fn verify_g2<H: FixedOutputReset + BlockSizeUser + Default + Clone>(
        &self,
        m: impl AsRef<[u8]>,
        dst: impl AsRef<[u8]>,
        signature: <Self::E as Pairing>::G2Affine,
        public_key: <Self::E as Pairing>::G1Affine,
    ) -> bool {
        let affine_signature: Affine<_> = signature.into();
        if !affine_signature.is_on_curve()
            || !affine_signature.is_in_correct_subgroup_assuming_on_curve()
            || affine_signature.is_zero()
        {
            return false;
        }

        let m = <Self::E as Pairing>::G2::hash_to_curve_custom::<H>(m.as_ref(), dst.as_ref());
        <Self::E as Pairing>::multi_pairing(
            [
                public_key.into_group().neg(),
                <Self::E as Pairing>::G1::generator(),
            ],
            [m, signature.into()],
        )
        .is_zero()
    }
}

impl<E> BlsSigner for BlsPairingSigner<E>
where
    E: Pairing,
    E::G1: CustomHashToCurve + NamedCurveGroup,
    E::G2: CustomHashToCurve + NamedCurveGroup,
    BlsPairingSigner<E>: BlsVerifier<E = E>,
{
    type Error = std::convert::Infallible;

    fn sign_g1<H: FixedOutputReset + BlockSizeUser + Default + Clone>(
        &self,
        m: impl AsRef<[u8]>,
        dst: impl AsRef<[u8]>,
    ) -> Result<<Self::E as Pairing>::G1Affine, Self::Error> {
        let m = <Self::E as Pairing>::G1::hash_to_curve_custom::<H>(m.as_ref(), dst.as_ref());
        let sig = m * self.sk;
        Ok(sig.into_affine())
    }

    fn sign_g2<H: FixedOutputReset + BlockSizeUser + Default + Clone>(
        &self,
        m: impl AsRef<[u8]>,
        dst: impl AsRef<[u8]>,
    ) -> Result<<Self::E as Pairing>::G2Affine, Self::Error> {
        let m = <Self::E as Pairing>::G2::hash_to_curve_custom::<H>(m.as_ref(), dst.as_ref());
        let sig = m * self.sk;
        Ok(sig.into_affine())
    }

    fn g1_public_key(&self) -> <Self::E as Pairing>::G1Affine {
        let pk = <Self::E as Pairing>::G1::generator() * self.sk;
        pk.into_affine()
    }

    fn g2_public_key(&self) -> <Self::E as Pairing>::G2Affine {
        let pk = <Self::E as Pairing>::G2::generator() * self.sk;
        pk.into_affine()
    }
}

#[cfg(test)]
mod tests {
    #[cfg(feature = "bn254")]
    mod bn254 {
        use crate::bls::{BlsPairingSigner, BlsSigner, BlsVerifier};
        use ark_bn254::Fr;
        use ark_ec::{AffineRepr, CurveGroup};
        use ark_ff::MontFp;

        #[test]
        fn test_g1_g2_consistency() {
            let sk: Fr = MontFp!(
                "7685086713915354683875500702831995067084988389812060097318430034144315778947"
            );
            let pk_g1 = ark_bn254::G1Affine::generator() * sk;
            let pk_g2 = ark_bn254::G2Affine::generator() * sk;
            let signer = BlsPairingSigner::new_bn254(sk);

            let m = b"test_g1_g2_consistency";
            let dst_g1 = b"TEST-BN254G1_XMD:KECCAK-256_SVDW_RO_";
            let dst_g2 = b"TEST-BN254G2_XMD:KECCAK-256_SVDW_RO_";
            let sig_g1 = signer.sign_g1::<sha3::Keccak256>(m, dst_g1).unwrap();
            let sig_g2 = signer.sign_g2::<sha3::Keccak256>(m, dst_g2).unwrap();

            assert!(signer.verify_g1::<sha3::Keccak256>(m, dst_g1, sig_g1, pk_g2.into_affine()));
            assert!(signer.verify_g2::<sha3::Keccak256>(m, dst_g2, sig_g2, pk_g1.into_affine()));
        }
    }
}
