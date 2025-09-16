//! Serialization module for field elements.
use super::SerializationError;
use crate::helpers::u32_to_usize;
use ::base64::prelude::*;
use ark_ff::{BigInteger, Field, Fp, FpConfig, PrimeField, QuadExtConfig, QuadExtField};
use itertools::Itertools;

/// Trait used to serialize field elements.
pub trait FqSerialize {
    fn ser(&self) -> Result<Vec<u8>, SerializationError>;

    fn ser_base64(&self) -> Result<String, SerializationError> {
        Ok(BASE64_STANDARD.encode(&self.ser()?))
    }
}

/// Trait used to deserialize field elements.
pub trait FqDeserialize: Sized {
    fn deser(v: &[u8]) -> Result<Self, SerializationError>;

    fn deser_base64(base64_str: &str) -> Result<Self, SerializationError>
    where
        Self: std::marker::Sized,
    {
        let buf = BASE64_STANDARD.decode(base64_str)?;
        Self::deser(&buf)
    }
}

/// Serialize for arkworks' field elements over GF(p).
impl<P: FpConfig<N>, const N: usize> FqSerialize for Fp<P, N> {
    fn ser(&self) -> Result<Vec<u8>, SerializationError> {
        Ok(self.into_bigint().to_bytes_be().to_vec())
    }
}

/// Serialize for arkworks' field elements over GF(p^2).
impl<P: QuadExtConfig> FqSerialize for QuadExtField<P>
where
    <Self as Field>::BasePrimeField: FqSerialize,
{
    fn ser(&self) -> Result<Vec<u8>, SerializationError> {
        let elems: Vec<_> = self.to_base_prime_field_elements().collect();
        elems
            .into_iter()
            .rev()
            .map(|xi| xi.ser())
            .flatten_ok()
            .collect::<Result<Vec<u8>, _>>()
    }
}

/// Deserialize for arkworks' field elements over GF(p).
impl<P: FpConfig<N>, const N: usize> FqDeserialize for Fp<P, N> {
    fn deser(v: &[u8]) -> Result<Self, SerializationError> {
        Ok(Self::from_be_bytes_mod_order(v))
    }
}

/// Deserialize for arkworks' field elements over GF(p^2).
impl<P: QuadExtConfig> FqDeserialize for QuadExtField<P>
where
    <Self as Field>::BasePrimeField: FqDeserialize,
{
    fn deser(v: &[u8]) -> Result<Self, SerializationError> {
        let n: usize = Self::extension_degree()
            .try_into() // extension_degree is always small
            .expect("failed to cast u64 to usize");
        let byte_size = u32_to_usize(<Self as Field>::BasePrimeField::MODULUS_BIT_SIZE).div_ceil(8);

        let x = (0..n)
            .map(|i| -> Result<_, SerializationError> {
                <Self as Field>::BasePrimeField::deser(
                    v.get(i * byte_size..(i + 1) * byte_size)
                        .ok_or(SerializationError::InvalidData)?,
                )
            })
            .rev()
            .collect::<Result<Vec<<Self as Field>::BasePrimeField>, _>>()?;

        Self::from_base_prime_field_elems(x).ok_or(SerializationError::InvalidData)
    }
}

/// Uses [`Base64OrBytes`](crate::Base64OrBytes) to serialize the field element
pub mod base64_or_bytes {
    use super::*;
    use crate::Base64OrBytes;
    use serde_with::{DeserializeAs, SerializeAs};

    pub fn serialize<S, A: FqSerialize>(p: &A, s: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::Error;

        Base64OrBytes::serialize_as(&p.ser().map_err(S::Error::custom)?, s)
    }

    pub fn deserialize<'de, D, A: FqDeserialize>(deserializer: D) -> Result<A, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use serde::de::Error;

        let bytes: Vec<u8> = Base64OrBytes::deserialize_as(deserializer)?;
        A::deser(&bytes).map_err(D::Error::custom)
    }
}

pub mod base64 {
    use super::{FqDeserialize, FqSerialize};
    use serde::{Deserialize, Deserializer, Serializer};

    pub fn serialize<S, A: FqSerialize>(p: &A, s: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        use serde::ser::Error;

        s.serialize_str(&p.ser_base64().map_err(S::Error::custom)?)
    }

    pub fn deserialize<'de, D, A: FqDeserialize>(deserializer: D) -> Result<A, D::Error>
    where
        D: Deserializer<'de>,
    {
        use serde::de::Error;

        let base64_str = String::deserialize(deserializer)?;
        A::deser_base64(&base64_str).map_err(D::Error::custom)
    }
}

#[cfg(test)]
mod tests {
    #[cfg(feature = "bn254")]
    mod bn254 {
        use crate::serialize::fq::{FqDeserialize, FqSerialize};
        use ark_bn254::{Fq, Fq2};
        use rstest::*;

        #[rstest]
        #[case(
            "1c9e1f5330818c9d31ea2d02f4955e3321aa5f1bce6f3b21b99ca5372e92c00b",
            "12944137793487050959627179588085833320475525222197941925023463052110376124427"
        )]
        #[case(
            "21d9245e8428d3aef9383ee782ba5b9e9fc2d35cf49c9e04ff63be732aaa7ab8",
            "15309980827326189096372412007135093182148252769146434563926332199955988118200"
        )]
        fn test_serialize_bn254_fq(#[case] hex_str: &str, #[case] expected: Fq) {
            let v = hex::decode(hex_str).unwrap();
            let p = Fq::deser(&v).unwrap();

            assert_eq!(p, expected);
            assert_eq!(expected.ser().unwrap(), v);
        }

        #[rstest]
        #[case(
            "07955ee545779de3a794323f3ca18e8311e1c426f39816c0e223b66bd60df8a71ab970e27d13e9cbae7e20f2f11eb9fff5adb982dea77592ca28f11c95e332a9",
            "12087779871859942818179561499990990866232131271289553902776998034058959336105",
            "3430105098034984678208529119488129298885448669263266856204855646739772602535",
        )]
        #[case(
            "0efa709e17d7b9d49b493fdba054ae8b4a1fd45f74c31959a095cdabc0b39d540a1918c7fa0052bf4d7a17f2e6f4e982a576d718f46e05f8de0036f271a48bbb",
            "4567470695722039115366127728004712498590917199179872887825859934199912631227",
            "6774868904133666668991833646223747217875197213697982315887128643501882056020",
        )]
        fn test_serialize_bn254_fq2(#[case] hex_str: &str, #[case] x0: Fq, #[case] x1: Fq) {
            let v = hex::decode(hex_str).unwrap();

            let expected = Fq2::new(x0, x1);
            let p = Fq2::deser(&v).unwrap();

            assert_eq!(p, expected);
            assert_eq!(expected.ser().unwrap(), v);
        }
    }
}
