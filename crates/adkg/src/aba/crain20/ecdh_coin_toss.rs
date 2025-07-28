//! Implementation of the Elliptic Curve Diffie-Hellman Based Threshold Coin-Tossing scheme of https://eprint.iacr.org/2000/034.pdf

use crate::helpers::{PartyId, lagrange_points_interpolate_at};
use crate::nizk::NIZKDleqProof;
use ark_ec::CurveGroup;
use digest::{DynDigest, core_api::BlockSizeUser};
use itertools::{Itertools, izip};
use rand::{CryptoRng, RngCore};
use serde::{Deserialize, Serialize};
use sha3::{Digest, Sha3_256};
use thiserror::Error;
use utils::{
    hash_to_curve::HashToCurve,
    serialize::point::{PointDeserializeCompressed, PointSerializeCompressed},
};

// todo: Support multiple DSTs
const ECDH_COIN_TOSS_GENERATOR_DST: &[u8] = b"ECDH_COIN_TOSS_H_BN254G1_XMD:SHA3-256";
const ECDH_COIN_TOSS_NIZK_DST: &[u8] = b"ECDH_COIN_TOSS_H_BN254G1_XMD:SHA3-256";

#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Coin {
    Zero = 0,
    One = 1,
}

#[derive(Error, Debug)]
#[error("opaque ecdh coin toss error")]
pub struct EcdhCoinTossError;

#[derive(Serialize, Deserialize, Clone)]
#[serde(bound(
    serialize = "CG: PointSerializeCompressed, NIZKDleqProof<CG, H>: Serialize",
    deserialize = "CG: PointDeserializeCompressed, NIZKDleqProof<CG, H>: Deserialize<'de>"
))]
pub struct EcdhCoinTossEval<CG: CurveGroup, H> {
    #[serde(with = "utils::serialize::point::base64")]
    pub eval: CG,
    proof: NIZKDleqProof<CG, H>,
}

impl<CG, H> EcdhCoinTossEval<CG, H>
where
    H: Default + DynDigest + BlockSizeUser + Clone,
    CG: CurveGroup + Copy + HashToCurve + PointSerializeCompressed,
{
    /// Outputs an evaluation Z_i = [si]H(coin input) and a discrete logarithm equivalence proof that there exists s s.t. [s]G = VK and [s]H(coin input) = Z_i.
    pub fn eval<RNG>(
        si: &CG::ScalarField,
        coin_input: &[u8],
        g: &CG,
        rng: &mut RNG,
    ) -> Result<Self, EcdhCoinTossError>
    where
        RNG: RngCore + CryptoRng,
    {
        let g_tilde = Self::get_g_tilde(coin_input);

        let vk = *g * si;
        let eval = g_tilde * si;
        let proof = NIZKDleqProof::prove(si, g, &g_tilde, &vk, &eval, ECDH_COIN_TOSS_NIZK_DST, rng)
            .map_err(|_| EcdhCoinTossError)?;

        Ok(EcdhCoinTossEval { eval, proof })
    }

    /// Verify the correctness of the evaluation by checking it against the verification key and the coin input.
    pub fn verify(&self, vk: &CG, coin_input: &[u8], g: &CG) -> Result<(), EcdhCoinTossError> {
        let g_tilde = Self::get_g_tilde(coin_input);

        self.verify_internal(vk, g, &g_tilde)
    }

    /// Given t different valid evaluations, their respective identifiers and verification keys, output the common coin defined by the most significant bit of SHA3-256(\sum_{i \in identifiers} [\lambda_i]Z_i) = SHA3-256(\sum [\lambda_i]H(common coin)).
    /// With less than t valid evaluations, get_coin returns an error.
    pub fn get_coin(
        evals: &[&Self],
        identifiers: &[PartyId],
        vks: &[CG],
        coin_input: &[u8],
        g: &CG,
        t: usize,
    ) -> Result<Coin, EcdhCoinTossError> {
        // All three arrays must be the same length >= t
        if evals.len() < t || evals.len() != identifiers.len() || evals.len() != vks.len() {
            Err(EcdhCoinTossError)?
        }

        let g_tilde = Self::get_g_tilde(coin_input);

        // Verify the correctness of each evaluation and separate the valid/invalid into two lists
        // This is done to have a constant flow.
        let (valid, invalid): (Vec<_>, Vec<_>) =
            izip!(identifiers, evals, vks).partition_map(|(&id, eval, vk)| {
                if eval.verify_internal(vk, g, &g_tilde).is_ok() {
                    itertools::Either::Left((id.into(), eval.eval))
                } else {
                    itertools::Either::Right((id.into(), eval.eval))
                }
            });

        // Count the number of valid
        let mut good = valid.len() >= t;

        // Append original evals to point to have a vector of valid and invalid evals, of length at least t
        let points = [valid, invalid].concat();

        // Lagrange interpolation with the first t points
        let g_tilde_0 = lagrange_points_interpolate_at(&points[..t], 0);

        // Serialize the resulting point
        let ser = match g_tilde_0.ser() {
            Ok(ser) => ser,
            Err(_) => {
                good = false;
                vec![0]
            }
        };

        // The coin is defined by the most significant bit
        let coin = (Sha3_256::digest(&ser)[0] >> 7) & 0b1;
        if good {
            Ok(coin.try_into().unwrap()) // coin is always 0 or 1 => cannot fail
        } else {
            Err(EcdhCoinTossError)
        }
    }

    /// Verify the validity of a proof.
    fn verify_internal(&self, vk: &CG, g: &CG, g_tilde: &CG) -> Result<(), EcdhCoinTossError> {
        self.proof
            .verify(g, g_tilde, vk, &self.eval, ECDH_COIN_TOSS_NIZK_DST)
            .map_err(|_| EcdhCoinTossError)
    }

    /// Get the g tilde generator by hashing the input.
    fn get_g_tilde(input: &[u8]) -> CG
    where
        CG: CurveGroup + HashToCurve,
    {
        CG::hash_to_curve(input, ECDH_COIN_TOSS_GENERATOR_DST)
    }
}

