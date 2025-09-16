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
    fn ser_compressed(&self) -> Result<Vec<u8>, SerializationError>;

    fn ser_compressed_base64(&self) -> Result<String, SerializationError> {
        Ok(BASE64_STANDARD.encode(&self.ser_compressed()?))
    }
}

pub trait PointDeserializeCompressed: Sized {
    fn deser_compressed(v: &[u8]) -> Result<Self, SerializationError>;

    fn deser_compressed_base64(base64_str: &str) -> Result<Self, SerializationError>
    where
        Self: std::marker::Sized,
    {
        let buf = BASE64_STANDARD.decode(base64_str)?;
        Self::deser_compressed(&buf)
    }
}

pub trait PointSerializeUncompressed {
    fn ser_uncompressed(&self) -> Result<Vec<u8>, SerializationError>;

    fn ser_uncompressed_base64(&self) -> Result<String, SerializationError> {
        Ok(BASE64_STANDARD.encode(&self.ser_uncompressed()?))
    }
}

pub trait PointDeserializeUncompressed: Sized {
    fn deser_uncompressed(v: &[u8]) -> Result<Self, SerializationError>;

    fn deser_uncompressed_base64(base64_str: &str) -> Result<Self, SerializationError>
    where
        Self: std::marker::Sized,
    {
        let buf = BASE64_STANDARD.decode(base64_str)?;
        Self::deser_uncompressed(&buf)
    }
}

