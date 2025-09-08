//! Filters supported applications and build rfc9380 DSTs.

use crate::dsigner::{Application, ApplicationArgs, BlsSignatureAlgorithm, BlsSignatureCurve};
use bytes::Bytes;
use std::collections::HashSet;
use strum::VariantArray;
use utils::dst::{CurveId, EncodingType, HashId, Rfc9380Dst, Rfc9380DstBuilder};

/// Supports all applications and builds rfc9380 DSTs
#[derive(Default, Clone, Debug)]
pub(super) struct BlsFilter {
    supported_apps: HashSet<Application>,
    supported_algs: HashSet<BlsSignatureAlgorithm>,
}

impl BlsFilter {
    #![allow(unused)]

    /// Create a filter that allows all applications
    pub(super) fn new(algs: impl IntoIterator<Item = BlsSignatureAlgorithm>) -> Self {
        Self {
            supported_apps: HashSet::from_iter(Application::VARIANTS.iter().copied()),
            supported_algs: HashSet::from_iter(algs),
        }
    }

    pub(super) fn filter_apps(&mut self, apps: impl IntoIterator<Item = Application>) {
        self.supported_apps = apps.into_iter().collect();
    }

    pub(super) fn is_supported(&self, app: &Application, alg: &BlsSignatureAlgorithm) -> bool {
        self.supported_algs.contains(alg) && self.supported_apps.contains(app)
    }

    pub(super) fn supported_apps(&self) -> impl Iterator<Item = Application> {
        self.supported_apps.iter().copied()
    }

    pub(super) fn supported_algs(
        &self,
        curve: &BlsSignatureCurve,
    ) -> impl Iterator<Item = BlsSignatureAlgorithm> {
        self.supported_algs
            .iter()
            .filter(|&a| a.curve.eq(curve))
            .copied()
    }

    /// Returns a dst if the application is supported, None otherwise
    pub(super) fn get_rfc9380_dst_if_supported(
        &self,
        app_args: &ApplicationArgs,
        alg: &BlsSignatureAlgorithm,
    ) -> Option<Bytes> {
        self.is_supported(&app_args.app(), alg)
            .then(|| get_rfc9380_application_dst(app_args, alg).0.into())
    }
}

pub(super) fn get_rfc9380_application_dst(
    app_args: &ApplicationArgs,
    alg: &BlsSignatureAlgorithm,
) -> Rfc9380Dst {
    let curve_id: CurveId = alg.curve.into();
    let hash_id: HashId = alg.hash.into();
    let builder = Rfc9380DstBuilder::empty()
        .with_curve_id(curve_id.clone())
        .with_hash_id(hash_id)
        .with_mapping(curve_id.default_mapping())
        .with_encoding(EncodingType::Uniform);

    let (app_name, app_name_sep, suffix) = match app_args {
        ApplicationArgs::Blocklock(args) => {
            // BLOCKLOCK_%curve_name%_%expand%:%hash_name%_%mapping%_%encoding%_H1_0x%chain_id%_
            (
                "BLOCKLOCK".to_owned(),
                b'_',
                format!("H1_0x{:064x}", args.chain_id),
            )
        }
        ApplicationArgs::Randomness(args) => {
            // dcipher-randomness-v01-%curve_name%:%expand%_%hash_name%_%mapping%_%encoding%_0x%chain_id%_
            (
                "dcipher-randomness-v01".to_owned(),
                b'-',
                format!("0x{:064x}", args.chain_id),
            )
        }
        ApplicationArgs::OnlySwapsVerifier(args) => {
            // swap-v1-BN254G1_XMD:KECCAK-256_SVDW_RO_0x0000000000000000000000000000000000000000000000000000000000014a34_
            (
                "swap-v1".to_owned(),
                b'-',
                format!("0x{:064x}", args.chain_id),
            )
        }
        ApplicationArgs::Any(args) => {
            // dcipher-anyapp-v01-%curve_name%_%expand%_%hash_name%_%mapping%_%encoding%_%custom_suffix%_
            (
                "dcipher-anyapp-v01".to_owned(),
                b'-',
                args.dst_suffix.clone(),
            )
        }
    };

    builder
        .with_application_name(app_name.into())
        .with_suffix(suffix.into())
        .build_with_app_name_sep(app_name_sep)
}
