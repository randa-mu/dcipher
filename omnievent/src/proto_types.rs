mod events {
    include!(concat!(env!("OUT_DIR"), "/events.rs"));

    impl From<BlockSafety> for alloy::eips::BlockNumberOrTag {
        fn from(value: BlockSafety) -> Self {
            match value {
                BlockSafety::Latest => Self::Latest,
                BlockSafety::Safe => Self::Safe,
                BlockSafety::Finalized => Self::Finalized,
            }
        }
    }

    impl From<alloy::dyn_abi::DynSolValue> for event_data::Value {
        fn from(value: alloy::dyn_abi::DynSolValue) -> Self {
            match value {
                // String
                alloy::dyn_abi::DynSolValue::String(s) => Self::StringValue(s),

                // Unsigned int => convert to hex string
                alloy::dyn_abi::DynSolValue::Uint(u, _size) => {
                    Self::IntHexValue(format!("0x{:x}", u))
                }

                // Signed int => convert to signed hex string
                alloy::dyn_abi::DynSolValue::Int(i, _size) => {
                    Self::IntHexValue(format!("0x{:x}", i))
                }

                // Boolean
                alloy::dyn_abi::DynSolValue::Bool(b) => Self::BoolValue(b),

                // Address - 20 bytes
                alloy::dyn_abi::DynSolValue::Address(addr) => {
                    Self::AddressValue(addr.to_vec().into())
                }

                // Fixed bytes
                alloy::dyn_abi::DynSolValue::FixedBytes(bytes, _size) => {
                    Self::BytesValue(bytes.to_vec().into())
                }

                // Dynamic bytes
                alloy::dyn_abi::DynSolValue::Bytes(bytes) => Self::BytesValue(bytes.into()),

                // Abi encode everything else
                _ => Self::AbiBytes(value.abi_encode().into()),
            }
        }
    }
}

pub use events::*;
