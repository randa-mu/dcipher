//! EIP-712 typed data for permit2's PermitWitnessTransferFrom.

use crate::gasless::permit2::transfer_from::{
    Permit2TransferFromParameters, permit2_transfer_from_json,
};
use alloy::dyn_abi::TypedData;
use alloy::primitives::B256;
use serde_json::{Value, json};

// Fixed permit2 witness transfer type string from https://github.com/Uniswap/permit2/blob/cc56ad0f3439c502c246fc5cfcc3db92bb8b7219/src/libraries/PermitHash.sol#L31-L32
const PERMIT2_WITNESS_TRANSFER_FIXED_TYPE_STR: &str = "PermitWitnessTransferFrom(TokenPermissions permitted,address spender,uint256 nonce,uint256 deadline,";

#[derive(thiserror::Error, Debug)]
pub enum Permit2WitnessError {
    #[error("invalid dynamic abi: check witness definition")]
    DynAbi(#[from] alloy::dyn_abi::Error),

    #[error("failed to deserialize witness")]
    JsonDeserialize(#[from] serde_json::Error),
}

/// A custom witness used by permit2
#[derive(Clone, Debug)]
pub struct Permit2CustomWitness {
    pub witness_type_name: String,
    pub witness_type_json: Value,

    pub witness_argument_name: String,
    pub witness_data_json: Value,
}

/// Compute the message to be signed to create a permit.
pub fn permit2_witness_transfer_from_message_hash(
    params: Permit2TransferFromParameters,
    witness: Permit2CustomWitness,
) -> Result<B256, Permit2WitnessError> {
    let typed_data = permit2_witness_transfer_from_typed_data(params, witness.clone())?;
    Ok(typed_data.eip712_signing_hash()?)
}

/// Create the permit2 witness transfer from eip-712 typed data.
pub fn permit2_witness_transfer_from_typed_data(
    params: Permit2TransferFromParameters,
    witness: Permit2CustomWitness,
) -> Result<TypedData, Permit2WitnessError> {
    let json_typed_data = permit2_witness_transfer_from_json(params, witness);
    Ok(serde_json::from_value(json_typed_data)?)
}

/// Create the permit2 witness transfer from eip-712 typed data in json format.
pub fn permit2_witness_transfer_from_json(
    params: Permit2TransferFromParameters,
    witness: Permit2CustomWitness,
) -> Value {
    const OLD_PRIMARY_TYPE: &str = "PermitTransferFrom";
    const NEW_PRIMARY_TYPE: &str = "PermitWitnessTransferFrom";

    let Permit2CustomWitness {
        witness_type_name,
        witness_type_json,
        witness_argument_name,
        witness_data_json,
    } = witness;

    // first, obtain the permit2 transfer from json, i.e., the eip-712 encoding for
    // permit2 transfer _without_ witness.
    let mut witness_transfer_from_json = permit2_transfer_from_json(params);

    // rename PermitTransferFrom type and update primary type
    let old_type = witness_transfer_from_json["types"]
        .as_object_mut()
        .expect("an object")
        .remove(OLD_PRIMARY_TYPE)
        .expect("entry to exist");
    witness_transfer_from_json["types"][NEW_PRIMARY_TYPE] = old_type;
    witness_transfer_from_json["primaryType"] = NEW_PRIMARY_TYPE.into();

    // add our custom witness, i.e., .types.Witness = witness_type
    witness_transfer_from_json["types"][&witness_type_name] = witness_type_json;

    // add our custom argument to the PermitWitnessTransferFrom type
    witness_transfer_from_json["types"][NEW_PRIMARY_TYPE]
        .as_array_mut()
        .expect("to contain an array of arguments")
        .push(json!({
            "name": witness_argument_name,
            "type": witness_type_name,
        }));

    // set the value of the witness argument
    witness_transfer_from_json["message"][&witness_argument_name] = witness_data_json;
    witness_transfer_from_json
}

#[cfg(test)]
mod tests {
    use super::*;

    const PARTIAL_WITNESS_TYPE_STR: &str = "Witness witness)TokenPermissions(address token,uint256 amount)Witness(bytes calldata_relayTokens)";

    fn get_custom_witness() -> Permit2CustomWitness {
        Permit2CustomWitness {
            witness_type_name: "Witness".to_owned(),
            witness_argument_name: "witness".to_owned(),
            witness_type_json: json!([
                {
                    "name": "calldata_relayTokens",
                    "type": "bytes",
                },
            ]),
            witness_data_json: json!({
                "calldata_relayTokens": alloy::primitives::Bytes::default(),
            }),
        }
    }

    #[test]
    fn should_match_expected_permit_witness_transfer_from_type_str() {
        let typed_data =
            permit2_witness_transfer_from_typed_data(Default::default(), get_custom_witness())
                .expect("a valid typed data");

        assert_eq!(
            typed_data.encode_type().expect("a valid typed data"),
            format!("{PERMIT2_WITNESS_TRANSFER_FIXED_TYPE_STR}{PARTIAL_WITNESS_TYPE_STR}")
        );
    }
}
