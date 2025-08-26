use anyhow::anyhow;
use ark_ff::{BigInteger, PrimeField};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt::Formatter;
use std::str::FromStr;

/// Wrapper around ark_bn254::Fr that allows deserialization from hex
pub struct Bn254SecretKey(ark_bn254::Fr);

/// Wrapper around libp2p::identity::Keypair with (de)serialization & cmd line parsing.
#[derive(Clone, Debug)]
pub struct Libp2pKeyWrapper(::libp2p::identity::Keypair);

impl std::fmt::Debug for Bn254SecretKey {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}

impl Clone for Bn254SecretKey {
    fn clone(&self) -> Self {
        Bn254SecretKey(self.0)
    }
}

impl Serialize for Bn254SecretKey {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let bytes = self.0.into_bigint().to_bytes_be();
        serializer.serialize_str(&format!("0x{}", hex::encode(&bytes)))
    }
}

impl<'de> Deserialize<'de> for Bn254SecretKey {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        use serde::de::Error;

        let hex_str = String::deserialize(deserializer)?;
        if &hex_str[0..2] != "0x" {
            Err(D::Error::custom("invalid hex string"))?
        }

        let bytes = hex::decode(&hex_str).map_err(D::Error::custom)?;
        Ok(Bn254SecretKey(ark_bn254::Fr::from_be_bytes_mod_order(
            &bytes,
        )))
    }
}

impl FromStr for Bn254SecretKey {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use ark_ff::PrimeField;

        if &s[0..2] != "0x" {
            Err(anyhow!("invalid hex string"))?
        }

        let bytes = hex::decode(&s[2..])?;
        let s = ark_bn254::Fr::from_be_bytes_mod_order(&bytes);
        Ok(Bn254SecretKey(s))
    }
}

impl From<Bn254SecretKey> for ark_bn254::Fr {
    fn from(value: Bn254SecretKey) -> Self {
        value.0
    }
}

impl FromStr for Libp2pKeyWrapper {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use base64::prelude::*;

        let bytes = BASE64_STANDARD.decode(s)?;
        Ok(Libp2pKeyWrapper(
            ::libp2p::identity::Keypair::from_protobuf_encoding(&bytes)?,
        ))
    }
}

impl std::fmt::Display for Libp2pKeyWrapper {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        use base64::prelude::*;
        let bytes = self.0.to_protobuf_encoding().expect("failed to encode key");
        let encoded = BASE64_STANDARD.encode(&bytes);
        f.write_str(&encoded)
    }
}

impl From<Libp2pKeyWrapper> for ::libp2p::identity::Keypair {
    fn from(value: Libp2pKeyWrapper) -> Self {
        value.0
    }
}

pub mod serde_to_string_from_str {
    use serde::{Deserialize, Deserializer, Serializer};
    use std::str::FromStr;

    pub fn serialize<S, T>(p: &T, s: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
        T: ToString,
    {
        s.serialize_str(&p.to_string())
    }

    pub fn deserialize<'de, D, T>(deserializer: D) -> Result<T, D::Error>
    where
        D: Deserializer<'de>,
        T: FromStr,
        T::Err: std::fmt::Display,
    {
        use serde::de::Error;

        let level = String::deserialize(deserializer)?;
        T::from_str(&level).map_err(D::Error::custom)
    }
}
