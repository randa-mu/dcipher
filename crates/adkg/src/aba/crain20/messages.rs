use super::ecdh_coin_toss::{Coin, EcdhCoinTossEval};
use crate::aba::Estimate;
use ark_ec::CurveGroup;
use serde::{Deserialize, Serialize};

/// Messages sent during the ABA protocol.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AbaMessage {
    Estimate(EstimateMessage),         // Estimate of the binary value.
    Auxiliary(AuxiliaryMessage),       // Auxiliary message containing an estimate.
    AuxiliarySet(AuxiliarySetMessage), // Auxiliary set message containing multiple estimates.
    CoinEval(CoinEvalMessage), // Message used when sending partial evaluations for the common coin.
}

/// Estimate for a specific round and stage.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct EstimateMessage {
    pub(crate) round: u8,
    pub(crate) stage: AuxStage,
    pub(crate) estimate: Estimate,
}

/// Representation of the two stages used within a round.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[repr(u8)]
pub enum AuxStage {
    Stage1,
    Stage2,
}

/// Auxiliary message used to provide an estimate for a specific round and stage.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct AuxiliaryMessage {
    pub(crate) round: u8,
    pub(crate) stage: AuxStage,
    pub(crate) estimate: Estimate,
}

/// Message used to send a set of estimates, i.e., a view.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct AuxiliarySetMessage {
    pub(crate) round: u8,
    pub(crate) view: View,
}

/// Set of all possible views during the ABA.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[repr(u8)]
pub enum View {
    Bot,
    Zero,
    One,

    BotZero,
    BotOne,
    ZeroOne,
}

/// Message to send a partial evaluation for the common coin tossing protocol.
#[serde_with::serde_as]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct CoinEvalMessage {
    pub(crate) round: u8,
    #[serde_as(as = "utils::Base64OrBytes")]
    pub(crate) eval: Vec<u8>,
}

/// Create CoinEvalMessage from an eval and a round number.
impl CoinEvalMessage {
    pub(crate) fn new<CG, H>(
        eval: EcdhCoinTossEval<CG, H>,
        round: u8,
    ) -> Result<Self, bson::ser::Error>
    where
        CG: CurveGroup,
        EcdhCoinTossEval<CG, H>: Serialize,
    {
        let ser = bson::to_vec(&eval)?;
        Ok(CoinEvalMessage { eval: ser, round })
    }
}

/// Convert a CoinEvalMessage into an eval.
impl<CG, H> TryInto<EcdhCoinTossEval<CG, H>> for &CoinEvalMessage
where
    CG: CurveGroup,
    EcdhCoinTossEval<CG, H>: for<'de> Deserialize<'de>,
{
    type Error = bson::de::Error;

    fn try_into(self) -> Result<EcdhCoinTossEval<CG, H>, Self::Error> {
        bson::from_slice(&self.eval)
    }
}

impl From<Coin> for Estimate {
    fn from(value: Coin) -> Self {
        match value {
            Coin::Zero => Estimate::Zero,
            Coin::One => Estimate::One,
        }
    }
}

/// Convert single element views into its corresponding estimate, otherwise into Estimate::Bot
impl From<View> for Estimate {
    fn from(val: View) -> Self {
        match val {
            View::Zero => Estimate::Zero,
            View::One => Estimate::One,
            _ => Estimate::Bot,
        }
    }
}
