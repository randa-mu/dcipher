use crate::proto_types::{self, BlockInfo, BlockSafety, RegisterNewEventRequest};
use alloy::dyn_abi::{DynSolEvent, DynSolType, DynSolValue};
use alloy::primitives::{Address, LogData};
use std::borrow::Cow;
use std::str::FromStr;

#[derive(Clone, Copy, Eq, PartialEq, Hash, Debug)]
pub struct EventStreamId(uuid::Uuid);

impl EventStreamId {
    pub fn new() -> EventStreamId {
        EventStreamId(uuid::Uuid::new_v4())
    }
}

impl From<uuid::Uuid> for EventStreamId {
    fn from(value: uuid::Uuid) -> Self {
        Self(value)
    }
}

impl From<EventStreamId> for uuid::Uuid {
    fn from(value: EventStreamId) -> Self {
        value.0
    }
}

impl From<EventStreamId> for prost::bytes::Bytes {
    fn from(value: EventStreamId) -> Self {
        value.0.as_bytes().to_vec().into()
    }
}

impl TryFrom<prost::bytes::Bytes> for EventStreamId {
    type Error = uuid::Error;

    fn try_from(value: prost::bytes::Bytes) -> Result<Self, Self::Error> {
        Ok(Self(uuid::Uuid::from_slice(&value)?))
    }
}

#[derive(Clone, Debug)]
pub(crate) struct ParsedEventField {
    pub(crate) sol_type: DynSolType,
    pub(crate) sol_type_str: Cow<'static, str>,
    pub(crate) indexed: bool,
}

#[derive(Clone, Debug)]
pub(crate) struct EventFieldData {
    pub(crate) sol_type_str: Cow<'static, str>,
    pub(crate) data: DynSolValue,
    pub(crate) indexed: bool,
}

#[derive(Clone, Debug)]
pub(crate) struct ParsedRegisterNewEventRequest {
    /// Chain ID
    pub(crate) chain_id: u64,
    /// Ethereum contract address (20 bytes) - what contract we're watching
    pub(crate) address: Address,
    /// Event name - what event we're watching for
    pub(crate) event_name: String,
    /// Event parameters - the structure of the event
    pub(crate) fields: Vec<ParsedEventField>,
    /// Block safety level - how we want to handle block finality
    pub(crate) block_safety: BlockSafety,
}

/// An event that has been registered with OmniEvent.
#[derive(Clone, Debug)]
pub struct RegisteredEvent {
    pub(crate) id: EventStreamId,
    pub(crate) chain_id: u64,
    pub(crate) address: Address,
    pub(crate) event_name: String,
    pub(crate) event_signature: String,
    pub(crate) fields: Vec<ParsedEventField>,
    pub(crate) sol_event: DynSolEvent,
    pub(crate) block_safety: BlockSafety,
}

/// The occurrence of an event.
#[derive(Clone, Debug)]
pub struct EventOccurrence {
    pub(crate) event_id: EventStreamId,
    pub(crate) block_info: BlockInfo,
    pub(crate) raw_log: LogData,
    pub(crate) data: Vec<EventFieldData>,
}

impl From<EventOccurrence> for proto_types::EventOccurrence {
    fn from(event: EventOccurrence) -> Self {
        let data = event
            .data
            .into_iter()
            .map(|d| proto_types::EventData {
                sol_type: d.sol_type_str.into_owned(),
                indexed: d.indexed,
                value: Some(d.data.into()),
            })
            .collect::<Vec<_>>();

        Self {
            event_uuid: event.event_id.into(),
            event_data: data,
            raw_log_data: Some(event.raw_log.data.into()),
            block_info: Some(event.block_info),
        }
    }
}

#[derive(thiserror::Error, Debug, Clone)]
pub(crate) enum ParseRegisterNewEventRequestError {
    #[error("failed to parse address")]
    TryFromAddress(#[source] <Address as TryFrom<&'static [u8]>>::Error),

    #[error("failed to parse solidity type: `{1}` is unknown")]
    SolType(#[source] <DynSolType as FromStr>::Err, String),

    #[error("failed to parse block safety: `{1}` is unknown")]
    BlockSafety(#[source] <BlockSafety as TryFrom<i32>>::Error, i32),
}

impl TryFrom<RegisterNewEventRequest> for ParsedRegisterNewEventRequest {
    type Error = ParseRegisterNewEventRequestError;

    fn try_from(value: RegisterNewEventRequest) -> Result<Self, Self::Error> {
        // Convert the bytes into an address
        let address =
            Address::try_from(value.address.as_ref()).map_err(Self::Error::TryFromAddress)?;

        // Convert string types into DynSolTypes
        let fields: Vec<_> = value
            .fields
            .into_iter()
            .map(|p| {
                let sol_type = p
                    .sol_type
                    .parse()
                    .map_err(|e| Self::Error::SolType(e, p.sol_type.clone()))?;

                Ok(ParsedEventField {
                    sol_type_str: Cow::Owned(p.sol_type),
                    sol_type,
                    indexed: p.indexed,
                })
            })
            .collect::<Result<_, _>>()?;

        let block_safety = BlockSafety::try_from(value.block_safety)
            .map_err(|e| Self::Error::BlockSafety(e, value.block_safety))?;

        Ok(Self {
            address,
            block_safety,
            fields,
            event_name: value.event_name,
            chain_id: value.chain_id,
        })
    }
}
