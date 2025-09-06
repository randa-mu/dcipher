use crate::proto_types::{self, BlockSafety, RegisterNewEventRequest};
use alloy::dyn_abi::{DynSolEvent, DynSolType, DynSolValue};
use alloy::eips::BlockNumberOrTag;
use alloy::primitives::{Address, B256, LogData, keccak256};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

const EVENT_UUID_NAMESPACE: uuid::Uuid = uuid::Uuid::NAMESPACE_OID;

#[derive(Serialize, Deserialize, Clone, Copy, Eq, PartialEq, Hash, Debug)]
#[serde(into = "bytes::Bytes", try_from = "bytes::Bytes")]
pub struct EventId(uuid::Uuid);

impl EventId {
    pub fn new(data: &[u8]) -> EventId {
        EventId(uuid::Uuid::new_v5(&EVENT_UUID_NAMESPACE, data))
    }
}

#[derive(thiserror::Error, Debug)]
pub enum UuidFromRegisterNewEventRequest {
    #[error("failed to serialize to cbor")]
    Cbor(#[from] serde_cbor::Error),
}

impl TryFrom<&ParsedRegisterNewEventRequest> for uuid::Uuid {
    type Error = UuidFromRegisterNewEventRequest;

    fn try_from(value: &ParsedRegisterNewEventRequest) -> Result<Self, Self::Error> {
        // encode to cbor
        let cbor_encoded = serde_cbor::to_vec(value)?;

        // Use the cbor-encoding to compute a deterministic uuid v5
        Ok(uuid::Uuid::new_v5(&EVENT_UUID_NAMESPACE, &cbor_encoded))
    }
}

impl TryFrom<&ParsedRegisterNewEventRequest> for EventId {
    type Error = UuidFromRegisterNewEventRequest;

    fn try_from(value: &ParsedRegisterNewEventRequest) -> Result<Self, Self::Error> {
        Ok(Self(uuid::Uuid::try_from(value)?))
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

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(into = "ParsedEventFieldDef", try_from = "ParsedEventFieldDef")]
pub struct ParsedEventField {
    pub(crate) sol_type: DynSolType,
    pub(crate) sol_type_str: Cow<'static, str>,
    pub(crate) indexed: bool,
}

impl ParsedEventField {
    pub fn new(sol_type: DynSolType, indexed: bool) -> Self {
        Self {
            sol_type_str: sol_type.sol_type_name(),
            sol_type,
            indexed,
        }
    }

    pub fn sol_type_name(&self) -> Cow<'static, str> {
        self.sol_type_str.clone()
    }

    pub fn sol_type(&self) -> &DynSolType {
        &self.sol_type
    }