impl TryFrom<u8> for Coin {
    type Error = ();
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Coin::Zero),
            1 => Ok(Coin::One),
            _ => Err(()),
        }
    }
}

impl From<Coin> for u8 {
    fn from(value: Coin) -> Self {
        value as u8
    }
}

#[cfg(test)]
mod tests {
    use super::{Coin, EcdhCoinTossEval};
    use crate::helpers::{PartyId, eval_poly};
    use ark_ec::pairing::Pairing;
    use ark_ec::{CurveGroup, Group};
    use ark_std::UniformRand;
    use rand::{Rng, thread_rng};
    use std::collections::HashSet;
    use std::sync::atomic::AtomicUsize;

    type CG = <ark_bn254::Bn254 as Pairing>::G1;

    fn get_shares<CG: CurveGroup>(g: &CG, n: usize, t: usize) -> (Vec<CG::ScalarField>, Vec<CG>) {
        let poly: Vec<_> = (0..=t)
            .map(|_| CG::ScalarField::rand(&mut thread_rng()))
            .collect();

        let shares: Vec<CG::ScalarField> = PartyId::iter_all(n)
            .map(|i| eval_poly(&u64::from(i).into(), &poly))
            .collect();
        let public_shares: Vec<_> = shares.iter().map(|s_i| *g * s_i).collect();

        (shares, public_shares)
    }

    #[test]
    fn test_consistency() {
        let n = 7;
        let t = 2;
        let g = CG::generator();
        let coin_input = b"test_consistency";
        let (shares, public_shares) = get_shares(&g, n, t);

        let eval_1 = EcdhCoinTossEval::<CG, sha3::Sha3_256>::eval(
            &shares[0],
            coin_input,
            &g,
            &mut thread_rng(),
        )
        .unwrap();

        let eval_2 = EcdhCoinTossEval::<CG, sha3::Sha3_256>::eval(
            &shares[1],
            coin_input,
            &g,
            &mut thread_rng(),
        )
        .unwrap();

        assert!(
            EcdhCoinTossEval::get_coin(
                &[&eval_1, &eval_2],
                &[PartyId(1), PartyId(2)],
                &[public_shares[0], public_shares[1]],
                coin_input,
                &g,
                t
            )
            .is_ok()
        );
    }

