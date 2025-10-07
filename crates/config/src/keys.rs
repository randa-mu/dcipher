use anyhow::anyhow;
use ark_ff::{BigInteger, PrimeField};
use base64::Engine;
use base64::prelude::BASE64_STANDARD;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt::Formatter;
use std::str::FromStr;

/// Wrapper around ark_*::Fr that allows deserialization from hex and base64
pub struct SecretKey<Fr>(pub Fr);

/// Wrapper around libp2p::identity::Keypair with (de)serialization & cmd line parsing.
#[derive(Clone, Debug)]
pub struct Libp2pKeyWrapper(pub ::libp2p::identity::Keypair);

impl<Fr: std::fmt::Debug> std::fmt::Debug for SecretKey<Fr> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}

impl<Fr: Clone> Clone for SecretKey<Fr> {
    fn clone(&self) -> Self {
        SecretKey(self.0.clone())
    }
}

impl<Fr: PrimeField> Serialize for SecretKey<Fr> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let bytes = self.0.into_bigint().to_bytes_be();
        serializer.serialize_str(&format!("0x{}", hex::encode(&bytes)))
    }
}

impl<'de, Fr: PrimeField> Deserialize<'de> for SecretKey<Fr> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        use serde::de::Error;

        let sk_str = String::deserialize(deserializer)?;
        try_decode_secret_key_hex_and_b64(&sk_str).map_err(Error::custom)
    }
}

impl<Fr: PrimeField> FromStr for SecretKey<Fr> {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        try_decode_secret_key_hex_and_b64(s)
    }
}

fn try_decode_secret_key_hex_and_b64<Fr: PrimeField>(
    sk_str: &str,
) -> anyhow::Result<SecretKey<Fr>> {
    // technicall base64 strings can start with 0x, so we check the length as well
    let sk_bytes = if &sk_str[0..2] == "0x" && sk_str.len() == 67 {
        hex::decode(&sk_str[2..]).map_err(|e| anyhow!("secret key wasn't valid hex: {}", e))
    } else {
        BASE64_STANDARD
            .decode(sk_str)
            .or_else(|_| hex::decode(&sk_str[2..]))
            .map_err(|e| anyhow!("secret key wasn't hex or base64: {}", e))
    }?;

    Ok(SecretKey(Fr::from_be_bytes_mod_order(&sk_bytes)))
}

pub type Bn254SecretKey = SecretKey<ark_bn254::Fr>;

impl<'de> Deserialize<'de> for Libp2pKeyWrapper {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        use serde::de::Error;

        let s = String::deserialize(deserializer)?;
        s.parse::<Libp2pKeyWrapper>().map_err(D::Error::custom)
    }
}

impl Serialize for Libp2pKeyWrapper {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.to_string().as_str())
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
