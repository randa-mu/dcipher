use ark_ec::pairing::Pairing;
use ark_ec::{CurveGroup, PrimeGroup, VariableBaseMSM};
use ark_ff::{BigInteger, Field, One, PrimeField};
use ark_poly::univariate::DensePolynomial;
use ark_poly::{DenseUVPolynomial, Polynomial};
use ark_std::UniformRand;
use base64::prelude::*;
use clap::{Arg, Command};
use libp2p::identity::Keypair;
use rand::rngs::OsRng;
use std::str::FromStr;
use strum::EnumString;
use utils::serialize::point::PointSerializeCompressed;

#[derive(Copy, Clone, Debug, EnumString)]
enum Scheme {
    Bn254,
}

fn main() -> anyhow::Result<()> {
    let args = Command::new("gen-keys")
        .arg(
            Arg::new("nodes")
                .short('n')
                .help("Number of nodes")
                .required(true)
                .value_parser(clap::value_parser!(u16)),
        )
        .arg(
            Arg::new("threshold")
                .short('t')
                .help("Threshold")
                .required(true)
                .value_parser(clap::value_parser!(u16)),
        )
        .arg(
            Arg::new("scheme")
                .long("scheme")
                .help("The scheme to use")
                .default_value("Bn254")
                .value_parser(clap::value_parser!(String)),
        )
        .get_matches();

    let n: u16 = *args.get_one("nodes").unwrap();
    let t: u16 = *args.get_one("threshold").unwrap();
    let scheme: Scheme = Scheme::from_str(args.get_one::<String>("scheme").unwrap())?;

    match scheme {
        Scheme::Bn254 => {
            println!("Generating bn254 keys:\n");
            gen_keys::<ark_bn254::Bn254>(n, t)
        }
    }
}

fn gen_keys<E: Pairing>(n: u16, t: u16) -> anyhow::Result<()>
where
    E::G1: PointSerializeCompressed,
    E::G2: PointSerializeCompressed,
{
    // Build polynomial from coefficients
    let poly_coeffs = (0..t)
        .map(|_| E::ScalarField::rand(&mut OsRng))
        .collect::<Vec<_>>();
    let p = DensePolynomial::from_coefficients_slice(&poly_coeffs);
    let sks = (1..=n).map(|i| p.evaluate(&i.into())).collect::<Vec<_>>();

    for i in 1..=n {
        let ski = sks[i as usize - 1];
        let pki_g1 = E::G1::generator() * ski;
        let pki_g2 = E::G2::generator() * ski;
        let libp2p_ski = ::libp2p::identity::Keypair::generate_ed25519();

        println!(
            "node {i}: bls private key    = {}",
            hex::encode(ski.into_bigint().to_bytes_be())
        );
        println!(
            "node {i}: bls public key g1  = {}",
            pki_g1.ser_compressed_base64()?
        );
        println!(
            "node {i}: bls public key g2  = {}",
            pki_g2.ser_compressed_base64()?
        );
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
        .map(|(i, ski)| (i.into(), E::G2::generator() * ski))
        .collect::<Vec<_>>();
    let exp_pk_g1 = E::G1::generator() * poly_coeffs[0];
    let exp_pk_g2 = E::G2::generator() * poly_coeffs[0];
    let pk_g2 = lagrange_points_interpolate_at(&points, 0);
    assert_eq!(pk_g2.into_affine(), exp_pk_g2.into_affine());

    println!(
        "group bls public key g1    = {}",
        exp_pk_g1.ser_compressed_base64()?
    );
    println!(
        "group bls public key g2    = {}",
        exp_pk_g2.ser_compressed_base64()?
    );
    Ok(())
}

fn encode_libp2p(sk: &Keypair) -> String {
    BASE64_STANDARD.encode(sk.to_protobuf_encoding().unwrap())
}

/// Lagrange interpolation of the polynomial defined by its points, evaluated at point eval_x.
pub fn lagrange_points_interpolate_at<G>(points: &[(u64, G)], eval_x: u64) -> G
where
    G: VariableBaseMSM + PrimeGroup,
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
