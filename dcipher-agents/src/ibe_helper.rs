//! Definition of a ciphersuite for pairing-based IBE.
//! Currently, we only support a partial definition used to sign and generate decryption keys.
//!
//! A concrete implementation of the ciphersuite on the bn254 pairing-friendly curve w/ the identity
//! on G1 is provided through the [`IbeIdentityOnBn254G1Suite`] struct.

use ark_ec::pairing::Pairing;
use ark_ec::{AffineRepr, Group};

/// Partial cipher suite for the IBE scheme described in <https://eprint.iacr.org/2023/189>, Algorithms 1-2
pub trait PairingIbeCipherSuite {
    type IdentityGroup: AffineRepr;
    type PublicKeyGroup: AffineRepr;
    type TargetGroup: Group;

    type HashOutput: AsRef<[u8]>;
    type Ciphertext: IbeCiphertext<EphemeralPublicKey = Self::PublicKeyGroup>;

    /// Cryptographic hash function H_1: \{0, 1\}^\ast \rightarrow IdentityGroup
    fn h1(&self, identity: &[u8]) -> Self::IdentityGroup;

    /// Cryptographic hash function H_2: G_T \rightarrow \{0, 1\}^\ell
    fn h2(&self, identity: &Self::TargetGroup) -> Self::HashOutput;

    /// Pairing function e: IdentityGroup \times PublicKeyGroup \rightarrow G_T
    fn pairing(&self, a: Self::IdentityGroup, b: Self::PublicKeyGroup) -> Self::TargetGroup;

    /// Outputs true if the decryption key is valid under the specified message and public key.
    fn verify_decryption_key(
        &self,
        identity: &[u8],
        decryption_key: Self::IdentityGroup,
        public_key: Self::PublicKeyGroup,
    ) -> bool;

    /// Preprocess the IBE decryption key using a decryption key and a ciphertext's ephemeral public key.
    /// Given a decryption key \pi_\rho = \[sk\] ID_\rho and an ephemeral public key U, return the
    /// mask H_2(e(\pi_\rho, U))
    fn preprocess_decryption_key(
        &self,
        decryption_key: Self::IdentityGroup,
        ephemeral_pk: Self::PublicKeyGroup,
    ) -> Self::HashOutput {
        // Derive the shared key using the decryption key and the ciphertext's ephemeral public key
        let shared_key = self.pairing(decryption_key, ephemeral_pk);

        // Return the mask H_2(e(\pi_\rho, U))
        self.h2(&shared_key)
    }
}

/// Signer for an IBE scheme as described in <https://eprint.iacr.org/2023/189>, Algorithms 1-2
/// that can be used to obtain decryption keys on an identity.
pub trait PairingIbeSigner: PairingIbeCipherSuite {
    /// Obtain a decryption key using a secret key and an identity.
    fn decryption_key(&self, identity: Self::IdentityGroup) -> Self::IdentityGroup;
}

/// Ciphertext output through IBE.
pub trait IbeCiphertext {
    type EphemeralPublicKey: AffineRepr;

    fn ephemeral_pk(&self) -> Self::EphemeralPublicKey;
}

pub use bn254::IbeIdentityOnBn254G1Ciphertext;
pub use bn254::IbeIdentityOnBn254G1Suite;

mod bn254 {
    use super::*;
    use crate::ibe_helper::expander::Expander;
    use ark_ec::CurveGroup;
    use ark_ec::pairing::PairingOutput;
    use ark_ff::{BigInteger, Field, PrimeField};
    use ark_std::Zero;
    use digest::core_api::BlockSizeUser;
    use std::ops::Neg;
    use utils::hash_to_curve::CustomPairingHashToCurve;

    /// Cipher suite for IBE w/ identity on bn254 G1.
    #[derive(Clone, Debug)]
    pub struct IbeIdentityOnBn254G1Suite<S = ()> {
        h1_dst: Vec<u8>,
        h2_dst: Vec<u8>,
        sk: S,
    }

