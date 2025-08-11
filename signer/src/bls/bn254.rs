use super::*;
use ark_ec::pairing::Pairing;
use ark_std::Zero;
use std::ops::Neg;
use utils::hash_to_curve::CustomPairingHashToCurve;

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

impl BlsSigner for BN254SignatureOnG1Signer {
    type Error = std::convert::Infallible;

    fn sign(&self, m: impl AsRef<[u8]>) -> Result<Self::SignatureGroup, Self::Error> {
        let m = ark_bn254::Bn254::hash_to_g1_custom::<sha3::Keccak256>(m.as_ref(), &self.dst);
        let sig = m * self.sk;
        Ok(sig.into_affine())
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
