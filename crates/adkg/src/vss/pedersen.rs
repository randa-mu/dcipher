//! Pedersen polynomial commitment based on https://eprint.iacr.org/2021/777.pdf, Figure 4
//! Outputs a Pedersen commitment to each coefficient and outputs two randomly generated polynomials.

pub mod commit;

use crate::helpers::{PartyId, eval_poly};
use ark_ec::CurveGroup;
use ark_poly::{DenseUVPolynomial, Polynomial, univariate::DensePolynomial};
use ark_std::UniformRand;
use commit::*;
use rand::RngCore;
use serde::{Deserialize, Serialize};
use utils::serialize::fq::{FqDeserialize, FqSerialize};

struct PedersenPolyCommit<CG: CurveGroup> {
    v: Vec<CG>,
    s: Vec<CG::ScalarField>,
    r: Vec<CG::ScalarField>,
}

pub struct PedersenVssShare<CG: CurveGroup>(PedersenPolyCommit<CG>);

#[derive(thiserror::Error, Debug)]
#[error("opaque pedersen error")]
pub struct PedersenError;

#[derive(Serialize, Deserialize, Clone)]
#[serde(bound(serialize = "F: FqSerialize", deserialize = "F: FqDeserialize"))]
pub struct PedersenPartyShare<F> {
    #[serde(with = "utils::serialize::fq::base64_or_bytes")]
    pub si: F,

    #[serde(with = "utils::serialize::fq::base64_or_bytes")]
    pub ri: F,
}

impl<CG: CurveGroup> PedersenVssShare<CG> {
    pub fn get_public_poly(&self) -> &[CG] {
        &self.0.v
    }

    pub fn get_party_secrets(
        &self,
        party: &PartyId,
    ) -> Option<PedersenPartyShare<CG::ScalarField>> {
        if party == &PartyId(0) {
            None?
        } else {
            let si = *self.0.s.get(party.as_index())?;
            let ri = *self.0.r.get(party.as_index())?;
            Some(PedersenPartyShare { si, ri })
        }
    }
}

/// Outputs a Pedersen commitment to a polynomial of a specific degree with constant secrets s and r.
fn poly_commit<CG: CurveGroup>(
    s: &CG::ScalarField,
    r: &CG::ScalarField,
    degree: u64,
    n: u64,
    g: &CG,
    h: &CG,
    rng: &mut impl RngCore,
) -> PedersenPolyCommit<CG> {
    // (1): Compute Pedersen commitment v_0
    let v_0 = commit(s, r, g, h);

    // (2): Generate random coefficients
    let aks: Vec<CG::ScalarField> = std::iter::once(*s)
        .chain((0..degree).map(|_| CG::ScalarField::rand(rng)))
        .collect();
    let bks: Vec<CG::ScalarField> = std::iter::once(*r)
        .chain((0..degree).map(|_| CG::ScalarField::rand(rng)))
        .collect();
    let p = DensePolynomial::from_coefficients_slice(&aks);
    let phi = DensePolynomial::from_coefficients_slice(&bks);

    // (3): Compute Pedersen commitments v_0, ..., v_t
    // todo optimization: use scalar mult for fixed g and h
    let vks: Vec<CG> = std::iter::once(v_0)
        .chain(
            aks.into_iter()
                .zip(bks)
                .skip(1) // skip the first commitment
                .map(|(ak, bk)| commit(&ak, &bk, g, h)),
        )
        .collect();

    // (4): Evaluate the polynomials at n points
    let s: Vec<CG::ScalarField> = (1..=n).map(|i| p.evaluate(&i.into())).collect();
    let r: Vec<CG::ScalarField> = (1..=n).map(|i| phi.evaluate(&i.into())).collect();

    PedersenPolyCommit { s, r, v: vks }
}

/// Verifies that the commitment at index i is correct by evaluating the polynomial in the exponent.
pub fn eval_verify<CG: CurveGroup>(
    v: &[CG],
    i: u64,
    si: &CG::ScalarField,
    ri: &CG::ScalarField,
    g: &CG,
    h: &CG,
) -> Result<(), PedersenError> {
    let expected = eval_poly(&i.into(), v);
    if expected == commit(si, ri, g, h) {
        Ok(())
    } else {
        Err(PedersenError)
    }
}

