//! Implementation of Feldman's VSS using arkworks.
use crate::helpers::{PartyId, eval_poly, u64_from_usize};
use ark_ec::CurveGroup;
use ark_poly::{DenseUVPolynomial, Polynomial, univariate::DensePolynomial};
use ark_std::UniformRand;
use rand::RngCore;
use thiserror::Error;

/// Feldman polynomial commitment
struct FeldmanPolyCommit<CG: CurveGroup> {
    v: Vec<CG>,
    s: Vec<CG::ScalarField>,
}

pub struct FeldmanVssShare<CG: CurveGroup>(FeldmanPolyCommit<CG>);

#[derive(Error, Debug, PartialEq)]
#[error("opaque feldman vss error")]
pub struct FeldmanError;

impl<CG: CurveGroup> FeldmanVssShare<CG> {
    pub fn get_public_poly(&self) -> &[CG] {
        &self.0.v
    }

    pub fn get_party_secrets(&self, party: PartyId) -> Option<&CG::ScalarField> {
        if usize::from(party) == 0 {
            None?
        } else {
            Some(self.0.s.get(party.as_index())?)
        }
    }
}

/// Generates a random polynomial f of degree t with f(0) = s with a commitment to each coefficient
/// of the polynomial.
fn poly_rand_coeffs_commit<CG: CurveGroup>(
    s: &CG::ScalarField,
    degree: usize,
    n: usize,
    g: &CG,
    rng: &mut impl RngCore,
) -> FeldmanPolyCommit<CG> {
    // Generate the rest of the random coefficients
    let aks: Vec<CG::ScalarField> = (0..degree).map(|_| CG::ScalarField::rand(rng)).collect();

    // Commit to p(x) = s + r_0 x + ... + r_n x
    poly_commit(&[&[*s][..], &aks].concat(), n, g)
}

/// Outputs a Feldman commitment to each coefficient of poly_coeffs alongside the polynomial build from poly_coeffs and a random polynomial.
/// The polynomials can be subsequently evaluated with:
fn poly_commit<CG: CurveGroup>(
    poly_coeffs: &[CG::ScalarField],
    n: usize,
    g: &CG,
) -> FeldmanPolyCommit<CG> {
    // Build polynomial from coefficients
    let p = DensePolynomial::from_coefficients_slice(poly_coeffs);

    // Compute commitments v_0 = g^{s_0}, ..., g^{s_t}
    // todo optimization: we could use a windowed scalar multiplication since the point g is fixed here.
    //  this could actually be done throughout the code base for fixed points.
    let vks: Vec<CG> = poly_coeffs.iter().map(|ak| *g * ak).collect();

    // Evaluate the polynomial at n points
    let s: Vec<CG::ScalarField> = (1..=n)
        .map(|i| p.evaluate(&u64_from_usize(i).into()))
        .collect();

    FeldmanPolyCommit { s, v: vks }
}

/// Verifies that the commitment at index i is correct by evaluating the polynomial in the exponent.
pub fn eval_verify<CG: CurveGroup>(
    v: &[CG],
    i: PartyId,
    si: &CG::ScalarField,
    g: &CG,
) -> Result<(), FeldmanError> {
    // Two ways to evaluate the polynomial,
    //   1. We use Horner's method to evaluate it in n scalar multiplications
    //   2. We first compute the nth powers, and then compute a multi-scalar multiplication
    let expected = eval_poly(&u64::from(i).into(), v);
    if expected == *g * si {
        Ok(())
    } else {
        Err(FeldmanError)
    }
}

pub fn share<CG: CurveGroup>(
    s: &CG::ScalarField,
    g: &CG,
    n: usize,
    t: usize,
    rng: &mut impl RngCore,
) -> FeldmanVssShare<CG> {
    FeldmanVssShare(poly_rand_coeffs_commit(s, t, n, g, rng))
}

#[cfg(test)]
mod tests {
    use super::*;
    use ark_ec::{Group, pairing::Pairing};
    use ark_std::UniformRand;

    #[test]
    fn feld_poly_commit_consistency() {
        use ark_bn254::Bn254;
        let g = <Bn254 as Pairing>::G1::generator();

        let t = 2;
        let n = 2;
        let mut rng = rand::thread_rng();
        let s = <Bn254 as Pairing>::ScalarField::rand(&mut rng);

        let FeldmanPolyCommit { s, v } = poly_rand_coeffs_commit(&s, t, n, &g, &mut rng);
        for (i, si) in s.iter().enumerate() {
            assert!(eval_verify(&v, PartyId(i + 1), si, &g).is_ok());
        }
    }

    #[test]
    fn feld_poly_commit_invalid() {
        use ark_bn254::Bn254;
        let g = <Bn254 as Pairing>::G1::generator();

        let t = 2;
        let n = 2;
        let mut rng = rand::thread_rng();
        let s = <Bn254 as Pairing>::ScalarField::rand(&mut rng);
        let FeldmanPolyCommit { s, v } = poly_rand_coeffs_commit(&s, t, n, &g, &mut rng);

        // Try to open commitment 1 with id 999, should fail
        assert_eq!(
            eval_verify(&v, 999usize.into(), &s[0], &g),
            Err(FeldmanError)
        );
        // Try to open commitment 1 with id 1, should succeed
        assert_eq!(eval_verify(&v, 1usize.into(), &s[0], &g), Ok(()));
        // Try to open commitment 2 with id 1, should fail
        assert_eq!(eval_verify(&v, 1usize.into(), &s[1], &g), Err(FeldmanError));
    }

    #[test]
    fn vss_parties_should_start_from_one() {
        use ark_bn254::Bn254;
        let g = <Bn254 as Pairing>::G1::generator();

        let t = 2;
        let n = 2;
        let mut rng = rand::thread_rng();
        let s = <Bn254 as Pairing>::ScalarField::rand(&mut rng);
        let vss = share(&s, &g, n, t, &mut rng);

        assert_eq!(vss.get_party_secrets(999usize.into()), None);
        assert_eq!(vss.get_party_secrets(1usize.into()), Some(&vss.0.s[0]));
        assert_eq!(vss.get_party_secrets(2usize.into()), Some(&vss.0.s[1]));
        assert_eq!(vss.get_party_secrets((n + 2).into()), None);
    }
}
