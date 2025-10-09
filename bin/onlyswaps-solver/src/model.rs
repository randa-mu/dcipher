use alloy::primitives::{Address, U256};
use generated::onlyswaps::router::IRouter::SwapRequestParameters;
use std::collections::HashMap;

pub type RequestId = [u8; 32];
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChainState {
    pub native_balance: U256,
    pub token_balances: HashMap<Address, U256>,
    pub transfers: Vec<Transfer>,
    pub already_fulfilled: Vec<RequestId>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Transfer {
    pub request_id: RequestId,
    pub params: SwapRequestParameters,
}

impl From<&Transfer> for Trade {
    fn from(transfer: &Transfer) -> Self {
        Trade {
            token_in_addr: transfer.params.tokenIn,
            token_out_addr: transfer.params.tokenOut,
            src_chain_id: transfer.params.srcChainId,
            dest_chain_id: transfer.params.dstChainId,
            sender_addr: transfer.params.sender,
            recipient_addr: transfer.params.recipient,
            request_id: transfer.request_id,
            swap_amount: transfer.params.amountOut,
            nonce: transfer.params.nonce,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Trade {
    pub token_in_addr: Address,
    pub token_out_addr: Address,
    pub src_chain_id: U256,
    pub dest_chain_id: U256,
    pub sender_addr: Address,
    pub recipient_addr: Address,
    pub request_id: RequestId,
    pub swap_amount: U256,
    pub nonce: U256,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub(crate) struct BlockEvent {
    pub chain_id: u64,
    pub block_number: u64,
}
