//! Various enum and structures used to interact with a [`DSignerScheme`].

use bytes::Bytes;
use futures_util::future::BoxFuture;
use serde::{Deserialize, Serialize};

/// Enum for supported signature algorithms
#[derive(Clone, Copy, Hash, PartialEq, Eq, Debug, Serialize, Deserialize)]
#[non_exhaustive]
pub enum SignatureAlgorithm {
    #[cfg(feature = "bls")]
    Bls(BlsSignatureAlgorithm),

    #[non_exhaustive]
    PlaceHolder(),
}

/// Options for BLS schemes
#[cfg(feature = "bls")]
#[derive(Clone, Copy, Hash, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub struct BlsSignatureAlgorithm {
    pub curve: BlsSignatureCurve,
    pub hash: BlsSignatureHash,
}

/// Curves supported for BLS signatures
#[cfg(feature = "bls")]
#[derive(Clone, Copy, Hash, PartialEq, Eq, Debug, Serialize, Deserialize)]
#[non_exhaustive]
pub enum BlsSignatureCurve {
    #[cfg(feature = "bn254")]
    Bn254G1,
    #[cfg(feature = "bn254")]
    Bn254G2,

    #[cfg(feature = "bls12-381")]
    Bls12_381G1,
    #[cfg(feature = "bls12-381")]
    Bls12_381G2,
}

/// Hashes supported by BLS signatures
#[cfg(feature = "bls")]
#[derive(strum::VariantArray, Clone, Copy, Hash, PartialEq, Eq, Debug, Serialize, Deserialize)]
#[non_exhaustive]
pub enum BlsSignatureHash {
    #[cfg(feature = "sha2")]
    Sha256,
    #[cfg(feature = "sha3")]
    Keccak256,
}

/// Applications supported by dsigner
#[derive(strum::VariantArray, Clone, Copy, Hash, PartialEq, Eq, Debug, Serialize, Deserialize)]
#[non_exhaustive]
pub enum Application {
    Blocklock,
    Randomness,
    Any,
}

/// Application-specific arguments used during signing.
#[derive(Clone, Hash, PartialEq, Eq, Debug, Serialize, Deserialize)]
#[non_exhaustive]
pub enum ApplicationArgs {
    Blocklock(ApplicationBlocklockArgs),
    Randomness(ApplicationRandomnessArgs),
    Any(ApplicationAnyArgs),
}

/// Blocklock requires chain-specific signatures.
#[derive(Clone, Copy, Hash, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub struct ApplicationBlocklockArgs {
    pub chain_id: u64,
}

/// Randomness requires chain-specific signatures.
#[derive(Clone, Copy, Hash, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub struct ApplicationRandomnessArgs {
    pub chain_id: u64,
}

/// Allows domain separation for any application through the use of a customizable app-based dst suffix.
#[derive(Clone, Hash, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub struct ApplicationAnyArgs {
    pub dst_suffix: String,
}

/// Verification parameters required to verify a signature.
#[derive(Clone, Hash, PartialEq, Eq, Debug, Serialize, Deserialize)]
#[non_exhaustive]
pub struct VerificationParameters {
    pub public_key: Bytes,
    pub dst: Bytes,
}

/// A signature request composed of a message, a signature algorithm, and application-specific argument(s).
#[derive(Clone, Hash, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub struct SignatureRequest {
    pub m: Bytes,
    pub alg: SignatureAlgorithm,
    pub args: ApplicationArgs,
}

#[derive(thiserror::Error, Debug)]
pub enum DSignerSchemeError {
    #[error("the specified application is not supported by the signer")]
    ApplicationNotSupported,

    #[error("the specified algorithm is not supported by the signer")]
    AlgorithmNotSupported,

    #[error("other scheme error")]
    Other(#[source] Box<dyn std::error::Error + Send + Sync + 'static>),
}

#[derive(Clone, Hash, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub struct SchemeDetails {
    /// The algorithms supported by that scheme
    pub scheme_algs: Vec<SchemeAlgorithm>,
    /// Number of signers
    pub n: u16,
    /// Threshold required to obtain a signature
    pub t: u16,
}

#[derive(Clone, Hash, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub struct SchemeAlgorithm {
    /// The public key in a scheme-specific format
    pub public_key: Bytes,
    /// Signature algorithms supported by this scheme
    pub algs: Vec<SignatureAlgorithm>,
    /// Applications supported by this scheme
    pub apps: Vec<Application>,
}

/// A dyn-compatible trait that can be used to group various signers of a specific type (e.g., BLS, Frost, ...).
pub trait DSignerSchemeSigner {
    /// Obtain a byte-encoded signature for the associated signature request.
    fn async_sign(&self, req: SignatureRequest) -> BoxFuture<Result<Bytes, DSignerSchemeError>>;
}

pub trait DSignerScheme: DSignerSchemeSigner {
    /// Obtain the scheme details.
    fn details(&self) -> SchemeDetails;

    /// Output the verification parameters (e.g., public key, dst, hash) for a specific application
    /// arguments.
    fn verification_parameters(
        &self,
        alg: &SignatureAlgorithm,
        args: &ApplicationArgs,
    ) -> Result<VerificationParameters, DSignerSchemeError>;
}

impl ApplicationArgs {
    pub fn app(&self) -> Application {
        match self {
            ApplicationArgs::Blocklock(_) => Application::Blocklock,
            ApplicationArgs::Randomness(_) => Application::Randomness,
            ApplicationArgs::Any(_) => Application::Any,
        }
    }
}

#[cfg(feature = "bls")]
impl From<BlsSignatureCurve> for utils::dst::CurveId {
    fn from(value: BlsSignatureCurve) -> Self {
        match value {
            #[cfg(feature = "bn254")]
            BlsSignatureCurve::Bn254G1 => utils::dst::CurveId::Bn254G1,
            #[cfg(feature = "bn254")]
            BlsSignatureCurve::Bn254G2 => utils::dst::CurveId::Bn254G2,

            #[cfg(feature = "bls12-381")]
            BlsSignatureCurve::Bls12_381G1 => utils::dst::CurveId::Bls12_381G1,
            #[cfg(feature = "bls12-381")]
            BlsSignatureCurve::Bls12_381G2 => utils::dst::CurveId::Bls12_381G2,
        }
    }
}

#[cfg(feature = "bls")]
impl From<utils::dst::CurveId> for BlsSignatureCurve {
    fn from(value: utils::dst::CurveId) -> Self {
        match value {
            #[cfg(feature = "bn254")]
            utils::dst::CurveId::Bn254G1 => BlsSignatureCurve::Bn254G1,
            #[cfg(feature = "bn254")]
            utils::dst::CurveId::Bn254G2 => BlsSignatureCurve::Bn254G2,

            #[cfg(feature = "bls12-381")]
            utils::dst::CurveId::Bls12_381G1 => BlsSignatureCurve::Bls12_381G1,
            #[cfg(feature = "bls12-381")]
            utils::dst::CurveId::Bls12_381G2 => BlsSignatureCurve::Bls12_381G2,

            _ => panic!("unsupported curve"),
        }
    }
}

#[cfg(feature = "bls")]
impl From<BlsSignatureHash> for utils::dst::HashId {
    fn from(value: BlsSignatureHash) -> Self {
        match value {
            #[cfg(feature = "sha2")]
            BlsSignatureHash::Sha256 => utils::dst::HashId::Sha256,
            #[cfg(feature = "sha3")]
            BlsSignatureHash::Keccak256 => utils::dst::HashId::Keccak256,
        }
    }
}
