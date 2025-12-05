//! Module to solve / relay tokens gaslessly

use crate::gasless::permit2::transfer_from::Permit2TransferFromParameters;
use crate::gasless::permit2::witness_transfer_from::{
    Permit2CustomWitness, Permit2WitnessError, permit2_witness_transfer_from_message_hash,
};
use crate::model::Trade;
use crate::network::Network;
use alloy::dyn_abi::DynSolValue;
use alloy::primitives::{Address, B256, U256};
use alloy::providers::Provider;
use anyhow::Context;
use generated::onlyswaps::permit2_relayer::Permit2Relayer::Permit2RelayerInstance;
use serde_json::json;

pub mod permit2;

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
    own_addr: Address,
    permit2_override: Option<Address>,
) -> Result<Permit2RelayTokensDetails, Permit2WitnessError> {
    let additional_data = DynSolValue::Tuple(vec![DynSolValue::Address(own_addr)]);
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
            "additionalData": additional_data.abi_encode(),
        }),
    };

    // TODO: Optimize for permit2 nonce schema: https://docs.uniswap.org/contracts/permit2/reference/signature-transfer#nonce-schema
    let rand_nonce = U256::random();

    let params = Permit2TransferFromParameters {
        // swap params
        chain_id: trade.dest_chain_id.try_into().expect("invalid chain_id"),
        token_addr: trade.token_out_addr,
        token_amount: trade.amount_out,
        spender_addr,

        // permit2 params
        deadline: U256::MAX, // infinite
        nonce: rand_nonce,
        permit2_address_override: permit2_override,
    };

    let message_hash = permit2_witness_transfer_from_message_hash(params, witness)?;
    Ok(Permit2RelayTokensDetails {
        message_hash,
        nonce: params.nonce,
        deadline: params.deadline,
    })
}

pub async fn fetch_permit2_addresses<'a, P>(
    networks: impl IntoIterator<Item = (&'a u64, &'a Network<P>)>,
) -> anyhow::Result<impl Iterator<Item = (u64, Address)>>
where
    P: Provider + 'a,
{
    let permit2_addresses =
        futures::future::try_join_all(networks.into_iter().map(async |(&id, c)| {
            Permit2RelayerInstance::new(c.permit2_relayer_address, c.router.provider())
                .PERMIT2()
                .call()
                .await
                .map(|addr| (id, addr))
        }))
        .await
        .context("failed to get permit2 addresses")?;

    Ok(permit2_addresses.into_iter())
}