    impl IbeIdentityOnBn254G1Suite {
        pub fn new(dst_prefix: &[u8], chain_id: u64) -> Self {
            let suffix = format!("0x{chain_id:064x}").as_bytes().to_vec();
            Self {
                h1_dst: Self::h1_dst(dst_prefix, &suffix),
                h2_dst: Self::h2_dst(dst_prefix, &suffix),
                sk: (),
            }
        }

        pub fn new_no_suffix(dst_prefix: &[u8]) -> Self {
            Self {
                h1_dst: Self::h1_dst(dst_prefix, b""),
                h2_dst: Self::h2_dst(dst_prefix, b""),
                sk: (),
            }
        }

        fn h1_dst(prefix: &[u8], suffix: &[u8]) -> Vec<u8> {
            let mut dst = vec![];
            if !prefix.is_empty() {
                dst.extend(prefix);
                dst.push(b'_');
            }

            dst.extend(b"BN254G1_XMD:KECCAK-256_SVDW_RO_H1_");

            if !suffix.is_empty() {
                dst.extend(suffix);
                dst.push(b'_');
            }
            dst
        }

        fn h2_dst(prefix: &[u8], suffix: &[u8]) -> Vec<u8> {
            let mut dst = vec![];
            if !prefix.is_empty() {
                dst.extend(prefix);
                dst.push(b'_');
            }

            dst.extend(b"BN254_XMD:KECCAK-256_H2_");

            if !suffix.is_empty() {
                dst.extend(suffix);
                dst.push(b'_');
            }
            dst
        }

        pub fn new_signer<S>(
            dst_prefix: &[u8],
            chain_id: u64,
            sk: S,
        ) -> IbeIdentityOnBn254G1Suite<S>
        where
            ark_bn254::G1Affine: std::ops::Mul<S, Output = ark_bn254::G1Projective>
                + for<'a> std::ops::Mul<&'a S, Output = ark_bn254::G1Projective>,
        {
            let suffix = format!("0x{chain_id:064x}").as_bytes().to_vec();
            IbeIdentityOnBn254G1Suite {
                h1_dst: Self::h1_dst(dst_prefix, &suffix),
                h2_dst: Self::h2_dst(dst_prefix, &suffix),
                sk,
            }
        }
    }

    /// Ciphertext for IBE w/ identity on bn254 G1.
    /// Currently, it only contains an ephemeral public key.
    pub struct IbeIdentityOnBn254G1Ciphertext {
        eph_pk: ark_bn254::G2Affine,
    }

    impl IbeIdentityOnBn254G1Ciphertext {
        pub fn new(eph_pk: ark_bn254::G2Affine) -> Self {
            Self { eph_pk }
        }
    }

    impl<S> PairingIbeCipherSuite for IbeIdentityOnBn254G1Suite<S> {
        type IdentityGroup = <ark_bn254::Bn254 as Pairing>::G1Affine;
        type PublicKeyGroup = <ark_bn254::Bn254 as Pairing>::G2Affine;
        type TargetGroup = PairingOutput<ark_bn254::Bn254>;
        type HashOutput = [u8; 32];
        type Ciphertext = IbeIdentityOnBn254G1Ciphertext;

        fn h1(&self, identity: &[u8]) -> Self::IdentityGroup {
            ark_bn254::Bn254::hash_to_g1_custom::<sha3::Keccak256>(identity, self.h1_dst.as_ref())
                .into_affine()
        }

