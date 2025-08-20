//! Helper traits used to serialize and deserialize various types into EVM bytes.

use alloy::primitives::Bytes;
use ark_ec::AffineRepr;

/// Serialize into an EVM Bytes type.
pub trait EvmSerialize {
    fn ser_bytes(&self) -> Bytes;
}

/// Deserialize from an EVM Bytes type.
pub trait EvmDeserialize {
    type Error: std::error::Error + Send + Sync + 'static;

    fn deser(bytes: &Bytes) -> Result<Self, Self::Error>
    where
        Self: Sized;
}

impl EvmSerialize for ark_ec::short_weierstrass::Affine<ark_bn254::g1::Config> {
    fn ser_bytes(&self) -> Bytes {
        use ark_bn254::Fq;
        use ark_ff::{BigInteger, PrimeField, Zero};

        let (x, y) = match self.xy() {
            Some((x, y)) => (x, y),
            None => (&Fq::zero(), &Fq::zero()),
        };

        [x.into_bigint().to_bytes_be(), y.into_bigint().to_bytes_be()]
            .concat()
            .into()
    }
}

impl EvmSerialize for ark_ec::short_weierstrass::Affine<ark_bn254::g2::Config> {
    fn ser_bytes(&self) -> Bytes {
        use ark_ff::{BigInteger, PrimeField, Zero};

        let (x, y) = match self.xy() {
            Some((x, y)) => (x, y),
            None => (&Zero::zero(), &Zero::zero()),
        };

        [x.c1, x.c0, y.c1, y.c0]
            .map(|v| v.into_bigint().to_bytes_be())
            .concat()
            .into()
    }
}

impl EvmSerialize for ark_ec::short_weierstrass::Affine<ark_bls12_381::g1::Config> {
    fn ser_bytes(&self) -> Bytes {
        use ark_serialize::CanonicalSerialize;
        let mut buf = Vec::with_capacity(48);
        self.serialize_compressed(&mut buf)
            .expect("serialization should not fail");
        buf.into()
    }
}

#[cfg(feature = "blocklock")]
pub use blocklock::*;

#[cfg(feature = "blocklock")]
mod blocklock {
    use super::*;
    use crate::agents::blocklock::contracts::TypesLib;
    use crate::ibe_helper::IbeIdentityOnBn254G1Ciphertext;
    use alloy::sol_types::SolType;

    #[derive(thiserror::Error, Debug)]
    pub enum IbeIdentityOnBn254G1CiphertextError {
        #[error("abi decode error")]
        AbiDecode(#[from] alloy::sol_types::Error),

        #[error("invalid ephemeral pk")]
        InvalidEphemeralPk,
    }

    impl EvmDeserialize for IbeIdentityOnBn254G1Ciphertext {
        type Error = IbeIdentityOnBn254G1CiphertextError;

        fn deser(bytes: &Bytes) -> Result<Self, Self::Error> {
            use ark_ff::PrimeField;

            let ciphertext = TypesLib::Ciphertext::abi_decode(bytes)?;
            let x0: [u8; 32] = ciphertext.u.x[0].to_be_bytes();
            let x1: [u8; 32] = ciphertext.u.x[1].to_be_bytes();
            let y0: [u8; 32] = ciphertext.u.y[0].to_be_bytes();
            let y1: [u8; 32] = ciphertext.u.y[1].to_be_bytes();

            let x0 = ark_bn254::Fq::from_be_bytes_mod_order(&x0);
            let x1 = ark_bn254::Fq::from_be_bytes_mod_order(&x1);
            let y0 = ark_bn254::Fq::from_be_bytes_mod_order(&y0);
            let y1 = ark_bn254::Fq::from_be_bytes_mod_order(&y1);

            let x = ark_bn254::Fq2::new(x0, x1);
            let y = ark_bn254::Fq2::new(y0, y1);

            // Use unchecked to return results instead of panicking
            let eph_pk = ark_ec::short_weierstrass::Affine::new_unchecked(x, y);
            if !eph_pk.is_on_curve()
                || !eph_pk.is_in_correct_subgroup_assuming_on_curve()
                || eph_pk.is_zero()
            {
                Err(Self::Error::InvalidEphemeralPk)?
            }

            Ok(Self::new(eph_pk))
        }
    }
}

#[cfg(test)]
pub(crate) mod tests {
    #[cfg(all(feature = "blocklock", feature = "ibe"))] // uses blocklock types & ibe
    pub(crate) mod bn254 {
        use super::super::*;
        use crate::agents::blocklock::contracts::{BLS, TypesLib};
        use crate::ibe_helper::{IbeCiphertext, IbeIdentityOnBn254G1Ciphertext};
        use alloy::primitives::U256;
        use alloy::sol_types::SolValue;
        use ark_ff::{BigInteger, Fp, PrimeField};

