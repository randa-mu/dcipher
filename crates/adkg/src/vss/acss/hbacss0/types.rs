//! Various public and private types used by hbacss0
use crate::helpers::PartyId;
use crate::nizk::NizkError;
use crate::pke::ec_hybrid_chacha20poly1305::{
    EphemeralMultiHybridCiphertext, HybridEncryptionError,
};
use crate::vss::acss::hbacss0::{FeldPublicPoly, Hbacss0Output, PedPublicPoly};
use crate::vss::pedersen::PedersenPartyShare;
use ark_ec::CurveGroup;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use thiserror::Error;
use tokio::sync::oneshot;
use utils::serialize::fq::FqDeserialize;
use utils::serialize::fq::FqSerialize;
use utils::serialize::{
    SerializationError,
    point::{PointDeserializeCompressed, PointSerializeCompressed},
};

/// Message sent throughout the ACSS protocol.
#[serde_with::serde_as]
#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(tag = "acss_message")]
pub enum AcssMessage {
    Ok,
    Ready,
    Implicate(ImplicateMessage),
    ShareRecovery(ShareRecoveryMessage),
}

#[serde_with::serde_as]
#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct ShareRecoveryMessage {
    pub v: Vec<u8>,
}

/// Message used to implicate the dealer upon receiving an invalid share.
#[serde_with::serde_as]
#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct ImplicateMessage {
    pub pi: Vec<u8>, // serialized dleq proof
    pub k: Vec<u8>,  // serialized shared key between the dealer and the receiving party
}

/// Various errors returned by the ACSS protocol.
#[derive(Error, Debug)]
#[error("hbacss0 error")]
pub enum AcssError {
    #[error("failed to serialize bson: `{1}`")]
    BsonSer(#[source] bson::ser::Error, &'static str),

    #[error("failed to deserialize bson: `{1}`")]
    BsonDe(#[source] bson::de::Error, &'static str),

    #[error("failed to (de)serialize: `{1}`")]
    Ser(#[source] SerializationError, &'static str),

    #[error("bad output obtained from rbc")]
    InconsistentRbc,

    #[error("rbc failed")]
    FailedRbc(#[source] Box<dyn std::error::Error + Send + Sync>),

    #[error("hybrid encryption: {1}")]
    HybridEncryption(#[source] HybridEncryptionError, &'static str),

    #[error("nizk: {1}")]
    Nizk(#[source] NizkError, &'static str),

    #[error("no more messages in stream")]
    NoMessages,

    #[error("failed to send message using transport")]
    Transport,

    #[error("failed to initialize transport")]
    TransportInit,
}

/// Status of the node taking part in the ACSS protocol.
#[derive(Clone)]
pub(super) enum AcssStatus<CG: CurveGroup> {
    /// ACSS has just started.
    New,

    /// An invalid share was received, enter share recovery mode.
    ShareRecovery,

    /// A valid share was received, waiting for 2t + 1 oks.
    WaitingForOks(PartyShares<CG::ScalarField>),

    /// Enough ok / readys were received, waiting for 2t + 1 readys.
    WaitingForReadys(PartyShares<CG::ScalarField>),

    /// A share was recovered, about to exit.
    Complete,
}

/// Message broadcasted by the dealer through the RBC protocol.
#[derive(Serialize, Deserialize)]
#[serde(bound(
    serialize = "CG: PointSerializeCompressed",
    deserialize = "CG: PointDeserializeCompressed"
))]
pub(super) struct AcssBroadcastMessage<CG: CurveGroup> {
    pub(super) enc_shares: EphemeralMultiHybridCiphertext<CG>,
    pub(super) feld_public_poly: FeldPublicPoly<CG>,
    pub(super) ped_public_polys: Vec<PedPublicPoly<CG>>,
}

/// Shares obtained by the ACSS
#[derive(Clone, Serialize, Deserialize)]
#[serde(bound(serialize = "F: FqSerialize", deserialize = "F: FqDeserialize"))]
pub(super) struct PartyShares<F> {
    #[serde(with = "utils::serialize::fq::base64_or_bytes")]
    pub(super) feld_share: F,
    pub(super) ped_shares: Vec<PedersenPartyShare<F>>,
}

/// State machine used by handlers to update the state of the node.
pub(super) struct StateMachine<CG: CurveGroup> {
    pub(super) status: AcssStatus<CG>,

    // could be replaced by a bitmap
    pub(super) nodes_oks: HashMap<PartyId, bool>, // count the number of parties that are OK
    pub(super) nodes_readys: HashMap<PartyId, bool>, // count the number of parties that are ready
    pub(super) shares_recovery: HashMap<PartyId, PartyShares<CG::ScalarField>>, // store the parties currently recovering

    pub(super) output: Option<oneshot::Sender<Hbacss0Output<CG>>>, // require an option since we move the sender upon sending
}

/// Predicate for Feldman's VSS verification.
pub(super) struct PedVerifyPredicate<CG: CurveGroup> {
    pub(super) expected_broadcaster: PartyId,
    pub(super) i: PartyId,
    pub(super) sk: CG::ScalarField,
    pub(super) pk: CG,
    pub(super) g: CG,
    pub(super) h: CG,
}