        fn h2(&self, identity: &Self::TargetGroup) -> Self::HashOutput {
            // encode m as BE(m.c0.c0.c0) || BE(m.c0.c0.c1) || BE(m.c0.c1.c0) || ...
            let m: Vec<_> = identity
                .0
                .to_base_prime_field_elements()
                .flat_map(|ci| ci.into_bigint().to_bytes_be())
                .collect();

            let xmd = expander::ExpanderXmd {
                hasher: sha3::Keccak256::default(),
                dst: self.h2_dst.to_owned(),
                block_size: sha3::Keccak256::block_size(),
            };
            let h = xmd.expand(&m, 32);
            h.try_into()
                .expect("ExpanderXmd returned a number of bytes != 32")
        }

        fn pairing(&self, a: Self::IdentityGroup, b: Self::PublicKeyGroup) -> Self::TargetGroup {
            ark_bn254::Bn254::pairing(a, b)
        }

        fn verify_decryption_key(
            &self,
            identity: &[u8],
            decryption_key: Self::IdentityGroup,
            public_key: Self::PublicKeyGroup,
        ) -> bool {
            if !decryption_key.is_on_curve()
                || !decryption_key.is_in_correct_subgroup_assuming_on_curve()
                || decryption_key.is_zero()
            {
                return false;
            }

            let m = self.h1(identity);
            ark_bn254::Bn254::multi_pairing(
                [m.neg(), decryption_key],
                [public_key, Self::PublicKeyGroup::generator()],
            )
            .is_zero()
        }
    }

    impl<S> PairingIbeSigner for IbeIdentityOnBn254G1Suite<S>
    where
        S: Clone,
        ark_bn254::G1Affine: std::ops::Mul<S, Output = ark_bn254::G1Projective>
            + for<'a> std::ops::Mul<&'a S, Output = ark_bn254::G1Projective>,
    {
        fn decryption_key(&self, identity: Self::IdentityGroup) -> Self::IdentityGroup {
            let ibe_key = identity * self.sk.clone();
            ibe_key.into()
        }
    }

    impl IbeCiphertext for IbeIdentityOnBn254G1Ciphertext {
        type EphemeralPublicKey = ark_bn254::G2Affine;

        fn ephemeral_pk(&self) -> Self::EphemeralPublicKey {
            self.eph_pk
        }
    }

    #[cfg(feature = "signer")]
    mod signer {
        use crate::ibe_helper::{
            IbeIdentityOnBn254G1Suite, PairingIbeCipherSuite, PairingIbeSigner,
        };
        use dcipher_signer::{BlsSigner, BlsVerifier};
        use std::convert::Infallible;

        /// Implementation of a BLS verifier [`IbeIdentityOnBn254G1Suite`].
        impl<S> BlsVerifier for IbeIdentityOnBn254G1Suite<S> {
            type SignatureGroup = <Self as PairingIbeCipherSuite>::IdentityGroup;
            type PublicKeyGroup = <Self as PairingIbeCipherSuite>::PublicKeyGroup;

            fn verify(
                &self,
                m: impl AsRef<[u8]>,
                signature: Self::SignatureGroup,
                public_key: Self::PublicKeyGroup,
            ) -> bool {
                self.verify_decryption_key(m.as_ref(), signature, public_key)
            }
        }

        /// Implementation of a BLS signer [`IbeIdentityOnBn254G1Suite`].
        impl<S> BlsSigner for IbeIdentityOnBn254G1Suite<S>
        where
            Self: BlsVerifier
                + PairingIbeSigner<IdentityGroup = <Self as BlsVerifier>::SignatureGroup>,
        {
            type Error = Infallible;

            fn sign(&self, m: impl AsRef<[u8]>) -> Result<Self::SignatureGroup, Self::Error> {
                let identity = self.h1(m.as_ref());
                Ok(self.decryption_key(identity))
            }
        }
    }
}

/// Stolen from ark because it's not public, this is so ugly
/// Source: <https://github.com/arkworks-rs/algebra/blob/57be20e56a142b059bca05653961f8a9ca4f54ae/ff/src/fields/field_hashers/expander/mod.rs#L1>
mod expander {
    #![allow(unused, clippy::manual_div_ceil)]
    // The below implementation is a rework of https://github.com/armfazh/h2c-rust-ref
    // With some optimisations

