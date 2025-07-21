use std::ops::Mul;

use crate::{kzg::UniversalParams, nizk::ShamirNIZKProof, setup::AggregateKey};
use ark_ec::{
    pairing::{Pairing, PairingOutput},
    Group,
};
use ark_serialize::*;
use ark_std::{UniformRand, Zero};
use rand::thread_rng;

// if we want to prevent having a pairing output in the ciphertext,
// we can likely turn the PKE scheme into a KEM by deriving a key
// k = H(ct_3 = enc_key) instead of encrypting a message / key.
// The ciphertext becomes ([\gamma]_2, ct_2) = (gamma_g2, sa1 || sa2).
#[derive(CanonicalSerialize, CanonicalDeserialize, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Ciphertext<E: Pairing> {
    #[cfg_attr(
        feature = "serde",
        serde(
            serialize_with = "crate::encode::ser_base64",
            deserialize_with = "crate::encode::deser_base64"
        )
    )]
    pub gamma_g2: E::G2,

    #[cfg_attr(
        feature = "serde",
        serde(
            serialize_with = "crate::encode::ser_base64",
            deserialize_with = "crate::encode::deser_base64"
        )
    )]
    pub sa1: [E::G1; 2],

    #[cfg_attr(
        feature = "serde",
        serde(
            serialize_with = "crate::encode::ser_base64",
            deserialize_with = "crate::encode::deser_base64"
        )
    )]
    pub sa2: [E::G2; 6],

    #[cfg_attr(
        feature = "serde",
        serde(
            serialize_with = "crate::encode::ser_base64",
            deserialize_with = "crate::encode::deser_base64"
        )
    )]
    pub enc_key: PairingOutput<E>, //key to be used for encapsulation

    pub t: usize, //threshold
}

#[derive(CanonicalSerialize, CanonicalDeserialize, Clone)]
pub struct CiphertextCCA2Generic<E: Pairing, H> {
    pub gamma_g2: E::G2,
    pub sa1: [E::G1; 2],
    pub sa2: [E::G2; 6],
    pub enc_key: PairingOutput<E>, //key to be used for encapsulation
    pub t: usize,                  //threshold
    pub proof: ShamirNIZKProof<E, H>,
}

pub type CiphertextCCA2<E> = CiphertextCCA2Generic<E, sha3::Sha3_256>;

impl<E: Pairing> Ciphertext<E> {
    pub fn new(
        gamma_g2: E::G2,
        sa1: [E::G1; 2],
        sa2: [E::G2; 6],
        enc_key: PairingOutput<E>,
        t: usize,
    ) -> Self {
        Ciphertext {
            gamma_g2,
            sa1,
            sa2,
            enc_key,
            t,
        }
    }
}

impl<E: Pairing, H> From<CiphertextCCA2Generic<E, H>> for Ciphertext<E> {
    fn from(ct: CiphertextCCA2Generic<E, H>) -> Self {
        Self {
            enc_key: ct.enc_key,
            gamma_g2: ct.gamma_g2,
            sa1: ct.sa1,
            sa2: ct.sa2,
            t: ct.t,
        }
    }
}

pub fn encrypt_cca2<E: Pairing>(
    apk: &AggregateKey<E>,
    t: usize,
    msg: PairingOutput<E>,
    params: &UniversalParams<E>,
    gamma: &E::ScalarField,
) -> CiphertextCCA2<E> {
    let gamma_g2 = E::G2::generator() * gamma;
    let ct = encrypt_with_gamma_g2(apk, t, msg, params, &gamma_g2);

    let proof =
        ShamirNIZKProof::<_, sha3::Sha3_256>::new_from_ciphertext(&ct, gamma, &mut thread_rng());

    CiphertextCCA2 {
        gamma_g2: ct.gamma_g2,
        sa1: ct.sa1,
        sa2: ct.sa2,
        enc_key: ct.enc_key,
        t: ct.t,
        proof,
    }
}

/// t is the threshold for encryption and apk is the aggregated public key
pub fn encrypt<E: Pairing>(
    apk: &AggregateKey<E>,
    t: usize,
    params: &UniversalParams<E>,
) -> Ciphertext<E> {
    let msg = PairingOutput::<E>::zero();
    encrypt_with_message(apk, t, msg, params)
}

