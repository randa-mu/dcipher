//! Declare dsigner's protobuf types & grpc client / server code.

pub mod dsigner {
    #![allow(clippy::enum_variant_names)]

    include!(concat!(env!("OUT_DIR"), "/dcipher.dsigner.v1.rs"));

    pub use application_args::Args as ApplicationArgsEnum;
    use std::collections::HashMap;

    impl From<ApplicationArgsEnum> for Application {
        fn from(value: application_args::Args) -> Self {
            value.application()
        }
    }

    impl ApplicationArgsEnum {
        pub fn application(&self) -> Application {
            match self {
                ApplicationArgsEnum::Blocklock(_) => Application::Blocklock,
                ApplicationArgsEnum::Randomness(_) => Application::Randomness,
                ApplicationArgsEnum::Any(_) => Application::Any,
            }
        }
    }

    #[derive(thiserror::Error, Debug)]
    #[non_exhaustive]
    pub enum ParseProtoError {
        #[error("unspecified field: {0}")]
        UnspecifiedField(&'static str),
    }

    impl TryFrom<Application> for dcipher_signer::dsigner::Application {
        type Error = ParseProtoError;

        fn try_from(value: Application) -> Result<Self, Self::Error> {
            match value {
                Application::Unspecified => Err(Self::Error::UnspecifiedField("application")),
                Application::Blocklock => Ok(Self::Blocklock),
                Application::Randomness => Ok(Self::Randomness),
                Application::Any => Ok(Self::Any),
            }
        }
    }

    impl From<dcipher_signer::dsigner::Application> for Application {
        fn from(value: dcipher_signer::dsigner::Application) -> Self {
            match value {
                dcipher_signer::dsigner::Application::Blocklock => Self::Blocklock,
                dcipher_signer::dsigner::Application::Randomness => Self::Randomness,
                dcipher_signer::dsigner::Application::Any => Self::Any,
                _ => Self::Unspecified,
            }
        }
    }

    impl From<ApplicationBlocklockArgs> for dcipher_signer::dsigner::ApplicationBlocklockArgs {
        fn from(value: ApplicationBlocklockArgs) -> Self {
            Self {
                chain_id: value.chain_id,
            }
        }
    }

    impl From<dcipher_signer::dsigner::ApplicationBlocklockArgs> for ApplicationBlocklockArgs {
        fn from(value: dcipher_signer::dsigner::ApplicationBlocklockArgs) -> Self {
            Self {
                chain_id: value.chain_id,
            }
        }
    }

    impl From<ApplicationRandomnessArgs> for dcipher_signer::dsigner::ApplicationRandomnessArgs {
        fn from(value: ApplicationRandomnessArgs) -> Self {
            Self {
                chain_id: value.chain_id,
            }
        }
    }

    impl From<dcipher_signer::dsigner::ApplicationRandomnessArgs> for ApplicationRandomnessArgs {
        fn from(value: dcipher_signer::dsigner::ApplicationRandomnessArgs) -> Self {
            Self {
                chain_id: value.chain_id,
            }
        }
    }

    impl From<ApplicationAnyArgs> for dcipher_signer::dsigner::ApplicationAnyArgs {
        fn from(value: ApplicationAnyArgs) -> Self {
            Self {
                dst_suffix: value.dst_suffix,
            }
        }
    }

    impl From<dcipher_signer::dsigner::ApplicationAnyArgs> for ApplicationAnyArgs {
        fn from(value: dcipher_signer::dsigner::ApplicationAnyArgs) -> Self {
            Self {
                dst_suffix: value.dst_suffix,
            }
        }
    }

    impl From<ApplicationArgsEnum> for dcipher_signer::dsigner::ApplicationArgs {
        fn from(value: ApplicationArgsEnum) -> Self {
            match value {
                ApplicationArgsEnum::Blocklock(args) => Self::Blocklock(args.into()),
                ApplicationArgsEnum::Randomness(args) => Self::Randomness(args.into()),
                ApplicationArgsEnum::Any(args) => Self::Any(args.into()),
            }
        }
    }

    impl TryFrom<ApplicationArgs> for dcipher_signer::dsigner::ApplicationArgs {
        type Error = ParseProtoError;

        fn try_from(value: ApplicationArgs) -> Result<Self, Self::Error> {
            let Some(args) = value.args else {
                Err(Self::Error::UnspecifiedField("args"))?
            };

            Ok(args.into())
        }
    }

    impl TryFrom<SignatureAlgorithm> for dcipher_signer::dsigner::SignatureAlgorithm {
        type Error = ParseProtoError;

