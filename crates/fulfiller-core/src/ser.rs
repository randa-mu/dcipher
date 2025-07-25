//! Helper traits used to serialize and deserialize various types into EVM bytes.

use ark_ec::AffineRepr;
use alloy::primitives::Bytes;

// Re-export from contracts-core to get the implementations
pub use contracts_core::ser::{EvmDeserialize, EvmSerialize};

// EvmSerialize implementation is now provided by contracts-core



#[cfg(test)]
pub(crate) mod tests {
    #[cfg(all(feature = "blocklock", feature = "ibe"))] // uses blocklock types & ibe
    pub(crate) mod bn254 {
        use super::super::*;
        use alloy::primitives::U256;
        use alloy::sol_types::SolValue;
        use ark_ff::{BigInteger, Fp, PrimeField};
        use contracts_core::blocklock::blocklock_sender::{TypesLib, BLS};
        use contracts_core::ibe_helper::{IbeCiphertext, IbeIdentityOnBn254G1Ciphertext};
        use contracts_core::ser::IbeIdentityOnBn254G1CiphertextError;

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