pub fn encrypt_with_message<E: Pairing>(
    apk: &AggregateKey<E>,
    t: usize,
    msg: PairingOutput<E>,
    params: &UniversalParams<E>,
) -> Ciphertext<E> {
    let mut rng = ark_std::test_rng();
    let gamma_g2 = E::G2::rand(&mut rng);
    encrypt_with_gamma_g2(apk, t, msg, params, &gamma_g2)
}

pub fn encrypt_with_gamma_g2<E: Pairing>(
    apk: &AggregateKey<E>,
    t: usize,
    msg: PairingOutput<E>,
    params: &UniversalParams<E>,
    gamma_g2: &E::G2,
) -> Ciphertext<E> {
    let mut rng = ark_std::test_rng();
    let g = params.powers_of_g[0];
    let h = params.powers_of_h[0];

    // todo: avoid benchmarking this
    // let e_gh = E::pairing(g, h);

    let mut sa1 = [E::G1::generator(); 2];
    let mut sa2 = [E::G2::generator(); 6];

    let mut s: [E::ScalarField; 5] = [E::ScalarField::zero(); 5];

    s.iter_mut()
        .for_each(|s| *s = E::ScalarField::rand(&mut rng));

    // sa1[0] = s0*ask + s3*g^{tau^t} + s4*g
    sa1[0] = (apk.ask * s[0]) + (params.powers_of_g[t] * s[3]) + (params.powers_of_g[0] * s[4]);

    // sa1[1] = s2*g
    sa1[1] = g * s[2];

    // sa2[0] = s0*h + s2*gamma_g2
    sa2[0] = (h * s[0]) + (*gamma_g2 * s[2]);

    // sa2[1] = s0*z_g2
    sa2[1] = apk.z_g2 * s[0];

    // sa2[2] = s0*h^tau + s1*h^tau
    sa2[2] = params.powers_of_h[1] * (s[0] + s[1]);

    // sa2[3] = s1*h
    sa2[3] = h * s[1];

    // sa2[4] = s3*h
    sa2[4] = h * s[3];

    // sa2[5] = s4*h^{tau - omega^0}
    sa2[5] = (params.powers_of_h[1] + apk.h_minus1) * s[4];

    // enc_key = s4*e_gh
    let enc_key = apk.e_gh.mul(s[4]) + msg;

    Ciphertext {
        gamma_g2: *gamma_g2,
        sa1,
        sa2,
        enc_key,
        t,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        kzg::KZG10,
        setup::{PublicKey, SecretKey},
    };
    use ark_poly::univariate::DensePolynomial;

    type E = ark_bls12_381::Bls12_381;
    type G1 = <E as Pairing>::G1;
    type G2 = <E as Pairing>::G2;
    type UniPoly381 = DensePolynomial<<E as Pairing>::ScalarField>;

    #[test]
    fn test_encryption() {
        let mut rng = ark_std::test_rng();
        let n = 8;
        let params = KZG10::<E, UniPoly381>::setup(n, &mut rng).unwrap();

        let mut sk: Vec<SecretKey<E>> = Vec::new();
        let mut pk: Vec<PublicKey<E>> = Vec::new();

        for i in 0..n {
            sk.push(SecretKey::<E>::new(&mut rng));
            pk.push(sk[i].get_pk(0, &params, n).unwrap())
        }

        let ak = AggregateKey::<E>::new(pk, &params);
        let ct = encrypt::<E>(&ak, 2, &params);

        let mut ct_bytes = Vec::new();
        ct.serialize_compressed(&mut ct_bytes).unwrap();
        println!("Compressed ciphertext: {} bytes", ct_bytes.len());

        let mut g1_bytes = Vec::new();
        let mut g2_bytes = Vec::new();
        let mut e_gh_bytes = Vec::new();

        let g = G1::generator();
        let h = G2::generator();

        g.serialize_compressed(&mut g1_bytes).unwrap();
        h.serialize_compressed(&mut g2_bytes).unwrap();
        ak.e_gh.serialize_compressed(&mut e_gh_bytes).unwrap();

        println!("G1 len: {} bytes", g1_bytes.len());
        println!("G2 len: {} bytes", g2_bytes.len());
        println!("GT len: {} bytes", e_gh_bytes.len());
    }
}
