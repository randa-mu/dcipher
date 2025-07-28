//! Serialization module for arkworks elliptic curve points.
//! The points are encoded differently based on the curve. Currently, we rely on point compression
//! to encode Bn254 (aka alt_bn128) points. The most significant bit is used to
//! store the sign of the y-coordinate, while the second most significant bit specifies if the
//! point is at infinity. The rest of the bits encode the x coordinate in big endian.
//! For extension fields, we encode the element x = x_0 + x_1 i + ... + x_m i^m, as
//! to_be_bytes(x) = to_be_bytes(x_0) || to_be_bytes(x_1) || ... || to_be_bytes(x_m)

/// Module for serde serialization in Base64 format.
pub mod base64;

use super::SerializationError;
use ::base64::prelude::*;

pub trait PointSerializeCompressed {
    fn ser(&self) -> Result<Vec<u8>, SerializationError>;

    fn ser_base64(&self) -> Result<String, SerializationError> {
        Ok(BASE64_STANDARD.encode(&self.ser()?))
    }
}

pub trait PointDeserializeCompressed: Sized {
    fn deser(v: &[u8]) -> Result<Self, SerializationError>;

    fn deser_base64(base64_str: &str) -> Result<Self, SerializationError>
    where
        Self: std::marker::Sized,
    {
        let buf = BASE64_STANDARD.decode(base64_str)?;
        Self::deser(&buf)
    }
}

#[cfg(any(feature = "bn254", feature = "bls12-381"))]
mod fq_mod_8_geq_2 {
    use crate::serialize::{
        fq::{FqDeserialize, FqSerialize},
        SerializationError,
    };
    use ark_ec::{
        short_weierstrass::{Affine, SWCurveConfig},
        AffineRepr, CurveGroup,
    };

    macro_rules! gen_ser_compressed_fq_mod_8_geq_2 {
        ($config:ty, $size:expr) => {
            impl PointSerializeCompressed for Affine<$config> {
                fn ser(&self) -> Result<Vec<u8>, SerializationError> {
                    ser_compressed_fq_mod_8_geq_2(self)
                }
            }

            impl PointDeserializeCompressed for Affine<$config> {
                fn deser(v: &[u8]) -> Result<Self, SerializationError> {
                    if v.len() != $size {
                        Err(SerializationError::InvalidData)?;
                    }
                    deser_compressed_fq_mod_8_geq_2(v)
                }
            }

            impl PointSerializeCompressed for Projective<$config> {
                fn ser(&self) -> Result<Vec<u8>, SerializationError> {
                    ser_compressed_fq_mod_8_geq_2(&self.into_affine())
                }
            }

            impl PointDeserializeCompressed for Projective<$config> {
                fn deser(v: &[u8]) -> Result<Self, SerializationError> {
                    if v.len() != $size {
                        Err(SerializationError::InvalidData)?;
                    }
                    Ok(deser_compressed_fq_mod_8_geq_2(v)?.into())
                }
            }
        };
    }

    #[cfg(feature = "bn254")]
    mod bn254 {
        use super::*;
        use crate::serialize::point::{PointDeserializeCompressed, PointSerializeCompressed};
        use ark_ec::short_weierstrass::Projective;

        gen_ser_compressed_fq_mod_8_geq_2!(ark_bn254::g1::Config, 32);
        gen_ser_compressed_fq_mod_8_geq_2!(ark_bn254::g2::Config, 64);
    }

    #[cfg(feature = "bls12-381")]
    mod bls12_381 {
        use super::*;
        use crate::serialize::point::{PointDeserializeCompressed, PointSerializeCompressed};
        use ark_ec::short_weierstrass::Projective;

        gen_ser_compressed_fq_mod_8_geq_2!(ark_bls12_381::g1::Config, 48);
        gen_ser_compressed_fq_mod_8_geq_2!(ark_bls12_381::g2::Config, 96);
    }