    pub fn indexed(&self) -> bool {
        self.indexed
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(into = "EventFieldDataDef", try_from = "EventFieldDataDef")]
pub struct EventFieldData {
    pub sol_type_str: Cow<'static, str>,
    pub data: DynSolValue,
    pub indexed: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ParsedRegisterNewEventRequest {
    /// Chain ID
    pub chain_id: u64,
    /// Ethereum contract address (20 bytes) - what contract we're watching
    pub address: Address,
    /// Event name - what event we're watching for
    pub event_name: String,
    /// Event parameters - the structure of the event
    pub fields: Vec<ParsedEventField>,
    /// Block safety level - how we want to handle block finality
    pub block_safety: BlockSafety,
}

/// An event that has been registered with OmniEvent.
#[derive(Clone, Debug)]
pub struct RegisteredEventSpec {
    pub id: EventId,
    pub chain_id: u64,
    pub address: Address,
    pub block_safety: BlockSafety,
    pub(crate) event_name: String,
    pub(crate) topic0: B256,
    pub(crate) fields: Vec<ParsedEventField>,
    pub(crate) sol_event: DynSolEvent,
}

#[derive(thiserror::Error, Debug)]
pub enum NewRegisteredEventSpecError {
    #[error("failed to compute uuid")]
    IntoEventId(#[from] UuidFromRegisterNewEventRequest),

    #[error("at most 3 indexed fields are supported")]
    TooManyIndexedFields,
}

impl RegisteredEventSpec {
    pub fn try_new(
        id: EventId,
        chain_id: u64,
        address: Address,
        event_name: String,
        fields: Vec<ParsedEventField>,
        block_safety: BlockSafety,
    ) -> Result<Self, NewRegisteredEventSpecError> {
        let indexed_fields: Vec<_> = fields
            .iter()
            .filter_map(|p| p.indexed.then_some(&p.sol_type))
            .cloned()
            .collect();
        if indexed_fields.len() > 3 {
            Err(NewRegisteredEventSpecError::TooManyIndexedFields)?
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

    pub fn topic0(&self) -> B256 {
        self.topic0
    }

    pub fn sol_event(&self) -> &DynSolEvent {
        &self.sol_event
    }

    pub fn event_name(&self) -> &str {
        &self.event_name
    }

    pub fn fields(&self) -> &Vec<ParsedEventField> {
        &self.fields
    }
}

impl TryFrom<ParsedRegisterNewEventRequest> for RegisteredEventSpec {
    type Error = NewRegisteredEventSpecError;

    fn try_from(req: ParsedRegisterNewEventRequest) -> Result<Self, Self::Error> {
        let id = EventId::try_from(&req)?;

        let ParsedRegisterNewEventRequest {
            chain_id,
            address,
            event_name,
            fields,
            block_safety,
        } = req;

        Self::try_new(id, chain_id, address, event_name, fields, block_safety)
    }
}

impl From<&RegisteredEventSpec> for alloy::rpc::types::Filter {
    fn from(event: &RegisteredEventSpec) -> Self {
        alloy::rpc::types::Filter::new()
            .address(event.address)
            .event_signature(event.topic0)
            .from_block(Into::<BlockNumberOrTag>::into(event.block_safety))
    }
}

/// A rusty type storing protobuf's [`BlockInfo`](crate::proto_types::BlockInfo)
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BlockInfo {
    pub number: u64,
    pub hash: bytes::Bytes,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

impl From<BlockInfo> for proto_types::BlockInfo {
    fn from(block: BlockInfo) -> Self {
        Self {
            block_number: block.number,
            block_hash: block.hash,
            timestamp: Some(prost_types::Timestamp {
                nanos: chrono::Timelike::nanosecond(&block.timestamp)
                    .try_into()
                    .expect("safe, outputs at most 1_999_999 < 2**31"),
                seconds: block.timestamp.timestamp(),
            }),
        }
    }
}

/// The occurrence of an event.
#[derive(Clone, Debug, PartialEq)]
pub struct EventOccurrence {
    pub event_id: EventId,
    pub chain_id: u64,
    pub address: Address,
    pub block_info: BlockInfo,
    pub raw_log: LogData,
    pub data: Vec<EventFieldData>,
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
            address: event.address.to_vec().into(),
            chain_id: event.chain_id,
            event_data: data,
            raw_log_data: Some(event.raw_log.data.into()),
            block_info: Some(event.block_info.into()),
        }
    }
}

#[derive(thiserror::Error, Debug)]
pub enum ParseRegisterNewEventRequestError {
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

                Ok(ParsedEventField::new(sol_type, p.indexed))
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

/// Serde-compatible [`ParsedEventField`]
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ParsedEventFieldDef {
    #[serde(rename = "sol_type")]
    pub sol_type_str: Cow<'static, str>,
    pub indexed: bool,
}

impl From<ParsedEventField> for ParsedEventFieldDef {
    fn from(value: ParsedEventField) -> Self {
        Self {
            sol_type_str: value.sol_type_str,
            indexed: value.indexed,
        }
    }
}

#[derive(thiserror::Error, Debug, Clone)]
pub enum ParsedEventFieldFromDefError {
    #[error("failed to parse type into DynSolType")]
    ParseDynSolType(#[source] <DynSolType as FromStr>::Err),
}

impl TryFrom<ParsedEventFieldDef> for ParsedEventField {
    type Error = ParsedEventFieldFromDefError;