    use ark_std::vec::Vec;
    use digest::{DynDigest, ExtendableOutput, Update};
    pub trait Expander {
        fn construct_dst_prime(&self) -> Vec<u8>;
        fn expand(&self, msg: &[u8], length: usize) -> Vec<u8>;
    }
    const MAX_DST_LENGTH: usize = 255;

    const LONG_DST_PREFIX: [u8; 17] = [
        //'H', '2', 'C', '-', 'O', 'V', 'E', 'R', 'S', 'I', 'Z', 'E', '-', 'D', 'S', 'T', '-',
        0x48, 0x32, 0x43, 0x2d, 0x4f, 0x56, 0x45, 0x52, 0x53, 0x49, 0x5a, 0x45, 0x2d, 0x44, 0x53,
        0x54, 0x2d,
    ];

    pub(super) struct ExpanderXof<T: Update + Clone + ExtendableOutput> {
        pub(super) xofer: T,
        pub(super) dst: Vec<u8>,
        pub(super) k: usize,
    }

    impl<T: Update + Clone + ExtendableOutput> Expander for ExpanderXof<T> {
        fn construct_dst_prime(&self) -> Vec<u8> {
            let mut dst_prime = if self.dst.len() > MAX_DST_LENGTH {
                let mut xofer = self.xofer.clone();
                xofer.update(&LONG_DST_PREFIX.clone());
                xofer.update(&self.dst);
                xofer.finalize_boxed((2 * self.k + 7) >> 3).to_vec()
            } else {
                self.dst.clone()
            };
            dst_prime.push(dst_prime.len() as u8);
            dst_prime
        }
        fn expand(&self, msg: &[u8], n: usize) -> Vec<u8> {
            let dst_prime = self.construct_dst_prime();
            let lib_str = &[((n >> 8) & 0xFF) as u8, (n & 0xFF) as u8];

            let mut xofer = self.xofer.clone();
            xofer.update(msg);
            xofer.update(lib_str);
            xofer.update(&dst_prime);
            xofer.finalize_boxed(n).to_vec()
        }
    }

    pub(super) struct ExpanderXmd<T: DynDigest + Clone> {
        pub(super) hasher: T,
        pub(super) dst: Vec<u8>,
        pub(super) block_size: usize,
    }