        fn try_from(value: SignatureAlgorithm) -> Result<Self, Self::Error> {
            match value {
                SignatureAlgorithm::Unspecified => {
                    Err(Self::Error::UnspecifiedField("signature algorithm"))
                }

                SignatureAlgorithm::Unknown => {
                    Err(Self::Error::UnspecifiedField("signature algorithm"))
                }

                SignatureAlgorithm::Bn254SigOnG1Keccak256 => {
                    Ok(Self::Bls(dcipher_signer::dsigner::BlsSignatureAlgorithm {
                        curve: dcipher_signer::dsigner::BlsSignatureCurve::Bn254G1,
                        hash: dcipher_signer::dsigner::BlsSignatureHash::Keccak256,
                    }))
                }

                SignatureAlgorithm::Bn254SigOnG1Sha256 => {
                    Ok(Self::Bls(dcipher_signer::dsigner::BlsSignatureAlgorithm {
                        curve: dcipher_signer::dsigner::BlsSignatureCurve::Bn254G1,
                        hash: dcipher_signer::dsigner::BlsSignatureHash::Sha256,
                    }))
                }

                SignatureAlgorithm::Bls12381SigOnG1Sha256 => {
                    Ok(Self::Bls(dcipher_signer::dsigner::BlsSignatureAlgorithm {
                        curve: dcipher_signer::dsigner::BlsSignatureCurve::Bls12_381G1,
                        hash: dcipher_signer::dsigner::BlsSignatureHash::Sha256,
                    }))
                }

                SignatureAlgorithm::Bls12381SigOnG2Sha256 => {
                    Ok(Self::Bls(dcipher_signer::dsigner::BlsSignatureAlgorithm {
                        curve: dcipher_signer::dsigner::BlsSignatureCurve::Bls12_381G2,
                        hash: dcipher_signer::dsigner::BlsSignatureHash::Sha256,
                    }))
                }
            }
        }
    }

    impl From<dcipher_signer::dsigner::SignatureAlgorithm> for SignatureAlgorithm {
        fn from(value: dcipher_signer::dsigner::SignatureAlgorithm) -> Self {
            match value {
                dcipher_signer::dsigner::SignatureAlgorithm::Bls(
                    dcipher_signer::dsigner::BlsSignatureAlgorithm {
                        curve: dcipher_signer::dsigner::BlsSignatureCurve::Bn254G1,
                        hash: dcipher_signer::dsigner::BlsSignatureHash::Keccak256,
                    },
                ) => Self::Bn254SigOnG1Keccak256,

                dcipher_signer::dsigner::SignatureAlgorithm::Bls(
                    dcipher_signer::dsigner::BlsSignatureAlgorithm {
                        curve: dcipher_signer::dsigner::BlsSignatureCurve::Bn254G1,
                        hash: dcipher_signer::dsigner::BlsSignatureHash::Sha256,
                    },
                ) => Self::Bn254SigOnG1Sha256,

                dcipher_signer::dsigner::SignatureAlgorithm::Bls(
                    dcipher_signer::dsigner::BlsSignatureAlgorithm {
                        curve: dcipher_signer::dsigner::BlsSignatureCurve::Bls12_381G1,
                        hash: dcipher_signer::dsigner::BlsSignatureHash::Sha256,
                    },
                ) => Self::Bls12381SigOnG1Sha256,

                dcipher_signer::dsigner::SignatureAlgorithm::Bls(
                    dcipher_signer::dsigner::BlsSignatureAlgorithm {
                        curve: dcipher_signer::dsigner::BlsSignatureCurve::Bls12_381G2,
                        hash: dcipher_signer::dsigner::BlsSignatureHash::Sha256,
                    },
                ) => Self::Bls12381SigOnG2Sha256,

                _ => Self::Unknown,
            }
        }
    }

    impl From<dcipher_signer::dsigner::SchemeAlgorithm> for SchemeAlgorithm {
        fn from(value: dcipher_signer::dsigner::SchemeAlgorithm) -> Self {
            Self {
                public_key: value.public_key,
                algs: value
                    .algs
                    .into_iter()
                    .map(|alg| SignatureAlgorithm::from(alg).into())
                    .filter(|&alg| alg != SignatureAlgorithm::Unknown as i32) // filter unknown algs
                    .collect(),
                apps: value
                    .apps
                    .into_iter()
                    .map(|app| Application::from(app).into())
                    .collect(),
            }
        }
    }

    impl Scheme {
        pub fn new(scheme_id: String, value: dcipher_signer::dsigner::SchemeDetails) -> Self {
            Self {
                scheme_id,
                scheme_algs: value
                    .scheme_algs
                    .into_iter()
                    .map(|alg| alg.into())
                    .collect(),
                n: value.n.into(),
                t: value.t.into(),
                metadata: HashMap::default(),
            }
        }
    }
}

/// Re-export dsigner proto types
pub use dsigner::*;