    /// Follow encoding used by gnark as described in
    /// <https://pkg.go.dev/github.com/consensys/gnark-crypto/ecc/bn254#G1Affine.Bytes>:
    /// Bytes returns binary representation of p will store X coordinate in regular form
    /// and a parity bit as we have less than 3 bits available in our coordinate, we can't
    /// follow BLS12-381 style encoding (ZCash/IETF)
    //
    /// we use the 2 most significant bits instead
    ///
    /// 00 -> uncompressed
    /// 10 -> compressed, use smallest lexicographically square root of Y^2
    /// 11 -> compressed, use largest lexicographically square root of Y^2
    /// 01 -> compressed infinity point
    /// the "uncompressed infinity point" will just have 00 (uncompressed) followed by zeroes (infinity = 0,0 in affine coordinates)
    fn ser_compressed_fq_mod_8_geq_2<P: SWCurveConfig>(
        p: &Affine<P>,
    ) -> Result<Vec<u8>, SerializationError>
    where
        P::BaseField: FqSerialize,
    {
        let mut compressed = p.x.ser()?;

        let sgn = if p.y <= -p.y { 0u8 } else { 1u8 };
        let infinity = p.infinity;

        compressed[0] |= match (sgn, infinity) {
            (sgn, false) => 0b1000_0000 | (sgn << 6),
            (_, true) => 0b0100_0000,
        };

        Ok(compressed)
    }

    fn deser_compressed_fq_mod_8_geq_2<P: SWCurveConfig>(
        v: &[u8],
    ) -> Result<Affine<P>, SerializationError>
    where
        P::BaseField: FqDeserialize,
    {
        let mut v = v.to_vec();
        // Get flags and clear them
        let flags = (v[0] & 0b1100_0000) >> 6;
        v[0] &= 0b0011_1111; // clear negative flag

        let x = match flags {
            0b00 => {
                return Err(SerializationError::InvalidData);
            }

            0b01 => {
                return Ok(Affine::<P>::zero());
            }

            0b10 | 0b11 => P::BaseField::deser(&v),

            _ => {
                unreachable!("flags cannot have more than two bits")
            }
        }?;

        let greatest = flags != 0b10;
        let p = Affine::<P>::get_point_from_x_unchecked(x, greatest)
            .ok_or(SerializationError::InvalidData)?;

        if !p.is_on_curve() || !p.is_in_correct_subgroup_assuming_on_curve() {
            Err(SerializationError::InvalidData)?
        }

        Ok(p)
    }
}

#[cfg(test)]
mod tests {
    #[cfg(feature = "bn254")]
    mod bn254 {
        use ark_bn254::{Fq, Fq2, G1Affine, G2Affine};
        use ark_ec::AffineRepr;
        use rstest::*;

        use crate::serialize::point::{PointDeserializeCompressed, PointSerializeCompressed};

