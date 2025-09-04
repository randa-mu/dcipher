//! Helper traits used to serialize and deserialize various types into EVM bytes.

#[cfg(feature = "blocklock")]
pub use blocklock::*;

#[cfg(feature = "blocklock")]
mod blocklock {
    use crate::agents::blocklock::contracts::TypesLib;
    use crate::ibe_helper::IbeIdentityOnBn254G1Ciphertext;
    use alloy::primitives::Bytes;
    use alloy::sol_types::SolType;
    use ark_ec::AffineRepr;

    #[derive(thiserror::Error, Debug)]
    pub enum IbeIdentityOnBn254G1CiphertextError {
        #[error("abi decode error")]
        AbiDecode(#[from] alloy::sol_types::Error),

        #[error("invalid ephemeral pk")]
        InvalidEphemeralPk,
    }

    impl IbeIdentityOnBn254G1Ciphertext {
        pub fn deser(bytes: &Bytes) -> Result<Self, IbeIdentityOnBn254G1CiphertextError> {
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
                Err(IbeIdentityOnBn254G1CiphertextError::InvalidEphemeralPk)?
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
        use alloy::primitives::{Bytes, U256};
        use alloy::sol_types::SolValue;
        use ark_ec::AffineRepr;
        use ark_ff::{BigInteger, PrimeField};

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

            let ibe_ct = IbeIdentityOnBn254G1Ciphertext::deser(&bytes).unwrap();
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

            let ibe_ct = IbeIdentityOnBn254G1Ciphertext::deser(&bytes);
            assert!(matches!(
                ibe_ct,
                Err(IbeIdentityOnBn254G1CiphertextError::InvalidEphemeralPk)
            ));

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

            let ibe_ct = IbeIdentityOnBn254G1Ciphertext::deser(&bytes);
            assert!(matches!(
                ibe_ct,
                Err(IbeIdentityOnBn254G1CiphertextError::InvalidEphemeralPk)
            ));
        }
    }
}
