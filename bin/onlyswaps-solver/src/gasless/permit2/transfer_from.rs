//! EIP-712 typed data for permit2's PermitTransferFrom.

use alloy::dyn_abi::TypedData;
use alloy::primitives::{Address, U256, address};
use serde_json::{Value, json};

const DEFAULT_PERMIT2_ADDRESS: Address = address!("0x000000000022D473030F116dDEE9F6B43aC78BA3");

/// Contains the various permit2 transfer from parameters.
#[derive(Copy, Clone, Default, Debug)]
pub struct Permit2TransferFromParameters {
    /// a fresh permit2 nonce
    pub nonce: U256,

    /// the deadline until which permit2 accepts the transfer
    pub deadline: U256,

    /// the chain id to approve the transfer for
    pub chain_id: u64,

    /// the spender of the token, i.e., the address to which the token can be transferred
    pub spender_addr: Address,

    /// the token to transfer
    pub token_addr: Address,

    /// the allowance amount
    pub token_amount: U256,

    /// optionally override the default permit2 address
    pub permit2_address_override: Option<Address>,
}

/// Create the permit2 transfer from eip-712 typed data.
pub fn permit2_transfer_from_typed_data(params: Permit2TransferFromParameters) -> TypedData {
    let permit2_transfer_from_json = permit2_transfer_from_json(params);

    // structure is always syntactically correct, the params cannot change that
    serde_json::from_value(permit2_transfer_from_json).expect("a valid typed data")
}

/// Create the permit2 transfer from json.
pub fn permit2_transfer_from_json(params: Permit2TransferFromParameters) -> Value {
    let Permit2TransferFromParameters {
        nonce,
        deadline,
        chain_id,
        spender_addr,
        token_addr,
        token_amount,
        permit2_address_override,
    } = params;

    json!({
        "types": {
            "EIP712Domain": [
                {
                    "name": "name",
                    "type": "string"
                },
                {
                    "name": "chainId",
                    "type": "uint256"
                },
                {
                    "name": "verifyingContract",
                    "type": "address"
                },
            ],
            "PermitTransferFrom": [
                {
                    "name": "permitted",
                    "type": "TokenPermissions"
                },
                {
                    "name": "spender",
                    "type": "address"
                },
                {
                    "name": "nonce",
                    "type": "uint256"
                },
                {
                    "name": "deadline",
                    "type": "uint256"
                },
            ],
            "TokenPermissions": [
                {
                    "name": "token",
                    "type": "address"
                },
                {
                    "name": "amount",
                    "type": "uint256"
                },
            ],
        },
        "domain": {
            "name": "Permit2",
            "chainId": chain_id,
            "verifyingContract": permit2_address_override.unwrap_or(DEFAULT_PERMIT2_ADDRESS),
        },
        "primaryType": "PermitTransferFrom",
        "message": {
            "permitted": {
                "token": token_addr,
                "amount": token_amount,
            },
            "spender": spender_addr,
            "nonce": nonce,
            "deadline": deadline,
        },
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_match_expected_permit_transfer_from_type_str() {
        // from: https://github.com/Uniswap/permit2/blob/cc56ad0f3439c502c246fc5cfcc3db92bb8b7219/src/libraries/PermitHash.sol#L21-L23
        const PERMIT_TRANSFER_FROM_TYPE_STR: &str = "PermitTransferFrom(TokenPermissions permitted,address spender,uint256 nonce,uint256 deadline)TokenPermissions(address token,uint256 amount)";

        let typed_data = permit2_transfer_from_typed_data(Default::default());
        assert_eq!(
            typed_data.encode_type().expect("a valid typed data"),
            PERMIT_TRANSFER_FROM_TYPE_STR
        )
    }
}