    #[test]
    #[ignore]
    fn count_tosses_seq_input() {
        let n = 7;
        let t = 3;
        let g = CG::generator();
        let (shares, public_shares) = get_shares(&g, n, t);

        let tosses_zero = AtomicUsize::default();
        let tosses_one = AtomicUsize::default();
        let ctr = AtomicUsize::default();

        use rayon::prelude::*;

        (0..10000).into_par_iter().for_each(|i| {
            let input = format!("test_{i}");
            let mut parties = HashSet::new();
            while parties.len() < t {
                parties.insert(rand::thread_rng().gen_range(0..n));
            }

            let (parties, vks, evals): (Vec<_>, Vec<_>, Vec<_>) = parties
                .into_iter()
                .map(|idx| {
                    let eval = EcdhCoinTossEval::<CG, sha3::Sha3_256>::eval(
                        &shares[idx],
                        input.as_bytes(),
                        &g,
                        &mut thread_rng(),
                    )
                    .unwrap();
                    (PartyId::from_index(idx), public_shares[idx], eval)
                })
                .collect();

            let coin = EcdhCoinTossEval::get_coin(
                evals.iter().collect::<Vec<_>>().as_slice(),
                &parties,
                &vks,
                input.as_bytes(),
                &g,
                t,
            )
            .unwrap();

            match coin {
                Coin::Zero => {
                    tosses_zero.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                }
                Coin::One => {
                    tosses_one.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                }
            }
            let ctr = ctr.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            if ctr % 200 == 0 {
                let zeros = tosses_zero.load(std::sync::atomic::Ordering::Relaxed);
                let ones = tosses_one.load(std::sync::atomic::Ordering::Relaxed);
                println!("ctr = {ctr}");
                println!("tosses_zero: {zeros}");
                println!("tosses_one : {ones}");
            }
        });

        println!("tosses_zero: {}", tosses_zero.into_inner());
        println!("tosses_one : {}", tosses_one.into_inner());
    }

    #[test]
    fn filters_invalid_share() {
        let n = 7;
        let t = 2;
        let g = CG::generator();
        let coin_input = b"filters_invalid_share";
        let (shares, public_shares) = get_shares(&g, n, t);

        let eval_1 = EcdhCoinTossEval::<CG, sha3::Sha3_256>::eval(
            &shares[0],
            coin_input,
            &g,
            &mut thread_rng(),
        )
        .unwrap();

        let mut eval_2 = EcdhCoinTossEval::<CG, sha3::Sha3_256>::eval(
            &shares[1],
            coin_input,
            &g,
            &mut thread_rng(),
        )
        .unwrap();

        let eval_3 = EcdhCoinTossEval::<CG, sha3::Sha3_256>::eval(
            &shares[2],
            coin_input,
            &g,
            &mut thread_rng(),
        )
        .unwrap();

        // We corrupt the second eval by doubling the point
        eval_2.eval.double_in_place();

        assert!(
            EcdhCoinTossEval::get_coin(
                &[&eval_1, &eval_2, &eval_3],
                &[PartyId(1), PartyId(2), PartyId(3)],
                &[public_shares[0], public_shares[1], public_shares[2]],
                coin_input,
                &g,
                t
            )
            .is_ok()
        );
    }

    #[test]
    fn too_many_invalid_shares() {
        let n = 7;
        let t = 2;
        let g = CG::generator();
        let coin_input = b"filters_invalid_share";
        let (shares, public_shares) = get_shares(&g, n, t);

        let eval_1 = EcdhCoinTossEval::<CG, sha3::Sha3_256>::eval(
            &shares[0],
            coin_input,
            &g,
            &mut thread_rng(),
        )
        .unwrap();

        let mut eval_2 = EcdhCoinTossEval::<CG, sha3::Sha3_256>::eval(
            &shares[1],
            coin_input,
            &g,
            &mut thread_rng(),
        )
        .unwrap();

        let mut eval_3 = EcdhCoinTossEval::<CG, sha3::Sha3_256>::eval(
            &shares[2],
            coin_input,
            &g,
            &mut thread_rng(),
        )
        .unwrap();

        // We corrupt the second and third eval by doubling the point
        eval_2.eval.double_in_place();
        eval_3.eval.double_in_place();

        assert!(
            EcdhCoinTossEval::get_coin(
                &[&eval_1, &eval_2, &eval_3],
                &[PartyId(1), PartyId(2), PartyId(3)],
                &[public_shares[0], public_shares[1], public_shares[2]],
                coin_input,
                &g,
                t
            )
            .is_err()
        );
    }
}
