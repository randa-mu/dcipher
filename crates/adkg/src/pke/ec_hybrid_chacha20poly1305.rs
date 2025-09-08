//! Hybrid encryption scheme to encrypt messages towards a public key.
//! Encryption and authentication done using ChaCha20+Poly1305.

use ark_ec::CurveGroup;
use ark_std::UniformRand;
use chacha20poly1305::{
    ChaCha20Poly1305, Key, Nonce,
    aead::{Aead, AeadCore, KeyInit},
};
use hkdf::Hkdf;
use rand::{CryptoRng, RngCore};
use serde::{Deserialize, Serialize};
use sha3::Sha3_256;
use thiserror::Error;
use utils::serialize::point::{PointDeserializeCompressed, PointSerializeCompressed};

pub const NONCE_LENGTH: usize = 12;
// todo: dynamically build DST using curve name when refactoring DSTs across codebase
const KDF_DST: &[u8] = b"EC_HYBRID-v1_CHACHA20POLY1305_HKDF_SHA3-256";

#[derive(Error, Debug)]
#[error("opaque ec hybrid encryption error")]
pub struct HybridEncryptionError;

#[serde_with::serde_as]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Ciphertext(#[serde_as(as = "utils::Base64OrBytes")] Vec<u8>);

/// A ciphertext that uses an ephemeral key.
#[serde_with::serde_as]
#[derive(Serialize, Deserialize)]
#[serde(bound(
    serialize = "CG: PointSerializeCompressed",
    deserialize = "CG: PointDeserializeCompressed"
))]
pub struct EphemeralHybridCiphertext<CG: CurveGroup> {
    #[serde(with = "utils::serialize::point::base64")]
    pub sender_pk: CG,
    pub inner: HybridCiphertext,
}

/// A ciphertext that uses a pre-shared key.
#[serde_with::serde_as]
#[derive(Serialize, Deserialize)]
pub struct HybridCiphertext {
    pub ct: Ciphertext,
    #[serde_as(as = "utils::Base64OrBytes")]
    pub nonce: [u8; NONCE_LENGTH],
}

/// A multi hybrid ciphertext that relies on an ephemeral key.
#[serde_with::serde_as]
#[derive(Serialize, Deserialize)]
#[serde(bound(
    serialize = "CG: PointSerializeCompressed",
    deserialize = "CG: PointDeserializeCompressed"
))]
pub struct EphemeralMultiHybridCiphertext<CG: CurveGroup> {
    #[serde(with = "utils::serialize::point::base64")]
    pub sender_pk: CG,
    pub inner: MultiHybridCiphertext,
}

/// A multi ciphertext that relies on a pre-shared key.
#[serde_with::serde_as]
#[derive(Serialize, Deserialize)]
pub struct MultiHybridCiphertext {
    #[serde_as(as = "utils::Base64OrBytes")]
    pub nonce: [u8; NONCE_LENGTH],
    pub cts: Vec<Ciphertext>,
}

/// Encrypt a message towards a specific public key by generating an ephemeral key pair.
pub fn encrypt<CG, RNG>(
    m: &[u8],
    recipient_pk: &CG,
    g: &CG,
    rng: &mut RNG,
) -> Result<EphemeralHybridCiphertext<CG>, HybridEncryptionError>
where
    CG: CurveGroup + PointSerializeCompressed,
    RNG: RngCore + CryptoRng,
{
    let sender_sk = CG::ScalarField::rand(rng);
    let sender_pk = *g * sender_sk;

    let inner = encrypt_with_sk(&sender_sk, &sender_pk, m, recipient_pk, rng)?;
    Ok(EphemeralHybridCiphertext { sender_pk, inner })
}

/// Encrypt a message towards a specific public key with a specific secret key.
pub fn encrypt_with_sk<CG, RNG>(
    sender_sk: &CG::ScalarField,
    sender_pk: &CG,
    m: &[u8],
    recipient_pk: &CG,
    rng: &mut RNG,
) -> Result<HybridCiphertext, HybridEncryptionError>
where
    CG: CurveGroup + PointSerializeCompressed,
    RNG: RngCore + CryptoRng,
{
    let nonce: Nonce = ChaCha20Poly1305::generate_nonce(rng);

    encrypt_internal(sender_sk, sender_pk, &nonce, m, recipient_pk)
}

