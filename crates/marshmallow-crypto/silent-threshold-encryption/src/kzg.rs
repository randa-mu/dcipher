//adapted from https://github.com/arkworks-rs/poly-commit/blob/master/src/kzg10/mod.rs
#![allow(dead_code)]
#![allow(unused_imports)]

use ark_ec::{pairing::Pairing, CurveGroup, Group};
use ark_ec::{scalar_mul::fixed_base::FixedBase, VariableBaseMSM};
use ark_ff::{One, PrimeField, UniformRand, Zero};
use ark_poly::DenseUVPolynomial;
use ark_std::{format, marker::PhantomData, ops::*, vec};

use ark_std::rand::RngCore;

use crate::errors::KzgError;

pub struct KZG10<E: Pairing, P: DenseUVPolynomial<E::ScalarField>> {
    _engine: PhantomData<E>,
    _poly: PhantomData<P>,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct UniversalParams<E: Pairing> {
    /// Group elements of the form `{ \beta^i G }`, where `i` ranges from 0 to `degree`.
    #[cfg_attr(
        feature = "serde",
        serde(
            serialize_with = "crate::encode::ser_vec_base64",
            deserialize_with = "crate::encode::deser_vec_base64"
        )
    )]
    pub powers_of_g: Vec<E::G1Affine>,
    /// Group elements of the form `{ \beta^i H }`, where `i` ranges from 0 to `degree`.
    #[cfg_attr(
        feature = "serde",
        serde(
            serialize_with = "crate::encode::ser_vec_base64",
            deserialize_with = "crate::encode::deser_vec_base64"
        )
    )]
    pub powers_of_h: Vec<E::G2Affine>,
}

impl<E, P> KZG10<E, P>
where
    E: Pairing,
    P: DenseUVPolynomial<E::ScalarField, Point = E::ScalarField>,
    for<'a, 'b> &'a P: Div<&'b P, Output = P>,
    for<'a, 'b> &'a P: Sub<&'b P, Output = P>,
{
    pub fn setup<R: RngCore>(
        max_degree: usize,
        rng: &mut R,
    ) -> Result<UniversalParams<E>, KzgError> {
        if max_degree < 1 {
            return Err(KzgError::DegreeIsZero);
        }

        //let setup_time = start_timer!(|| format!("KZG10::Setup with degree {}", max_degree));
        let beta = E::ScalarField::rand(rng);
        let g = E::G1::generator();
        let h = E::G2::generator();

        let mut powers_of_beta = vec![E::ScalarField::one()];

        let mut cur = beta;
        for _ in 0..max_degree {
            powers_of_beta.push(cur);
            cur *= &beta;
        }

        let window_size = FixedBase::get_mul_window_size(max_degree + 1);
        let scalar_bits = E::ScalarField::MODULUS_BIT_SIZE as usize;

        let g_table = FixedBase::get_window_table(scalar_bits, window_size, g);
        let powers_of_g =
            FixedBase::msm::<E::G1>(scalar_bits, window_size, &g_table, &powers_of_beta);

        let h_table = FixedBase::get_window_table(scalar_bits, window_size, h);
        let powers_of_h =
            FixedBase::msm::<E::G2>(scalar_bits, window_size, &h_table, &powers_of_beta);

        let powers_of_g = E::G1::normalize_batch(&powers_of_g);
        let powers_of_h = E::G2::normalize_batch(&powers_of_h);

        let pp = UniversalParams {
            powers_of_g,
            powers_of_h,
        };

        //end_timer!(setup_time);
        Ok(pp)
    }

    pub fn commit_g1(params: &UniversalParams<E>, polynomial: &P) -> Result<E::G1Affine, KzgError> {
        let d = polynomial.degree();
        check_degree_is_too_large(d, params.powers_of_g.len())?;

        let plain_coeffs = convert_to_bigints(polynomial.coeffs());

        let powers_of_g = &params.powers_of_g[..=d].to_vec();
        //let msm_time = start_timer!(|| "MSM to compute commitment to plaintext poly");
        let commitment = <E::G1 as VariableBaseMSM>::msm_bigint(&powers_of_g[..], &plain_coeffs);
        //end_timer!(msm_time);
        Ok(commitment.into_affine())
    }

    pub fn commit_g2(params: &UniversalParams<E>, polynomial: &P) -> Result<E::G2Affine, KzgError> {
        let d = polynomial.degree();
        check_degree_is_too_large(d, params.powers_of_h.len())?;

        let plain_coeffs = convert_to_bigints(polynomial.coeffs());

        let powers_of_h = &params.powers_of_h[..=d].to_vec();
        //let msm_time = start_timer!(|| "MSM to compute commitment to plaintext poly");
        let commitment = <E::G2 as VariableBaseMSM>::msm_bigint(&powers_of_h[..], &plain_coeffs);
        //end_timer!(msm_time);

        Ok(commitment.into_affine())
    }

    pub fn compute_opening_proof(
        params: &UniversalParams<E>,
        polynomial: &P,
        point: &E::ScalarField,
    ) -> Result<E::G1Affine, KzgError> {
        let eval = polynomial.evaluate(point);
        let eval_as_poly = P::from_coefficients_vec(vec![eval]);
        let numerator = polynomial.clone().sub(&eval_as_poly);
        let divisor =
            P::from_coefficients_vec(vec![E::ScalarField::zero() - point, E::ScalarField::one()]);
        let witness_polynomial = numerator.div(&divisor);

        Self::commit_g1(params, &witness_polynomial)
    }
}

fn skip_leading_zeros_and_convert_to_bigints<F: PrimeField, P: DenseUVPolynomial<F>>(
    p: &P,
) -> (usize, Vec<F::BigInt>) {
    let mut num_leading_zeros = 0;
    while num_leading_zeros < p.coeffs().len() && p.coeffs()[num_leading_zeros].is_zero() {
        num_leading_zeros += 1;
    }
    let coeffs = convert_to_bigints(&p.coeffs()[num_leading_zeros..]);
    (num_leading_zeros, coeffs)
}

pub fn convert_to_bigints<F: PrimeField>(p: &[F]) -> Vec<F::BigInt> {
    p.iter().map(|s| s.into_bigint()).collect::<Vec<_>>()
}

fn check_degree_is_too_large(degree: usize, num_powers: usize) -> Result<(), KzgError> {
    let num_coefficients = degree + 1;
    if num_coefficients > num_powers {
        Err(KzgError::TooManyCoefficients {
            num_coefficients,
            num_powers,
        })
    } else {
        Ok(())
    }
}
