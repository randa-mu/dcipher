use alloy::signers::k256::elliptic_curve::rand_core::OsRng;
use ark_bn254::Fr;
use ark_ec::{CurveGroup, Group, VariableBaseMSM};
use ark_ff::{BigInteger, Field, One, PrimeField};
use ark_poly::univariate::DensePolynomial;
use ark_poly::{DenseUVPolynomial, Polynomial};
use ark_std::UniformRand;
use base64::prelude::*;
use clap::{Arg, Command};
use libp2p::identity::Keypair;
use pairing_utils::serialize::point::PointSerializeCompressed;

fn main() -> anyhow::Result<()> {
    let args = Command::new("gen-keys")
        .arg(
            Arg::new("nodes")
                .short('n')
                .help("Number of nodes")
                .required(true)
                .value_parser(clap::value_parser!(u32)),
        )
        .arg(
            Arg::new("threshold")
                .short('t')
                .help("Threshold")
                .required(true)
                .value_parser(clap::value_parser!(u32)),
        )
        .get_matches();

    let n: u32 = *args.get_one("nodes").unwrap();
    let t: u32 = *args.get_one("threshold").unwrap();

    // Build polynomial from coefficients
    let poly_coeffs = (0..t).map(|_| Fr::rand(&mut OsRng)).collect::<Vec<_>>();
    let p = DensePolynomial::from_coefficients_slice(&poly_coeffs);
    let sks = (1..=n).map(|i| p.evaluate(&i.into())).collect::<Vec<_>>();

    for i in 1..=n {
        let ski = sks[i as usize - 1];
        let pki = ark_bn254::G2Projective::generator() * ski;
        let libp2p_ski = ::libp2p::identity::Keypair::generate_ed25519();

        println!(
            "node {i}: bls private key    = {}",
            hex::encode(ski.into_bigint().to_bytes_be())
        );
        println!("node {i}: bls public key     = {}", pki.ser_base64()?);
        println!(
            "node {i}: libp2p private key = {}",
            encode_libp2p(&libp2p_ski)
        );
        println!(
            "node {i}: libp2p peer id     = {}",
            libp2p_ski.public().to_peer_id()
        );

        println!();
    }

    let points = (1..=n)
        .zip(sks)
        .map(|(i, ski)| (i.into(), ark_bn254::G2Projective::generator() * ski))
        .collect::<Vec<_>>();
    let skp = lagrange_points_interpolate_at(&points, 0);
    assert_eq!(
        skp.into_affine(),
        (ark_bn254::G2Projective::generator() * poly_coeffs[0]).into_affine()
    );

    Ok(())
}

fn encode_libp2p(sk: &Keypair) -> String {
    BASE64_STANDARD.encode(&sk.to_protobuf_encoding().unwrap())
}

/// Lagrange interpolation of the polynomial defined by its points, evaluated at point eval_x.
pub fn lagrange_points_interpolate_at<G>(points: &[(u64, G)], eval_x: u64) -> G
where
    G: VariableBaseMSM + Group,
    G::ScalarField: PrimeField,
{
    let eval_point: G::ScalarField = eval_x.into();
    let scalars = points
        .iter()
        .enumerate()
        .map(|(i, (x_i, _))| {
            let mut numerator = G::ScalarField::one();
            let mut denominator = G::ScalarField::one();
            let x_i: G::ScalarField = (*x_i).into();

            // Lagrange basis polynomial at eval_point
            for (j, (x_j, _)) in points.iter().enumerate() {
                let x_j: G::ScalarField = (*x_j).into();
                if i != j {
                    numerator *= eval_point - x_j; // (x - x_j)
                    denominator *= x_i - x_j; // (x_i - x_j)
                }
            }

            // L_i(x_i)
            numerator
                * denominator
                    .inverse() // we are in a prime field, i.e., all points have an inverse but zero
                    .expect("received two points with the same identifiers")
        })
        .collect::<Vec<_>>();

    let bases: Vec<G> = points.iter().copied().map(|(_, b)| b).collect();
    let bases = G::batch_convert_to_mul_base(&bases);
    G::msm(&bases, &scalars).expect("msm failed: bases and scalars have different lengths")
}