/// Encrypt multiple messages towards multiple recipients while re-using the same nonce to slightly
/// optimize the ciphertext length.
pub fn encrypt_multi<CG, RNG>(
    ms: &[Vec<u8>],
    recipients_pks: &[CG],
    g: &CG,
    rng: &mut RNG,
) -> Result<EphemeralMultiHybridCiphertext<CG>, HybridEncryptionError>
where
    CG: CurveGroup + PointSerializeCompressed,
    RNG: RngCore + CryptoRng,
{
    // Generate random ephemeral keypair (sk_i, PK_i = [sk_i]G) for the multi encryption
    let sender_sk = CG::ScalarField::rand(rng);
    let sender_pk = *g * sender_sk;

    let inner = encrypt_multi_static(&sender_sk, sender_pk, ms, recipients_pks, rng)?;
    Ok(EphemeralMultiHybridCiphertext { sender_pk, inner })
}

/// Encrypt multiple messages towards multiple recipients with a static key while re-using the same
/// nonce to slightly optimize the ciphertext length.
pub fn encrypt_multi_static<CG, RNG>(
    sender_sk: &CG::ScalarField,
    sender_pk: CG,
    ms: &[Vec<u8>],
    recipients_pks: &[CG],
    rng: &mut RNG,
) -> Result<MultiHybridCiphertext, HybridEncryptionError>
where
    CG: CurveGroup + PointSerializeCompressed,
    RNG: RngCore + CryptoRng,
{
    // Generate random nonce common to all parties
    let nonce: Nonce = ChaCha20Poly1305::generate_nonce(rng);

    // Encrypt the message towards each public key
    let cts = recipients_pks
        .iter()
        .zip(ms)
        .map(|(pk, m)| Ok(encrypt_internal(sender_sk, &sender_pk, &nonce, m, pk)?.ct))
        .collect::<Result<Vec<_>, _>>()?;

    Ok(MultiHybridCiphertext {
        cts,
        nonce: nonce.into(),
    })
}

/// Encrypt a message towards a specific public key with a specific secret key.
fn encrypt_internal<CG>(
    sender_sk: &CG::ScalarField,
    sender_pk: &CG,
    nonce: &Nonce,
    m: &[u8],
    recipient_pk: &CG,
) -> Result<HybridCiphertext, HybridEncryptionError>
where
    CG: CurveGroup + PointSerializeCompressed,
{
    // Derive a shared 256 bits key k_ij = KDF([sk_i]PK_j)
    let p = *recipient_pk * sender_sk;
    let k: Key = derive_shared_sym_key(&p, sender_pk, recipient_pk)?.into();

    // Encrypt ciphertext
    let ct = sym_encrypt(&k, nonce, m)?.into();
    Ok(HybridCiphertext {
        ct,
        nonce: nonce.to_owned().into(),
    })
}

/// Derive a shared symmetric key from the shared key, the sender's ephemeral public key, and the
/// recipient public key.
fn derive_shared_sym_key<CG, const N: usize>(
    shared_key: &CG,
    sender_eph_pk: &CG,
    recipient_pk: &CG,
) -> Result<[u8; N], HybridEncryptionError>
where
    CG: CurveGroup + PointSerializeCompressed,
{
    let mut out_buf = [0; N];

    let shared_key = shared_key
        .ser_compressed()
        .map_err(|_| HybridEncryptionError)?;
    let sender_eph_pk = sender_eph_pk
        .ser_compressed()
        .map_err(|_| HybridEncryptionError)?;
    let recipient_pk = recipient_pk
        .ser_compressed()
        .map_err(|_| HybridEncryptionError)?;

    // Derive a shared key using a DST as the salt, the shared key, and both the ephemeral pk and recipient pk
    // to prevent malleability attack
    Hkdf::<Sha3_256>::new(Some(KDF_DST), &shared_key)
        .expand_multi_info(&[&sender_eph_pk, &recipient_pk], &mut out_buf)
        .map_err(|_| HybridEncryptionError)?;

    Ok(out_buf)
}

fn sym_encrypt(k: &Key, nonce: &Nonce, m: &[u8]) -> Result<Vec<u8>, HybridEncryptionError> {
    let cipher = ChaCha20Poly1305::new(k);
    cipher.encrypt(nonce, m).map_err(|_| HybridEncryptionError)
}

fn sym_decrypt(k: &Key, nonce: &Nonce, ct: &[u8]) -> Result<Vec<u8>, HybridEncryptionError> {
    let cipher = ChaCha20Poly1305::new(k);
    cipher.decrypt(nonce, ct).map_err(|_| HybridEncryptionError)
}

impl<CG> EphemeralHybridCiphertext<CG>
where
    CG: CurveGroup + PointSerializeCompressed,
{
    /// Decrypt a hybrid ciphertext using the recipient's secret key.
    pub fn decrypt(
        &self,
        recipient_sk: &CG::ScalarField,
        recipient_pk: &CG,
    ) -> Result<Vec<u8>, HybridEncryptionError> {
        self.inner
            .decrypt(recipient_sk, recipient_pk, &self.sender_pk)
    }

    /// Decrypt a hybrid ciphertext using a pre-computed shared_key.
    pub fn decrypt_shared_key(
        &self,
        shared_key: &CG,
        recipient_pk: &CG,
    ) -> Result<Vec<u8>, HybridEncryptionError> {
        self.inner
            .decrypt_shared_key(shared_key, recipient_pk, &self.sender_pk)
    }
}