        #[rstest]
        #[case(
            "9c9e1f5330818c9d31ea2d02f4955e3321aa5f1bce6f3b21b99ca5372e92c00b",
            "12944137793487050959627179588085833320475525222197941925023463052110376124427",
            "10896862716660394070052883842210345168713014336897759569278661617046880975193"
        )]
        #[case(
            "e1d9245e8428d3aef9383ee782ba5b9e9fc2d35cf49c9e04ff63be732aaa7ab8",
            "15309980827326189096372412007135093182148252769146434563926332199955988118200",
            "17733777285276601967339819109490954626156771176129905955385745773408824118676"
        )]
        #[case(
            "813b2ba0ebd14f73ef98f18f9041d0fd3f9076d3a56507bbadace60de6a53084",
            "556857938924063633242045507904598738483495996641991671786405702788102566020",
            "7389825774715502185727598399949473093767793420888466591324311168644012002688"
        )]
        #[case(
            "d8acb209cafe5185e194630970729d617c82a5595a149c924c74c636e728fff3",
            "11160634836007227609163789219499220637455127904022933779512951002155467735027",
            "17798307832203411809679026901019109144031960856419954122399237666630579138481"
        )]
        fn test_serialize_bn254_g1(#[case] hex_str: &str, #[case] x: Fq, #[case] y: Fq) {
            let v = hex::decode(hex_str).unwrap();
            let expected = G1Affine::new(x, y);
            let p = G1Affine::deser(&v).unwrap();

            assert_eq!(p, expected);
            assert_eq!(expected.ser().unwrap(), v);
        }

        #[rstest]
        #[case("4000000000000000000000000000000000000000000000000000000000000000")]
        fn test_serialize_infinity_bn254_g1(#[case] hex_str: &str) {
            let v = hex::decode(hex_str).unwrap();
            let p = G1Affine::deser(&v).unwrap();

            assert_eq!(p, G1Affine::zero());
            assert_eq!(G1Affine::zero().ser().unwrap(), v)
        }

        #[rstest]
        #[case(
            "87955ee545779de3a794323f3ca18e8311e1c426f39816c0e223b66bd60df8a71ab970e27d13e9cbae7e20f2f11eb9fff5adb982dea77592ca28f11c95e332a9",
            "12087779871859942818179561499990990866232131271289553902776998034058959336105",
            "3430105098034984678208529119488129298885448669263266856204855646739772602535",
            "12638169004508722521172690989536255594206161897807081111061651335252985743169",
            "4974868872360495613672227158124944531008329814127946855645320423789124577757"
        )]
        #[case(
            "dc8f35bb6bd7cfc1b5e9175220739d8ce53ff9d98e36b35ede8d785d5979098025868661f5ad20a0e171ae0d9a045f132fd88473c30f69db1cf194863c849fa2",
            "16973260379259188481918951301875644428589061424216379660981749314523173789602",
            "12917789736018369421241726792794996907143697826710937623123301999942183750016",
            "21582600514836770342690326862476609385845053554281530293407241856283573461114",
            "20722674693500884297385529423651402496299430974327121615770182163893076975194"
        )]
        #[case(
            "8948b632d4173225d85392cdf124f3ca4fa0f7092c9b0c81ada68d230e3b93951b390dbc04e1c9d721e148e6a57b15662f3c20010b8626a0f04faef13be6ba62",
            "12313251986127175237493569723355982465721078730388322107962414508563021347426",
            "4199286114081638476629224730125318530312450673393522305649829462779174294421",
            "2479264883597552037732570039652437236386523067742688696050859745222910300131",
            "7674260511872300795762029847018319692445618760418006508359511600754908847401"
        )]
        #[case(
            "cb94d80bb95657b9f9ea7f873fbda55762ba031c17261ccef6ba47d259b4987f0c130333baad20aee01203eebe88cc49b37ba83fdde14b84b1e3ff476406792a",
            "5461346377085613201299051086494169110022245823609568235382153970563376642346",
            "5238425793291744530987097184619223135761534100679150826563610746910273738879",
            "11557577163568669010125254637570166599636070232393092756132866429568465157989",
            "13300676979591475709621787778141956427136111027723232875706919849700457338851"
        )]
        fn test_serialize_bn254_g2(
            #[case] hex_str: &str,
            #[case] x0: Fq,
            #[case] x1: Fq,
            #[case] y0: Fq,
            #[case] y1: Fq,
        ) {
            let v = hex::decode(hex_str).unwrap();

            let expected = G2Affine::new(Fq2::new(x0, x1), Fq2::new(y0, y1));
            let p = G2Affine::deser(&v).unwrap();

            assert_eq!(p, expected);
            assert_eq!(expected.ser().unwrap(), v);
        }

        #[rstest]
        #[case("40000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000")]
        fn test_serialize_infinity_bn254_g2(#[case] hex_str: &str) {
            let v = hex::decode(hex_str).unwrap();
            let p = G2Affine::deser(&v).unwrap();

            assert_eq!(p, G2Affine::zero());
            assert_eq!(G2Affine::zero().ser().unwrap(), v)
        }
    }
}
