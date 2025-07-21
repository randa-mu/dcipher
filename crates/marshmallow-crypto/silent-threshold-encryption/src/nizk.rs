use crate::encryption::{Ciphertext, CiphertextCCA2Generic};
use ark_ec::pairing::{Pairing, PairingOutput};
use ark_ec::Group;
use ark_ff::field_hashers::{DefaultFieldHasher, HashToField};
use ark_serialize::*;
use ark_std::{UniformRand, Zero};
use rand::Rng;
use sha3::digest::{core_api::BlockSizeUser, DynDigest};
use std::marker::PhantomData;

#[derive(CanonicalSerialize, CanonicalDeserialize, Clone)]
pub struct ShamirNIZKProof<E: Pairing, H> {
    pub c: E::ScalarField,
    pub s: E::ScalarField,
    _h: PhantomData<fn(H) -> H>,
}

#[derive(CanonicalSerialize, CanonicalDeserialize, Clone)]
struct ShamirNIZKProofContent<E: Pairing> {
    pub gamma_g2: E::G2,
    pub sa1: [E::G1; 2],
    pub sa2: [E::G2; 6],
    pub enc_key: PairingOutput<E>,
    pub r_g2: E::G2,
}

impl<E: Pairing, H> ShamirNIZKProof<E, H>
where
    H: Default + DynDigest + BlockSizeUser + Clone,
{
    /// Create a new Shamir NIZK proof using the ciphertext and the secret gamma.
    pub fn new_from_ciphertext<R: Rng>(
        ct: &Ciphertext<E>,
        gamma: &E::ScalarField,
        rng: &mut R,
    ) -> ShamirNIZKProof<E, H> {
        // Generate r \getsr Z_n
        let r = E::ScalarField::rand(rng);
        let r_g2 = E::G2::generator() * r;

        // c = H([\gamma]_2, ct_2, ct_3, [r]_2)
        let c = challenge_from_ciphertext::<E, H>(ct, &r_g2);
        // s = r + c * \gamma
        let s = r + c * gamma;

        ShamirNIZKProof {
            c,
            s,
            _h: PhantomData,
        }
    }

    /// Verify the proof using the provided ciphertext.
    pub fn verify_with_ciphertext(&self, ct: &CiphertextCCA2Generic<E, H>) -> Result<(), ()> {
        // Ensure that gamma_g2 is a valid point
        if ct.gamma_g2.is_zero() {
            Err(())?
        }

        // Verify the proof
        // s = r + c * \gamma
        // [r]_2 = [s]_2 - c[\gamma]_2 = [r + c \gamma - c \gamma]_2
        let r_g2 = E::G2::generator() * ct.proof.s - ct.gamma_g2 * self.c;

        // c = H([\gamma]_2, ct_2, ct_3, [r]_2)
        let c = challenge_from_ciphertext::<E, H>(&ct.clone().into(), &r_g2);

        if c != self.c {
            Err(())
        } else {
            Ok(())
        }
    }
}

fn challenge_from_ciphertext<E: Pairing, H>(ct: &Ciphertext<E>, r_g2: &E::G2) -> E::ScalarField
where
    H: Default + DynDigest + BlockSizeUser + Clone,
{
    // Prepare hasher to hash to the scalar field
    let hash_content = ShamirNIZKProofContent {
        gamma_g2: ct.gamma_g2,
        sa1: ct.sa1,
        sa2: ct.sa2,
        enc_key: ct.enc_key,
        r_g2: *r_g2,
    };
    let mut content = Vec::new();
    hash_content
        .serialize_uncompressed(&mut content)
        .expect("failed to serialize content");

    let hasher: DefaultFieldHasher<H> = HashToField::<E::ScalarField>::new(b"todo: dst");

    // c = H([\gamma]_2, ct_2, ct_3, [r]_2)
    hasher.hash_to_field(&content, 1)[0]
}
