//! NIZK proofs.
use ark_ec::CurveGroup;
use ark_ff::field_hashers::{DefaultFieldHasher, HashToField};
use ark_std::UniformRand;
use itertools::Itertools;
use rand::{CryptoRng, RngCore};
use serde::{Deserialize, Serialize};
use sha3::digest::{DynDigest, core_api::BlockSizeUser};
use std::marker::PhantomData;
use thiserror::Error;
use utils::serialize::fq::{FqDeserialize, FqSerialize};
use utils::serialize::point::PointSerializeCompressed;

#[derive(Error, Debug)]
#[error("opaque nizk error")]
pub struct NizkError;

#[derive(Serialize, Deserialize, Clone)]
#[serde(bound(
    serialize = "CG::ScalarField: FqSerialize",
    deserialize = "CG::ScalarField: FqDeserialize"
))]
pub struct NIZKDleqProof<CG: CurveGroup, H> {
    #[serde(with = "utils::serialize::fq::base64")]
    pub c: CG::ScalarField,

    #[serde(with = "utils::serialize::fq::base64")]
    pub z: CG::ScalarField,

    _h: PhantomData<fn(H) -> H>,
}

impl<CG, H> NIZKDleqProof<CG, H>
where
    H: Default + DynDigest + BlockSizeUser + Clone,
    CG: CurveGroup + PointSerializeCompressed,
{
    /// Create a new NIZK proof proving the knowledge of s s.t. G_s = [s] G and P_s = [s] P
    /// based on Chaum-Pedersen protocol / https://toc.cryptobook.us/book.pdf, 20.3.6
    pub fn prove<R: CryptoRng + RngCore>(
        s: &CG::ScalarField,
        g: &CG,
        p: &CG,
        g_s: &CG,
        p_s: &CG,
        dst: &[u8],
        rng: &mut R,
    ) -> Result<Self, NizkError> {
        let r = CG::ScalarField::rand(rng); // r \gets Z_q
        let g_r = *g * r; // G_r := [r] G
        let p_r = *p * r; // P_r := [r] P

        // c \gets H((G, P, G_s, P_s), (G_r, P_r))
        let c = Self::get_challenge(g, p, g_s, p_s, &g_r, &p_r, dst)?;
        // z := r + cs
        let z = r + c * s;

        Ok(Self {
            c,
            z,
            _h: PhantomData,
        })
    }

    /// Verify a NIZK proof that proves the knowledge of s s.t. G_s = [s] G and P_s = [s] P
    /// based on Chaum-Pedersen protocol / https://toc.cryptobook.us/book.pdf, 20.3.6
    pub fn verify(&self, g: &CG, p: &CG, g_s: &CG, p_s: &CG, dst: &[u8]) -> Result<(), NizkError> {
        // G_r = [r] G = [z] G - [c] G_s
        let g_r = *g * self.z - *g_s * self.c;
        // P_r = [r] P = [z] P - [c] P_s
        let p_r = *p * self.z - *p_s * self.c;

        // c == H((G, P, G_s, P_s), (G_r, P_r))
        if self.c == Self::get_challenge(g, p, g_s, p_s, &g_r, &p_r, dst)? {
            Ok(())
        } else {
            Err(NizkError)
        }
    }

    /// Compute challenge for NIZK proof.
    fn get_challenge(
        g: &CG,
        p: &CG,
        g_s: &CG,
        p_s: &CG,
        g_r: &CG,
        p_r: &CG,
        dst: &[u8],
    ) -> Result<CG::ScalarField, NizkError> {
        // Fixed length encoding as g || p || g_s || p_s || g_r || p_r
        // where each point is encoded with the same length.
        let input = [g, p, g_s, p_s, g_r, p_r]
            .iter()
            .map(|p| p.ser_compressed())
            .flatten_ok()
            .collect::<Result<Vec<u8>, _>>()
            .map_err(|_| NizkError)?;
        let hasher: DefaultFieldHasher<H> = HashToField::<CG::ScalarField>::new(dst);

        // c = H(g, p, g_s, p_s, g_r, p_r)
        Ok(hasher.hash_to_field(&input, 1)[0])
    }
}