impl HybridCiphertext {
    /// Decrypt a hybrid ciphertext using the recipient's secret key.
    pub fn decrypt<CG>(
        &self,
        recipient_sk: &CG::ScalarField,
        recipient_pk: &CG,
        sender_pk: &CG,
    ) -> Result<Vec<u8>, HybridEncryptionError>
    where
        CG: CurveGroup + PointSerializeCompressed,
    {
        // Derive symmetric key
        let shared_key = *sender_pk * recipient_sk;
        self.decrypt_shared_key(&shared_key, recipient_pk, sender_pk)
    }

    /// Decrypt a hybrid ciphertext using a pre-computed shared_key.
    pub fn decrypt_shared_key<CG>(
        &self,
        shared_key: &CG,
        recipient_pk: &CG,
        sender_pk: &CG,
    ) -> Result<Vec<u8>, HybridEncryptionError>
    where
        CG: CurveGroup + PointSerializeCompressed,
    {
        // Derive symmetric key
        let k: Key = derive_shared_sym_key(shared_key, sender_pk, recipient_pk)?.into();

        // Decrypt ciphertext
        let nonce = Nonce::from_slice(&self.nonce);
        sym_decrypt(&k, nonce, self.ct.as_ref())
    }
}

impl<CG> EphemeralMultiHybridCiphertext<CG>
where
    CG: CurveGroup + PointSerializeCompressed,
{
    /// Derive a shared key using the recipient's secret key.
    pub fn derive_shared_key(&self, recipient_sk: &CG::ScalarField) -> CG {
        self.sender_pk * recipient_sk
    }

    /// Decrypt a hybrid ciphertext at index i.
    pub fn decrypt_one(
        &self,
        i: usize,
        recipient_sk: &CG::ScalarField,
        recipient_pk: &CG,
    ) -> Result<Vec<u8>, HybridEncryptionError> {
        // Derive shared key
        let shared_key = self.sender_pk * recipient_sk;
        self.decrypt_one_with_shared_key(i, &shared_key, recipient_pk)
    }

    /// Decrypt a hybrid ciphertext at index i using a pre-computed shared_key.
    pub fn decrypt_one_with_shared_key(
        &self,
        i: usize,
        shared_key: &CG,
        recipient_pk: &CG,
    ) -> Result<Vec<u8>, HybridEncryptionError> {
        let Some(ct) = self.inner.cts.get(i) else {
            Err(HybridEncryptionError)?
        };

        // Derive symmetric key
        let k: Key = derive_shared_sym_key(shared_key, &self.sender_pk, recipient_pk)?.into();

        // Decrypt ciphertext
        let nonce = Nonce::from_slice(&self.inner.nonce);
        sym_decrypt(&k, nonce, ct.as_ref())
    }
}

impl MultiHybridCiphertext {
    /// Decrypt a hybrid ciphertext at index i.
    pub fn decrypt_one<CG>(
        &self,
        i: usize,
        recipient_sk: &CG::ScalarField,
        recipient_pk: &CG,
        sender_pk: &CG,
    ) -> Result<Vec<u8>, HybridEncryptionError>
    where
        CG: CurveGroup + PointSerializeCompressed,
    {
        // Derive shared key
        let shared_key = *sender_pk * recipient_sk;
        self.decrypt_one_with_shared_key(i, &shared_key, recipient_pk, sender_pk)
    }

    /// Decrypt a hybrid ciphertext at index i using a pre-computed shared_key.
    pub fn decrypt_one_with_shared_key<CG>(
        &self,
        i: usize,
        shared_key: &CG,
        recipient_pk: &CG,
        sender_pk: &CG,
    ) -> Result<Vec<u8>, HybridEncryptionError>
    where
        CG: CurveGroup + PointSerializeCompressed,
    {
        let Some(ct) = self.cts.get(i) else {
            Err(HybridEncryptionError)?
        };

        // Derive symmetric key
        let k: Key = derive_shared_sym_key(shared_key, sender_pk, recipient_pk)?.into();

        // Decrypt ciphertext
        let nonce = Nonce::from_slice(&self.nonce);
        sym_decrypt(&k, nonce, ct.as_ref())
    }
}

impl From<Vec<u8>> for Ciphertext {
    fn from(val: Vec<u8>) -> Ciphertext {
        Ciphertext(val)
    }
}

impl AsRef<[u8]> for Ciphertext {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}