pub fn share<CG: CurveGroup>(
    s: &CG::ScalarField,
    r: &CG::ScalarField,
    g: &CG,
    h: &CG,
    n: u64,
    t: u64,
    rng: &mut impl RngCore,
) -> PedersenVssShare<CG> {
    PedersenVssShare(poly_commit(s, r, t, n, g, h, rng))
}

#[cfg(test)]
mod tests {
    use super::*;
    use ark_ec::{Group, pairing::Pairing};
    use ark_std::UniformRand;
    use utils::hash_to_curve::HashToCurve;

    #[test]
    fn ped_poly_commit_consistency() {
        use ark_bn254::Bn254;
        let g = <Bn254 as Pairing>::G1::generator();
        let h = ark_bn254::G1Projective::hash_to_curve(b"PEDERSEN_H", b"TEST_DST_PEDERSEN_H");

        let t = 2;
        let n = 2;
        let mut rng = rand::thread_rng();
        let s = <Bn254 as Pairing>::ScalarField::rand(&mut rng);
        let r = <Bn254 as Pairing>::ScalarField::rand(&mut rng);

        let PedersenPolyCommit { s, r, v } = poly_commit(&s, &r, t, n, &g, &h, &mut rng);
        for (i, (si, ri)) in s.iter().zip(r.iter()).enumerate() {
            let i = u64::try_from(i + 1).unwrap();
            assert!(eval_verify(&v, i, si, ri, &g, &h).is_ok());
        }
    }

    #[test]
    fn ped_poly_commit_invalid() {
        use ark_bn254::Bn254;
        let g = <Bn254 as Pairing>::G1::generator();
        let h = ark_bn254::G1Projective::hash_to_curve(b"PEDERSEN_H", b"TEST_DST_PEDERSEN_H");

        let t = 2;
        let n = 2;
        let mut rng = rand::thread_rng();
        let s = <Bn254 as Pairing>::ScalarField::rand(&mut rng);
        let r = <Bn254 as Pairing>::ScalarField::rand(&mut rng);

        let PedersenPolyCommit { s, r, v } = poly_commit(&s, &r, t, n, &g, &h, &mut rng);

        // Try to open commitment 1 with id 0, should fail
        assert!(eval_verify(&v, u64::try_from(0).unwrap(), &s[0], &r[0], &g, &h).is_err());
        // Try to open commitment 1 with id 1, should succeed
        assert!(eval_verify(&v, u64::try_from(1).unwrap(), &s[0], &r[0], &g, &h).is_ok());
        // Try to open commitment 2 with id 1, should fail
        assert!(eval_verify(&v, u64::try_from(1).unwrap(), &s[1], &r[1], &g, &h).is_err());
    }

    #[test]
    fn vss_parties_should_start_from_one() {
        use ark_bn254::Bn254;
        let g = <Bn254 as Pairing>::G1::generator();
        let h = ark_bn254::G1Projective::hash_to_curve(b"PEDERSEN_H", b"TEST_DST_PEDERSEN_H");

        let t = 2;
        let n = 2;
        let mut rng = rand::thread_rng();
        let s = <Bn254 as Pairing>::ScalarField::rand(&mut rng);
        let r = <Bn254 as Pairing>::ScalarField::rand(&mut rng);

        let vss = share(&s, &r, &g, &h, n, t, &mut rng);

        assert!(vss.get_party_secrets(&PartyId(0)).is_none());
        assert_eq!(
            vss.get_party_secrets(&PartyId(1)).map(|sk| sk.si),
            Some(vss.0.s[0])
        );
        assert_eq!(
            vss.get_party_secrets(&PartyId(1)).map(|sk| sk.ri),
            Some(vss.0.r[0])
        );
        assert_eq!(
            vss.get_party_secrets(&PartyId(2)).map(|sk| sk.si),
            Some(vss.0.s[1])
        );
        assert_eq!(
            vss.get_party_secrets(&PartyId(2)).map(|sk| sk.ri),
            Some(vss.0.r[1])
        );
        assert!(vss.get_party_secrets(&PartyId(n as usize + 2)).is_none());
    }
}
