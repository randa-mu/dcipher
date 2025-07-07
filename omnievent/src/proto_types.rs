mod events {
    #![allow(clippy::enum_variant_names)]

    use alloy::dyn_abi::DynSolValue;
    use alloy::primitives::Address;

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

    impl occurrence_data_filter::Filter {
        pub fn is_compatible(&self, sol_type: &alloy::dyn_abi::DynSolType) -> bool {
            use alloy::dyn_abi::DynSolType;

            match self {
                Self::String(_) => {
                    matches!(sol_type, DynSolType::String)
                }
                Self::Int(_) => {
                    matches!(sol_type, DynSolType::Int(_))
                }
                Self::Uint(_) => {
                    matches!(sol_type, DynSolType::Uint(_))
                }
                Self::Bool(_) => {
                    matches!(sol_type, DynSolType::Bool)
                }
                Self::Bytes(_) => {
                    matches!(sol_type, DynSolType::Bytes)
                }
                Self::Address(_) => {
                    matches!(sol_type, DynSolType::Address)
                }
                Self::AbiBytes(_) => {
                    // We can always compare filter by abi bytes
                    true
                }
            }
        }

        /// Returns Some(bool) if the filter can be applied, None otherwise
        pub fn apply(&self, value: &DynSolValue) -> Option<bool> {
            match (self, value) {
                (Self::String(filter), DynSolValue::String(value)) => Some(filter.apply(value)),
                (Self::Int(filter), DynSolValue::Int(value, ..)) => Some(filter.apply(*value)),
                (Self::Uint(filter), DynSolValue::Uint(value, ..)) => Some(filter.apply(*value)),
                (Self::Bool(filter), DynSolValue::Bool(value)) => Some(filter.apply(*value)),
                (Self::Address(filter), DynSolValue::Address(value)) => Some(filter.apply(value)),
                (Self::Bytes(filter), DynSolValue::Bytes(value)) => {
                    Some(filter.apply(value.to_owned()))
                }
                (Self::AbiBytes(filter), value) => Some(filter.apply(value.abi_encode())),
                _ => None, // Value cannot be filtered, return None
            }
        }
    }

    impl StringDataFilter {
        pub fn apply(&self, value: &String) -> bool {
            self.exact_values.contains(value)
        }
    }

    impl IntDataFilter {
        pub fn apply(&self, value: alloy::primitives::I256) -> bool {
            if let event_data::Value::IntHexValue(int_hex) =
                event_data::Value::from(DynSolValue::Int(value, 256))
            {
                self.exact_hex_values.contains(&int_hex)
            } else {
                // panic as this would be a bug
                panic!("Value::from(int) resulted in a non-IntHexValue");
            }
        }
    }

    impl UintDataFilter {
        pub fn apply(&self, value: alloy::primitives::U256) -> bool {
            if let event_data::Value::IntHexValue(int_hex) =
                event_data::Value::from(DynSolValue::Uint(value, 256))
            {
                self.exact_hex_values.contains(&int_hex)
            } else {
                // panic as this would be a bug
                panic!("Value::from(uint) resulted in a non-IntHexValue");
            }
        }
    }

    impl AddressDataFilter {
        pub fn apply(&self, value: &Address) -> bool {
            self.exact_values.contains(&value.to_vec().into())
        }
    }

    impl BoolDataFilter {
        pub fn apply(&self, value: bool) -> bool {
            self.exact_value == value
        }
    }

    impl BytesDataFilter {
        pub fn apply(&self, value: impl Into<tonic::codegen::Bytes>) -> bool {
            self.exact_values.contains(&value.into())
        }
    }
}

pub use events::*;