    impl<T: DynDigest + Clone> Expander for ExpanderXmd<T> {
        fn construct_dst_prime(&self) -> Vec<u8> {
            let mut dst_prime = if self.dst.len() > MAX_DST_LENGTH {
                let mut hasher = self.hasher.clone();
                hasher.update(&LONG_DST_PREFIX);
                hasher.update(&self.dst);
                hasher.finalize_reset().to_vec()
            } else {
                self.dst.clone()
            };
            dst_prime.push(dst_prime.len() as u8);
            dst_prime
        }
        fn expand(&self, msg: &[u8], n: usize) -> Vec<u8> {
            let mut hasher = self.hasher.clone();
            // output size of the hash function, e.g. 32 bytes = 256 bits for sha2::Sha256
            let b_len = hasher.output_size();
            let ell = (n + (b_len - 1)) / b_len;
            assert!(
                ell <= 255,
                "The ratio of desired output to the output size of hash function is too large!"
            );

            let dst_prime = self.construct_dst_prime();
            let z_pad: Vec<u8> = vec![0; self.block_size];
            // // Represent `len_in_bytes` as a 2-byte array.
            // // As per I2OSP method outlined in https://tools.ietf.org/pdf/rfc8017.pdf,
            // // The program should abort if integer that we're trying to convert is too large.
            assert!(n < (1 << 16), "Length should be smaller than 2^16");
            let lib_str: [u8; 2] = (n as u16).to_be_bytes();

            hasher.update(&z_pad);
            hasher.update(msg);
            hasher.update(&lib_str);
            hasher.update(&[0u8]);
            hasher.update(&dst_prime);
            let b0 = hasher.finalize_reset();

            hasher.update(&b0);
            hasher.update(&[1u8]);
            hasher.update(&dst_prime);
            let mut bi = hasher.finalize_reset();

            let mut uniform_bytes: Vec<u8> = Vec::with_capacity(n);
            uniform_bytes.extend_from_slice(&bi);
            for i in 2..=ell {
                // update the hasher with xor of b_0 and b_i elements
                for (l, r) in b0.iter().zip(bi.iter()) {
                    hasher.update(&[*l ^ *r]);
                }
                hasher.update(&[i as u8]);
                hasher.update(&dst_prime);
                bi = hasher.finalize_reset();
                uniform_bytes.extend_from_slice(&bi);
            }
            uniform_bytes[0..n].to_vec()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod bn254 {
        use super::*;
        use ark_ff::MontFp;

        #[test]
        fn h1_compatibility_js() {
            let cs = IbeIdentityOnBn254G1Suite::new_no_suffix(b"TEST_IBE");
            let exp_g1 = ark_bn254::G1Affine::new(
                MontFp!(
                    "3653173467790182248506061396572709101962704209335577284294737943301013580835"
                ),
                MontFp!(
                    "2746942348379889347830045590181038295853386711647916449093173473614869629216"
                ),
            );
            assert_eq!(cs.h1(b""), exp_g1);

            let exp_g1 = ark_bn254::G1Affine::new(
                MontFp!(
                    "16321686657743529192052651493099263906314638256513471437877788171012494023490"
                ),
                MontFp!(
                    "1350849970859344403057974536687145189475558284863891842544885697009576643682"
                ),
            );
            assert_eq!(cs.h1(b"AAAA"), exp_g1);

            let exp_g1 = ark_bn254::G1Affine::new(
                MontFp!(
                    "8929120621272588982321893216115445711479984949242622726428064156435284450717"
                ),
                MontFp!(
                    "14990022920127397634122290672777445403200199610320581106924212874052592382108"
                ),
            );
            assert_eq!(cs.h1(b"UOOQHNXMOVXWJZYTFTJCVYZCIXBSPVQY"), exp_g1);
        }

        #[test]
        fn h2_compatibility_js() {
            let two = ark_bn254::Fr::from(2u64);
            let cs = IbeIdentityOnBn254G1Suite::new_no_suffix(b"TEST_IBE");
            let exp_h2: [u8; 32] =
                hex::decode(b"ad886214af94515c0d08269799f69ef80ccd8f6f63ccc40bfcd6517c5b62510c")
                    .unwrap()
                    .try_into()
                    .unwrap();
            let gt = ark_bn254::Bn254::pairing(
                ark_bn254::G1Affine::generator(),
                ark_bn254::G2Affine::generator(),
            );
            assert_eq!(cs.h2(&gt), exp_h2);

            let exp_h2: [u8; 32] =
                hex::decode(b"80a06d11d632a76edf7c3b2772f8c4d9d72095295315977620d224b363c3c49c")
                    .unwrap()
                    .try_into()
                    .unwrap();
            let gt = ark_bn254::Bn254::pairing(
                ark_bn254::G1Affine::generator() * two,
                ark_bn254::G2Affine::generator(),
            );
            assert_eq!(cs.h2(&gt), exp_h2);

            let exp_h2: [u8; 32] =
                hex::decode(b"39dc28417110a63f330a0dca9ff58bb936cfcb70407c875f5a114a56488112f5")
                    .unwrap()
                    .try_into()
                    .unwrap();
            let gt = ark_bn254::Bn254::pairing(
                ark_bn254::G1Affine::generator() * two,
                ark_bn254::G2Affine::generator() * two,
            );
            assert_eq!(cs.h2(&gt), exp_h2);
        }
    }
}
