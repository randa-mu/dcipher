//! Module to solve / relay tokens gaslessly

use crate::gasless::permit2::transfer_from::Permit2TransferFromParameters;
use crate::gasless::permit2::witness_transfer_from::{
    Permit2CustomWitness, Permit2WitnessError, permit2_witness_transfer_from_message_hash,
};
use crate::model::Trade;
use alloy::primitives::{Address, B256, U256};
use serde_json::json;

mod permit2;

#[derive(Copy, Clone, Default, Debug)]
pub struct Permit2RelayTokensDetails {
    /// the message hash to sign
    pub message_hash: B256,

    /// a fresh permit2 nonce
    pub nonce: U256,

    /// the deadline until which permit2 accepts the transfer
    pub deadline: U256,
}

/// Compute the message hash to sign to issue a permit2 allowance signature.
pub fn permit2_relay_tokens_details(
    trade: &Trade,
    spender_addr: Address,
) -> Result<Permit2RelayTokensDetails, Permit2WitnessError> {
    let witness = Permit2CustomWitness {
        witness_type_name: "RelayerWitness".to_owned(),
        witness_argument_name: "witness".to_owned(),
        witness_type_json: json!([
            {
                "name": "requestId",
                "type": "bytes32",
            },
            {
                "name": "recipient",
                "type": "address",
            },
            {
                "name": "additionalData",
                "type": "bytes",
            },
        ]),
        witness_data_json: json!({
            "requestId": trade.request_id,
            "recipient": trade.recipient_addr,
            "additionalData": alloy::primitives::Bytes::default(),
        }),
    };

    // TODO: Optimize for permit2 nonce schema: https://docs.uniswap.org/contracts/permit2/reference/signature-transfer#nonce-schema
    let rand_nonce = U256::random();

    let params = Permit2TransferFromParameters {
        // swap params
        chain_id: trade.dest_chain_id.try_into().expect("invalid chain_id"),
        token_addr: trade.token_out_addr,
        token_amount: trade.swap_amount,
        spender_addr,

        // permit2 params
        deadline: U256::MAX, // infinite
        nonce: rand_nonce,

        // rest is default
        ..Default::default()
    };

    let message_hash = permit2_witness_transfer_from_message_hash(params, witness)?;
    Ok(Permit2RelayTokensDetails {
        message_hash,
        nonce: params.nonce,
        deadline: params.deadline,
    })
}
