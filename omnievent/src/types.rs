use crate::proto_types::{self, BlockInfo, BlockSafety, RegisterNewEventRequest};
use alloy::dyn_abi::{DynSolEvent, DynSolType, DynSolValue};
use alloy::eips::BlockNumberOrTag;
use alloy::primitives::{Address, B256, LogData, keccak256};
use std::borrow::Cow;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

#[derive(Clone, Copy, Eq, PartialEq, Hash, Debug)]
pub struct EventId(uuid::Uuid);

impl EventId {
    pub fn new(data: &[u8]) -> EventId {
        EventId(uuid::Uuid::new_v5(&uuid::Uuid::NAMESPACE_OID, data))
    }
}

impl Display for EventId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0.to_string())
    }
}

impl From<uuid::Uuid> for EventId {
    fn from(value: uuid::Uuid) -> Self {
        Self(value)
    }
}

impl From<EventId> for uuid::Uuid {
    fn from(value: EventId) -> Self {
        value.0
    }
}

impl From<EventId> for prost::bytes::Bytes {
    fn from(value: EventId) -> Self {
        value.0.as_bytes().to_vec().into()
    }
}

impl TryFrom<prost::bytes::Bytes> for EventId {
    type Error = uuid::Error;

    fn try_from(value: prost::bytes::Bytes) -> Result<Self, Self::Error> {
        Ok(Self(uuid::Uuid::from_slice(&value)?))
    }
}

#[derive(Clone, Debug)]
pub struct ParsedEventField {
    pub(crate) sol_type: DynSolType,
    pub(crate) sol_type_str: Cow<'static, str>,
    pub(crate) indexed: bool,
}

impl ParsedEventField {
    pub(crate) fn new(sol_type: DynSolType, indexed: bool) -> Self {
        Self {
            sol_type_str: sol_type.sol_type_name(),
            sol_type,
            indexed,
        }
    }
}

#[derive(Clone, Debug)]
pub(crate) struct EventFieldData {
    pub(crate) sol_type_str: Cow<'static, str>,
    pub(crate) data: DynSolValue,
    pub(crate) indexed: bool,
}

#[derive(Clone, Debug)]
pub(crate) struct ParsedRegisterNewEventRequest {
    /// Deterministic UUID v5
    pub(crate) id: EventId,
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
    pub id: EventId,
    pub chain_id: u64,
    pub address: Address,
    pub event_name: String,
    pub topic0: B256,
    pub fields: Vec<ParsedEventField>,
    pub sol_event: DynSolEvent,
    pub block_safety: BlockSafety,
}

#[derive(thiserror::Error, Debug)]
pub enum NewRegisteredEventError {
    #[error("at most 3 indexed fields are supported")]
    TooManyIndexedFields,
}

impl RegisteredEvent {
    pub fn try_new(
        id: EventId,
        chain_id: u64,
        address: Address,
        event_name: String,
        fields: Vec<ParsedEventField>,
        block_safety: BlockSafety,
    ) -> Result<Self, NewRegisteredEventError> {
        let indexed_fields: Vec<_> = fields
            .iter()
            .filter_map(|p| p.indexed.then_some(&p.sol_type))
            .cloned()
            .collect();
        if indexed_fields.len() > 3 {
            Err(NewRegisteredEventError::TooManyIndexedFields)?
        }

        let sol_type_name = if fields.len() != 1 {
            // Sol type include all fields
            DynSolType::Tuple(fields.iter().map(|p| &p.sol_type).cloned().collect())
                .sol_type_name()
                .into_owned()
        } else {
            // annoying to use tuple here because sol_type_name() uses a trailing ',' for 1-tuples.
            format!("({})", fields[0].sol_type.sol_type_name())
        };

        let event_signature = format!("{}{}", event_name, sol_type_name);
        let topic0 = keccak256(event_signature.as_bytes());

        // Body only includes non-indexed fields
        let body_type = DynSolType::Tuple(
            fields
                .iter()
                .filter_map(|p| if p.indexed { None } else { Some(&p.sol_type) })
                .cloned()
                .collect(),
        );
        let sol_event = DynSolEvent::new(Some(topic0), indexed_fields, body_type)
            .expect("at most 3 indexed fields and sol_type is a tuple");

        Ok(Self {
            id,
            chain_id,
            address,
            event_name,
            topic0,
            fields,
            sol_event,
            block_safety,
        })
    }

    pub(crate) fn try_from_req(
        req: ParsedRegisterNewEventRequest,
    ) -> Result<Self, NewRegisteredEventError> {
        let ParsedRegisterNewEventRequest {
            id,
            chain_id,
            address,
            event_name,
            fields,
            block_safety,
        } = req;

        Self::try_new(id, chain_id, address, event_name, fields, block_safety)
    }
}

impl From<&RegisteredEvent> for alloy::rpc::types::Filter {
    fn from(event: &RegisteredEvent) -> Self {
        alloy::rpc::types::Filter::new()
            .address(event.address)
            .event_signature(event.topic0)
            .from_block(Into::<BlockNumberOrTag>::into(event.block_safety))
    }
}

/// The occurrence of an event.
#[derive(Clone, Debug)]
pub struct EventOccurrence {
    pub(crate) event_id: EventId,
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
        // Use the protobuf-encoded value to compute an uuid v5
        let id = EventId::new(&prost::Message::encode_to_vec(&value));

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

                Ok(ParsedEventField::new(sol_type, p.indexed))
            })
            .collect::<Result<_, _>>()?;

        let block_safety = BlockSafety::try_from(value.block_safety)
            .map_err(|e| Self::Error::BlockSafety(e, value.block_safety))?;

        Ok(Self {
            id,
            address,
            block_safety,
            fields,
            event_name: value.event_name,
            chain_id: value.chain_id,
        })
    }
}