        #[test]
        fn ark_bn254_g1_serialize() {
            let bytes_encoding = hex::decode("043864e59644fbf5c3c5360d584aef2d97d489184d90cc10c2bff113803650e9296aa6398a0793d763e7196aa34c2513887400698d2c2aa6d817920c1937a8af").unwrap();
            let p = ark_bn254::g1::G1Affine::new(
                Fp::from_be_bytes_mod_order(&bytes_encoding[0..32]),
                Fp::from_be_bytes_mod_order(&bytes_encoding[32..64]),
            );
            assert_eq!(EvmSerialize::ser_bytes(&p).as_ref(), bytes_encoding);

            let bytes_encoding = hex::decode("13283ef9a6033433f275974e17308058b9af6f2661ebb20e169b3ec20e696a5a06d7e95e2ac8bcbbf7ee22fb64c60e6572869d57cc636bbb517d686a0fea4ace").unwrap();
            let p = ark_bn254::g1::G1Affine::new(
                Fp::from_be_bytes_mod_order(&bytes_encoding[0..32]),
                Fp::from_be_bytes_mod_order(&bytes_encoding[32..64]),
            );
            assert_eq!(EvmSerialize::ser_bytes(&p).as_ref(), bytes_encoding);

            let bytes_encoding = hex::decode("00000000000000000000000000000000000000000000000000000000000000010000000000000000000000000000000000000000000000000000000000000002").unwrap();
            let p = ark_bn254::g1::G1Affine::new(
                Fp::from_be_bytes_mod_order(&bytes_encoding[0..32]),
                Fp::from_be_bytes_mod_order(&bytes_encoding[32..64]),
            );
            assert_eq!(EvmSerialize::ser_bytes(&p).as_ref(), bytes_encoding);
        }

        pub(crate) fn encode_ciphertext(x0: &[u8], x1: &[u8], y0: &[u8], y1: &[u8]) -> Bytes {
            let x0 = U256::from_be_bytes::<32>(x0.try_into().unwrap());
            let x1 = U256::from_be_bytes::<32>(x1.try_into().unwrap());
            let y0 = U256::from_be_bytes::<32>(y0.try_into().unwrap());
            let y1 = U256::from_be_bytes::<32>(y1.try_into().unwrap());

            let ciphertext = TypesLib::Ciphertext {
                u: BLS::PointG2 {
                    x: [x0, x1],
                    y: [y0, y1],
                },
                v: Bytes::from(vec![0; 32]),
                w: Bytes::from(vec![0; 4]),
            };

            Bytes::from(ciphertext.abi_encode())
        }

        #[test]
        fn ark_bn254_identity_g1_ciphertext_deserialize() {
            // Eph pk in the correct subgroup
            let x0_bytes =
                hex::decode("053c8b871fb2beb16c6dd8505b72606b5b41f02327f5258ccc705bff58ba5e62")
                    .unwrap();
            let x1_bytes =
                hex::decode("26f30f9423464a04164a502f389c120f816f6a2f356d0de7950bd305a4387bc4")
                    .unwrap();
            let y0_bytes =
                hex::decode("244ba4b679b276dbf383b8bb789aeed7eeb2b53ceac229f3ed046d0dd0f2fee5")
                    .unwrap();
            let y1_bytes =
                hex::decode("078dceb0ae82b7172e0fdace28e1fbd30c7eb9fed1fdc3b33a577eeee2fb383d")
                    .unwrap();
            let bytes = encode_ciphertext(&x0_bytes, &x1_bytes, &y0_bytes, &y1_bytes);

            let ibe_ct: IbeIdentityOnBn254G1Ciphertext = EvmDeserialize::deser(&bytes).unwrap();
            let eph_pk = ibe_ct.ephemeral_pk();
            let (x, y) = eph_pk.xy().unwrap();
            assert_eq!(x.c0.into_bigint().to_bytes_be(), x0_bytes);
            assert_eq!(x.c1.into_bigint().to_bytes_be(), x1_bytes);
            assert_eq!(y.c0.into_bigint().to_bytes_be(), y0_bytes);
            assert_eq!(y.c1.into_bigint().to_bytes_be(), y1_bytes);

            // Eph pk not in the subgroup
            let x0_bytes =
                hex::decode("06509868c2ba31177d3270794d7841f3e766660dc40ced0a97605e17400f6623")
                    .unwrap();
            let x1_bytes =
                hex::decode("138bfb37eb88c9d84d593464934b5adc773da3d8365798e2ef27021f3c4df689")
                    .unwrap();
            let y0_bytes =
                hex::decode("29822a33cb9495185e7513b72b8db622a6e46ffa5c8f43b3f472bf7c0379c528")
                    .unwrap();
            let y1_bytes =
                hex::decode("11ef0178ec001657d3681b0956386733b666c281fe150c302fa3bce7f26e8580")
                    .unwrap();
            let bytes = encode_ciphertext(&x0_bytes, &x1_bytes, &y0_bytes, &y1_bytes);

            let ibe_ct: Result<IbeIdentityOnBn254G1Ciphertext, _> = EvmDeserialize::deser(&bytes);
            matches!(
                ibe_ct,
                Err(IbeIdentityOnBn254G1CiphertextError::InvalidEphemeralPk)
            );

            // Eph pk point at infinity
            let x0_bytes =
                hex::decode("0000000000000000000000000000000000000000000000000000000000000000")
                    .unwrap();
            let x1_bytes =
                hex::decode("0000000000000000000000000000000000000000000000000000000000000000")
                    .unwrap();
            let y0_bytes =
                hex::decode("0000000000000000000000000000000000000000000000000000000000000000")
                    .unwrap();
            let y1_bytes =
                hex::decode("0000000000000000000000000000000000000000000000000000000000000000")
                    .unwrap();
            let bytes = encode_ciphertext(&x0_bytes, &x1_bytes, &y0_bytes, &y1_bytes);

            let ibe_ct: Result<IbeIdentityOnBn254G1Ciphertext, _> = EvmDeserialize::deser(&bytes);
            matches!(
                ibe_ct,
                Err(IbeIdentityOnBn254G1CiphertextError::InvalidEphemeralPk)
            );
        }
    }
}