    fn try_from(value: ParsedEventFieldDef) -> Result<Self, Self::Error> {
        let sol_type =
            DynSolType::from_str(&value.sol_type_str).map_err(Self::Error::ParseDynSolType)?;

        Ok(Self {
            sol_type_str: value.sol_type_str,
            sol_type,
            indexed: value.indexed,
        })
    }
}

/// Serde-compatible [`EventFieldData`]
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct EventFieldDataDef {
    pub sol_type_str: Cow<'static, str>,
    pub data: Vec<u8>,
    pub indexed: bool,
}

impl From<EventFieldData> for EventFieldDataDef {
    fn from(value: EventFieldData) -> Self {
        Self {
            sol_type_str: value.sol_type_str,
            indexed: value.indexed,
            data: value.data.abi_encode(),
        }
    }
}

#[derive(thiserror::Error, Debug, Clone)]
pub enum EventFieldDataFromDefError {
    #[error("failed to parse type into DynSolType")]
    ParseDynSolType(#[source] <DynSolType as FromStr>::Err),

    #[error("failed to abi decode bytes")]
    AbiDecode(#[source] alloy::dyn_abi::Error),
}

impl TryFrom<EventFieldDataDef> for EventFieldData {
    type Error = EventFieldDataFromDefError;

    fn try_from(value: EventFieldDataDef) -> Result<Self, Self::Error> {
        let dyn_sol_type =
            DynSolType::from_str(&value.sol_type_str).map_err(Self::Error::ParseDynSolType)?;
        let data = dyn_sol_type
            .abi_decode(&value.data)
            .map_err(Self::Error::AbiDecode)?;

        Ok(Self {
            sol_type_str: value.sol_type_str,
            data,
            indexed: value.indexed,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::proto_types::BlockSafety;
    use crate::types::{EventId, ParsedEventField, ParsedRegisterNewEventRequest};
    use alloy::dyn_abi::DynSolType;
    use std::str::FromStr;

    #[test]
    fn event_id_from_register_new_event_request() {
        let req = ParsedRegisterNewEventRequest {
            chain_id: 1337,
            address: "0x20EEF038C83B7a0f357D4aBC64b8f639427D7Af6"
                .parse()
                .unwrap(),
            event_name: "Subscribed".to_owned(),
            fields: vec![
                ParsedEventField::new(DynSolType::Address, true),
                ParsedEventField::new(DynSolType::Uint(256), true),
            ],
            block_safety: BlockSafety::Latest,
        };
        // cbor diagnostic notation:
        // {
        //     "chain_id": 1337_1,
        //     "address": h'20eef038c83b7a0f357d4abc64b8f639427d7af6',
        //     "event_name": "Subscribed",
        //     "fields": [
        //         {"sol_type": "address", "indexed": true},
        //         {"sol_type": "uint256", "indexed": true},
        //     ],
        //     "block_safety": "BLOCK_SAFETY_LATEST",
        // }
        let cbor = hex::decode("a568636861696e5f696419053967616464726573735420eef038c83b7a0f357d4abc64b8f639427d7af66a6576656e745f6e616d656a53756273637269626564666669656c647382a268736f6c5f74797065676164647265737367696e6465786564f5a268736f6c5f747970656775696e7432353667696e6465786564f56c626c6f636b5f73616665747973424c4f434b5f5341464554595f4c4154455354").unwrap();
        let expected_event_id =
            EventId::from(uuid::Uuid::from_str("d451b061-fc2e-5391-ae0b-5b1699a55304").unwrap());
        let cbor_event_id = EventId::new(&cbor);

        let event_id = EventId::try_from(&req).expect("failed to convert req to uuid");
        assert_eq!(event_id, expected_event_id);
        assert_eq!(event_id, cbor_event_id);
    }
}
