//! Module for a proof of knowledge using Schnorr protocol.

use ark_ec::CurveGroup;
use ark_ff::field_hashers::{DefaultFieldHasher, HashToField};
use ark_std::UniformRand;
use digest::FixedOutputReset;
use itertools::Itertools;
use rand::{CryptoRng, RngCore};
use serde::{Deserialize, Serialize};
use sha3::digest::{DynDigest, core_api::BlockSizeUser};
use std::marker::PhantomData;
use thiserror::Error;
use utils::serialize::fq::{FqDeserialize, FqSerialize};
use utils::serialize::point::PointSerializeCompressed;

#[derive(Error, Debug)]
#[error("opaque pok error")]
pub struct PokError;

#[derive(Serialize, Deserialize, Clone)]
#[serde(bound(
    serialize = "CG::ScalarField: FqSerialize",
    deserialize = "CG::ScalarField: FqDeserialize"
))]
pub struct PokProof<CG: CurveGroup, H> {
    #[serde(with = "utils::serialize::fq::base64_or_bytes")]
    pub c: CG::ScalarField,

    #[serde(with = "utils::serialize::fq::base64_or_bytes")]
    pub z: CG::ScalarField,

    _h: PhantomData<fn(H) -> H>,
}

impl<CG, H> PokProof<CG, H>
where
    H: Default + DynDigest + FixedOutputReset + BlockSizeUser + Clone,
    CG: CurveGroup + PointSerializeCompressed,
{
    /// Create a new proof proving the knowledge of s s.t. G_s = [s] G
    pub fn prove<R: CryptoRng + RngCore>(
        s: &CG::ScalarField,
        g: &CG,
        g_s: &CG,
        dst: &[u8],
        rng: &mut R,
    ) -> Result<Self, PokError> {
        let r = CG::ScalarField::rand(rng); // r \gets Z_q
        let g_r = *g * r; // G_r := [r] G

        // c \gets H((G, G_s), (G_r))
        let c = Self::get_challenge(g, g_s, &g_r, dst)?;
        // z := r + cs
        let z = r + c * s;

        Ok(Self {
            c,
            z,
            _h: PhantomData,
        })
    }

    /// Verify a proof that proves the knowledge of s s.t. G_s = [s] G
    pub fn verify(&self, g: &CG, g_s: &CG, dst: &[u8]) -> Result<(), PokError> {
        // G_r = [r] G = [z] G - [c] G_s
        let g_r = *g * self.z - *g_s * self.c;

        // c == H((G, G_s), (G_r)
        if self.c == Self::get_challenge(g, g_s, &g_r, dst)? {
            Ok(())
        } else {
            Err(PokError)
        }
    }

    /// Compute challenge for proof.
    // Fixed length encoding as g || g_s || g_r
    fn get_challenge(g: &CG, g_s: &CG, g_r: &CG, dst: &[u8]) -> Result<CG::ScalarField, PokError> {
        // where each point is encoded with the same length.
        let input = [g, g_s, g_r]
            .iter()
            .map(|p| p.ser_compressed())
            .flatten_ok()
            .collect::<Result<Vec<u8>, _>>()
            .map_err(|_| PokError)?;
        let hasher: DefaultFieldHasher<H> = HashToField::<CG::ScalarField>::new(dst);

        // c = H(g, g_s, g_r)
        Ok(hasher.hash_to_field::<1>(&input)[0])
    }
}

#[cfg(test)]
mod tests {
    #[cfg(feature = "bn254")]
    mod bn254 {
        use super::super::*;
        use ark_ec::PrimeGroup;
        use rand::thread_rng;

        #[test]
        fn test_pok_consistency() {
            let mut rng = thread_rng();
            let dst = b"test_domain_separation";

            // Generate random secret and base point
            let s = ark_bn254::Fr::rand(&mut rng);
            let g = ark_bn254::G1Projective::generator();
            let g_s = g * s;

            // Create proof
            let proof = PokProof::<_, sha3::Sha3_256>::prove(&s, &g, &g_s, dst, &mut rng)
                .expect("Proof generation should succeed");

            // Verify proof
            proof
                .verify(&g, &g_s, dst)
                .expect("Valid proof should verify successfully");
        }
    }
}