#[cfg(feature = "bn254")]
mod fq_mod_8_eq_2 {
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
                fn ser_compressed(&self) -> Result<Vec<u8>, SerializationError> {
                    ser_compressed_fq_mod_8_geq_2(self)
                }
            }

            impl PointDeserializeCompressed for Affine<$config> {
                fn deser_compressed(v: &[u8]) -> Result<Self, SerializationError> {
                    if v.len() != $size {
                        Err(SerializationError::InvalidData)?;
                    }
                    deser_compressed_fq_mod_8_geq_2(v)
                }
            }

            impl PointSerializeCompressed for Projective<$config> {
                fn ser_compressed(&self) -> Result<Vec<u8>, SerializationError> {
                    self.into_affine().ser_compressed()
                }
            }

            impl PointDeserializeCompressed for Projective<$config> {
                fn deser_compressed(v: &[u8]) -> Result<Self, SerializationError> {
                    Affine::<$config>::deser_compressed(v).map(AffineRepr::into_group)
                }
            }
        };
    }

    #[cfg(feature = "bn254")]
    mod bn254 {
        use super::*;
        use crate::serialize::point::{
            PointDeserializeCompressed, PointDeserializeUncompressed, PointSerializeCompressed,
            PointSerializeUncompressed,
        };
        use ark_bn254::{Fq, Fq2};
        use ark_ec::short_weierstrass::Projective;
        use ark_ff::PrimeField;

        gen_ser_compressed_fq_mod_8_geq_2!(ark_bn254::g1::Config, 32);
        gen_ser_compressed_fq_mod_8_geq_2!(ark_bn254::g2::Config, 64);

        impl PointSerializeUncompressed for Affine<ark_bn254::g1::Config> {
            fn ser_uncompressed(&self) -> Result<Vec<u8>, SerializationError> {
                use ark_ff::{BigInteger, PrimeField, Zero};

                let (x, y) = match self.xy() {
                    Some((x, y)) => (x, y),
                    _ => (Zero::zero(), Zero::zero()),
                };

                Ok([x, y].map(|v| v.into_bigint().to_bytes_be()).concat())
            }
        }

        impl PointSerializeUncompressed for Affine<ark_bn254::g2::Config> {
            fn ser_uncompressed(&self) -> Result<Vec<u8>, SerializationError> {
                use ark_ff::{BigInteger, PrimeField, Zero};

                let (x, y) = match self.xy() {
                    Some((x, y)) => (x, y),
                    _ => (Zero::zero(), Zero::zero()),
                };

                Ok([x.c1, x.c0, y.c1, y.c0]
                    .map(|v| v.into_bigint().to_bytes_be())
                    .concat())
            }
        }

        impl PointDeserializeUncompressed for Affine<ark_bn254::g1::Config> {
            fn deser_uncompressed(v: &[u8]) -> Result<Self, SerializationError> {
                if v.len() != 64 {
                    Err(SerializationError::InvalidData)?;
                }

                let x = Fq::from_be_bytes_mod_order(&v[0..32]);
                let y = Fq::from_be_bytes_mod_order(&v[32..64]);

                Ok(ark_bn254::G1Affine::new(x, y))
            }
        }

        impl PointDeserializeUncompressed for Affine<ark_bn254::g2::Config> {
            fn deser_uncompressed(v: &[u8]) -> Result<Self, SerializationError> {
                if v.len() != 128 {
                    Err(SerializationError::InvalidData)?;
                }

                let x_c1 = Fq::from_be_bytes_mod_order(&v[0..32]);
                let x_c0 = Fq::from_be_bytes_mod_order(&v[32..64]);
                let y_c1 = Fq::from_be_bytes_mod_order(&v[64..96]);
                let y_c0 = Fq::from_be_bytes_mod_order(&v[96..128]);

                Ok(ark_bn254::G2Affine::new(
                    Fq2::new(x_c0, x_c1),
                    Fq2::new(y_c0, y_c1),
                ))
            }
        }
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

#[cfg(feature = "bls12-381")]
mod bls12_381 {
    use super::*;
    use crate::serialize::SerializationError;
    use ark_ec::{
        short_weierstrass::{Affine, Projective},
        AffineRepr, CurveGroup,
    };

    /// Follows encoding used by gnark / zcash as described in
    /// <https://pkg.go.dev/github.com/consensys/gnark-crypto/ecc/bls12-381#G1Affine.Bytes>:
    /// Bytes returns binary representation of p will store X coordinate in regular form and a parity bit
    /// we follow the BLS12-381 style encoding as specified in ZCash and now IETF.
    /// The most significant bit, when set, indicates that the point is in compressed form. Otherwise,
    /// the point is in uncompressed form.
    ///
    /// The second-most significant bit indicates that the point is at infinity. If this bit is set,
    /// the remaining bits of the group element's encoding should be set to zero.
    ///
    /// The third-most significant bit is set if (and only if) this point is in compressed form,
    /// and it is not the point at infinity and its y-coordinate is the lexicographically largest,
    /// of the two associated with the encoded x-coordinate.
    ///
    /// 000 -> uncompressed
    /// 010 -> point at infinity, remaining bits should be 0
    /// 110 -> compressed point at infinity, remaining bits should be 0
    /// 100 -> compressed, use smallest lexicographically square root of Y^2   8
    /// 101 -> compressed, use largest lexicographically square root of Y^2    A
    /// otherwise, invalid encoding
    macro_rules! gen_ser_compressed_ark {
        ($config:ty, $size:expr) => {
            impl PointSerializeCompressed for Affine<$config> {
                fn ser_compressed(&self) -> Result<Vec<u8>, SerializationError> {
                    let mut buf = Vec::with_capacity($size);
                    ark_serialize::CanonicalSerialize::serialize_compressed(self, &mut buf)?;
                    Ok(buf)
                }
            }

            impl PointDeserializeCompressed for Affine<$config> {
                fn deser_compressed(v: &[u8]) -> Result<Self, SerializationError> {
                    if v.len() != $size {
                        Err(SerializationError::InvalidData)?;
                    }
                    Ok(ark_serialize::CanonicalDeserialize::deserialize_compressed(
                        v,
                    )?)
                }
            }

            impl PointSerializeCompressed for Projective<$config> {
                fn ser_compressed(&self) -> Result<Vec<u8>, SerializationError> {
                    self.into_affine().ser_compressed()
                }
            }

            impl PointDeserializeCompressed for Projective<$config> {
                fn deser_compressed(v: &[u8]) -> Result<Self, SerializationError> {
                    Affine::<$config>::deser_compressed(v).map(AffineRepr::into_group)
                }
            }
        };
    }

    gen_ser_compressed_ark!(ark_bls12_381::g1::Config, 48);
    gen_ser_compressed_ark!(ark_bls12_381::g2::Config, 96);

    macro_rules! gen_ser_uncompressed_ark {
        ($config:ty, $size:expr) => {
            impl PointSerializeUncompressed for Affine<$config> {
                fn ser_uncompressed(&self) -> Result<Vec<u8>, SerializationError> {
                    let mut buf = Vec::with_capacity($size);
                    ark_serialize::CanonicalSerialize::serialize_uncompressed(self, &mut buf)?;
                    Ok(buf)
                }
            }

            impl PointDeserializeUncompressed for Affine<$config> {
                fn deser_uncompressed(v: &[u8]) -> Result<Self, SerializationError> {
                    if v.len() != $size {
                        Err(SerializationError::InvalidData)?;
                    }
                    Ok(ark_serialize::CanonicalDeserialize::deserialize_uncompressed(v)?)
                }
            }

            impl PointSerializeUncompressed for Projective<$config> {
                fn ser_uncompressed(&self) -> Result<Vec<u8>, SerializationError> {
                    self.into_affine().ser_uncompressed()
                }
            }

            impl PointDeserializeUncompressed for Projective<$config> {
                fn deser_uncompressed(v: &[u8]) -> Result<Self, SerializationError> {
                    Affine::<$config>::deser_uncompressed(v).map(AffineRepr::into_group)
                }
            }
        };
    }

    gen_ser_uncompressed_ark!(ark_bls12_381::g1::Config, 96);
    gen_ser_uncompressed_ark!(ark_bls12_381::g2::Config, 192);
}

#[cfg(test)]
mod tests {
    #[cfg(feature = "bn254")]
    mod bn254 {
        use ark_bn254::{Fq, Fq2, G1Affine, G2Affine};
        use ark_ec::AffineRepr;
        use ark_ff::{Fp, PrimeField};
        use rstest::*;

        use crate::serialize::point::{
            PointDeserializeCompressed, PointDeserializeUncompressed, PointSerializeCompressed,
            PointSerializeUncompressed,
        };

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
            let p = G1Affine::deser_compressed(&v).unwrap();

            assert_eq!(p, expected);
            assert_eq!(expected.ser_compressed().unwrap(), v);
        }

        #[rstest]
        #[case("4000000000000000000000000000000000000000000000000000000000000000")]
        fn test_serialize_infinity_bn254_g1(#[case] hex_str: &str) {
            let v = hex::decode(hex_str).unwrap();
            let p = G1Affine::deser_compressed(&v).unwrap();

            assert_eq!(p, G1Affine::zero());
            assert_eq!(G1Affine::zero().ser_compressed().unwrap(), v)
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
            let p = G2Affine::deser_compressed(&v).unwrap();

            assert_eq!(p, expected);
            assert_eq!(expected.ser_compressed().unwrap(), v);
        }

        #[rstest]
        #[case("40000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000")]
        fn test_serialize_infinity_bn254_g2(#[case] hex_str: &str) {
            let v = hex::decode(hex_str).unwrap();
            let p = G2Affine::deser_compressed(&v).unwrap();

            assert_eq!(p, G2Affine::zero());
            assert_eq!(G2Affine::zero().ser_compressed().unwrap(), v)
        }

        #[test]
        fn ark_serialize_bn254_g1_uncompressed() {
            let bytes_encoding = hex::decode("043864e59644fbf5c3c5360d584aef2d97d489184d90cc10c2bff113803650e9296aa6398a0793d763e7196aa34c2513887400698d2c2aa6d817920c1937a8af").unwrap();
            let p = ark_bn254::G1Affine::new(
                Fp::from_be_bytes_mod_order(&bytes_encoding[0..32]),
                Fp::from_be_bytes_mod_order(&bytes_encoding[32..64]),
            );
            assert_eq!(p.ser_uncompressed().unwrap(), bytes_encoding);
            assert_eq!(
                ark_bn254::G1Affine::deser_uncompressed(&bytes_encoding).unwrap(),
                p
            );

            let bytes_encoding = hex::decode("13283ef9a6033433f275974e17308058b9af6f2661ebb20e169b3ec20e696a5a06d7e95e2ac8bcbbf7ee22fb64c60e6572869d57cc636bbb517d686a0fea4ace").unwrap();
            let p = ark_bn254::G1Affine::new(
                Fp::from_be_bytes_mod_order(&bytes_encoding[0..32]),
                Fp::from_be_bytes_mod_order(&bytes_encoding[32..64]),
            );
            assert_eq!(p.ser_uncompressed().unwrap(), bytes_encoding);
            assert_eq!(
                ark_bn254::G1Affine::deser_uncompressed(&bytes_encoding).unwrap(),
                p
            );

            let bytes_encoding = hex::decode("00000000000000000000000000000000000000000000000000000000000000010000000000000000000000000000000000000000000000000000000000000002").unwrap();
            let p = ark_bn254::G1Affine::new(
                Fp::from_be_bytes_mod_order(&bytes_encoding[0..32]),
                Fp::from_be_bytes_mod_order(&bytes_encoding[32..64]),
            );
            assert_eq!(p.ser_uncompressed().unwrap(), bytes_encoding);
            assert_eq!(
                ark_bn254::G1Affine::deser_uncompressed(&bytes_encoding).unwrap(),
                p
            );
        }
    }

    #[cfg(feature = "bls12-381")]
    mod bls12_381 {
        use ark_bls12_381::{Fq, Fq2, G1Affine, G2Affine};
        use ark_ec::AffineRepr;
        use rstest::*;

        use crate::serialize::point::{PointDeserializeCompressed, PointSerializeCompressed};

        #[rstest]
        #[case(
            "8d358c1941003f30799bb56ce77686cfc5f744d50967d990f8e43bfcbad4de079ea1d9862a4f0db8e61a9918b7825519",
            "2033077180564551536999477171607878401375230380827222711239573993459466789485584636120800018305671343721381160441113",
            "1587474023738694561354819370912689335694261845355829155176237881610923941956633300668824511568885714047469217942984"
        )]
        #[case(
            "b247c0cd2d9ad0805beef3c725e12ae9f77141e25d2ccf822c88d5986d2825cfc680d8ff05c1f5cd77fbeb397846ce2d",
            "2813593473147529468924457930338535075892410823639120822460362212229121473693775109282234793163001245198277534993965",
            "2943563905086780779266684424209847439717415490548202619608483395838570670320351661448481300195473963052641964450134"
        )]
        fn test_serialize_g1(#[case] hex_str: &str, #[case] x: Fq, #[case] y: Fq) {
            let v = hex::decode(hex_str).unwrap();
            let expected = G1Affine::new(x, y);
            let p = G1Affine::deser_compressed(&v).unwrap();

            assert_eq!(p, expected);
            assert_eq!(expected.ser_compressed().unwrap(), v);
        }

        #[rstest]
        #[case("c00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000")]
        fn test_serialize_infinity_g1(#[case] hex_str: &str) {
            let v = hex::decode(hex_str).unwrap();
            let p = G1Affine::deser_compressed(&v).unwrap();

            assert_eq!(p, G1Affine::zero());
            assert_eq!(G1Affine::zero().ser_compressed().unwrap(), v)
        }

        #[rstest]
        #[case(
            "8c47e5a78d7e1e2fb20e59bf30db05407b870635e03cae47c844b9c45ffe62d7f7f53ea00fba5076d086d89ec8aff0360eb219e132531d7cda8f65c8ebc2783506b18c5a5b9b63d71028f833882c4fbc48fdaf593b28450995a6a4d3a0b4dab7",
            "2261876381793915445191073560184852490163293543330235380116663006798893947271160560394499202076550239541785714285239",
            "1890195503819537729457904927223157402583537418911035489895476953648633672956914081436153364050459436637325752266806",
            "1420853607630576647501206418676885404720511777042347885144987072293639199434895774099679668139610650409532008654307",
            "810064965271110693387389539734638654941234363675088634358727598018596316999507629952610167632787243279350416401641"
        )]
        #[case(
            "b0c94ad0a715a596cba6d0cc58763436165a99d11c07254dff0a7b41088d04bbbc634e5ea07e381e78eee5607e0e1a5308d2e64e4a5f5871d7a4e710f86fdcdd9160913a305a55c4d2edb94d607aabcc28a9603072350ab8a3cf5625979a76f2",
            "1358111225918059542086735828999449942817755076352805326854189913742923296244180701933655028097255667607002278426354",
            "2583647700743467243002944428165788429822194101978337975298221374661051679319908660122240883638917796757058414844499",
            "1911358698156181913061699570190045538257290156192558007272239989432652339469795113304507569780568442264305797982683",
            "2707734995167438465250618791316209525189120113387517169971923535335221527332454663125903630654634588036776572953212"
        )]
        fn test_serialize_g2(
            #[case] hex_str: &str,
            #[case] x0: Fq,
            #[case] x1: Fq,
            #[case] y0: Fq,
            #[case] y1: Fq,
        ) {
            let v = hex::decode(hex_str).unwrap();

            let expected = G2Affine::new(Fq2::new(x0, x1), Fq2::new(y0, y1));
            let p = G2Affine::deser_compressed(&v).unwrap();

            assert_eq!(p, expected);
            assert_eq!(expected.ser_compressed().unwrap(), v);
        }

        #[rstest]
        #[case("c00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000")]
        fn test_serialize_infinity_g2(#[case] hex_str: &str) {
            let v = hex::decode(hex_str).unwrap();
            let p = G2Affine::deser_compressed(&v).unwrap();

            assert_eq!(p, G2Affine::zero());
            assert_eq!(G2Affine::zero().ser_compressed().unwrap(), v)
        }
    }
}
