//! This module provides a randomness trait that allows to obtain per-component randomness.

use crate::helpers::SessionId;
use rand::{CryptoRng, RngCore};
use std::fmt::Formatter;

/// Helper trait to combine various Rng traits.
pub trait AdkgRngInstance: RngCore + CryptoRng + Send + Sync + 'static {}

/// List of components requiring randomness.
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum AdkgRngType {
    AcssSecret,
    Acss(SessionId),
    Aba(SessionId),
    KeyPok,
}

/// Trait to obtain per-component randomness.
pub trait AdkgRng: Send {
    type RngInstance: AdkgRngInstance;
    type Error: std::error::Error + Send + Sync + 'static;

    fn get(&mut self, rng_type: AdkgRngType) -> Result<Self::RngInstance, Self::Error>;
}

/// Deterministic randomness.
pub use deterministic_rand::DeterministicRand;

/// Per-component randomness derived from a single source of randomness.
pub struct AdkgStdRng<RNG> {
    rng: RNG,
}

impl<RNG> AdkgStdRng<RNG> {
    pub fn new(rng: RNG) -> Self {
        Self { rng }
    }
}

/// Blanket implementation of AdkgRngInstance for supported types.
impl<RNG: RngCore + CryptoRng + Send + Sync + 'static> AdkgRngInstance for RNG {}

impl<RNG> AdkgRng for AdkgStdRng<RNG>
where
    RNG: AdkgRngInstance,
{
    type RngInstance = rand::rngs::StdRng;
    type Error = rand::Error;

    fn get(&mut self, _rng_type: AdkgRngType) -> Result<Self::RngInstance, Self::Error> {
        use rand::SeedableRng;
        Self::RngInstance::from_rng(&mut self.rng)
    }
}

impl std::fmt::Display for AdkgRngType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            AdkgRngType::AcssSecret => write!(f, "AcssSecret"),
            AdkgRngType::Acss(sid) => write!(f, "Acss({sid})"),
            AdkgRngType::Aba(sid) => write!(f, "Aba({sid})"),
            AdkgRngType::KeyPok => write!(f, "KeyPok"),
        }
    }
}

mod deterministic_rand {
    use crate::rand::{AdkgRng, AdkgRngType};

    use digest::Digest;
    use rand::SeedableRng;
    use rand_chacha::ChaCha20Rng;
    use std::convert::Infallible;

    /// Deterministic randomness based on a seed.
    pub struct DeterministicRand {
        seed: [u8; 32],
    }

    impl DeterministicRand {
        pub fn new(seed: [u8; 32]) -> Self {
            Self { seed }
        }
    }

    impl AdkgRng for DeterministicRand {
        type RngInstance = ChaCha20Rng;
        type Error = Infallible;

        fn get(&mut self, rng_type: AdkgRngType) -> Result<Self::RngInstance, Self::Error> {
            // todo: use markers and panic if called twice

            // Generate the seed deterministically as Sha3(rng_type || seed)
            let seed: [u8; 32] = sha3::Sha3_256::default()
                .chain_update(rng_type.to_string().as_bytes())
                .chain_update(self.seed)
                .finalize()
                .into();

            Ok(ChaCha20Rng::from_seed(seed))
        }
    }
}

#[cfg(test)]
pub(crate) use test_rand::*;

#[cfg(test)]
mod test_rand {
    use crate::helpers::PartyId;
    use crate::rand::DeterministicRand;
    use digest::Digest;

    pub fn get_rng(id: PartyId, seed: &[u8]) -> DeterministicRand {
        let seed = get_party_seed(id, seed);
        DeterministicRand::new(seed)
    }

    fn get_party_seed(id: PartyId, seed: &[u8]) -> [u8; 32] {
        sha3::Sha3_256::default()
            .chain_update(seed)
            .chain_update(u64::from(id).to_be_bytes())
            .finalize()
            .into()
    }
}
