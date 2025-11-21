use alloy::primitives::{Address, FixedBytes, U256};
use generated::onlyswaps::i_router::IRouter::{Hook, SwapRequestParametersWithHooks};
use std::collections::HashMap;

pub type RequestId = FixedBytes<32>;
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
    pub params: SwapRequestParametersWithHooks,
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
            amount_in: transfer.params.amountIn,
            amount_out: transfer.params.amountOut,
            solver_fee: transfer.params.solverFee,
            nonce: transfer.params.nonce,
            pre_hooks: transfer.params.preHooks.clone(),
            post_hooks: transfer.params.postHooks.clone(),
        }
    }
}
use std::cmp::Ordering;
use std::hash::{Hash, Hasher};

#[derive(Debug, Clone)]
pub struct Trade {
    pub token_in_addr: Address,
    pub token_out_addr: Address,
    pub src_chain_id: U256,
    pub dest_chain_id: U256,
    pub sender_addr: Address,
    pub recipient_addr: Address,
    pub request_id: RequestId,
    pub amount_in: U256,
    pub amount_out: U256,
    pub solver_fee: U256,
    pub nonce: U256,
    pub pre_hooks: Vec<Hook>,
    pub post_hooks: Vec<Hook>,
}

// we ignore hooks because they aren't `PartialEq`
impl PartialEq for Trade {
    fn eq(&self, other: &Self) -> bool {
        (
            &self.token_in_addr,
            &self.token_out_addr,
            &self.src_chain_id,
            &self.dest_chain_id,
            &self.sender_addr,
            &self.recipient_addr,
            &self.request_id,
            &self.amount_in,
            &self.amount_out,
            &self.solver_fee,
            &self.nonce,
        ) == (
            &other.token_in_addr,
            &other.token_out_addr,
            &other.src_chain_id,
            &other.dest_chain_id,
            &other.sender_addr,
            &other.recipient_addr,
            &other.request_id,
            &other.amount_in,
            &other.amount_out,
            &other.solver_fee,
            &other.nonce,
        )
    }
}

impl Eq for Trade {}

impl PartialOrd for Trade {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Trade {
    fn cmp(&self, other: &Self) -> Ordering {
        (
            &self.token_in_addr,
            &self.token_out_addr,
            &self.src_chain_id,
            &self.dest_chain_id,
            &self.sender_addr,
            &self.recipient_addr,
            &self.request_id,
            &self.amount_in,
            &self.amount_out,
            &self.solver_fee,
            &self.nonce,
        )
            .cmp(&(
                &other.token_in_addr,
                &other.token_out_addr,
                &other.src_chain_id,
                &other.dest_chain_id,
                &other.sender_addr,
                &other.recipient_addr,
                &other.request_id,
                &other.amount_in,
                &other.amount_out,
                &other.solver_fee,
                &other.nonce,
            ))
    }
}

impl Hash for Trade {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.token_in_addr.hash(state);
        self.token_out_addr.hash(state);
        self.src_chain_id.hash(state);
        self.dest_chain_id.hash(state);
        self.sender_addr.hash(state);
        self.recipient_addr.hash(state);
        self.request_id.hash(state);
        self.amount_in.hash(state);
        self.amount_out.hash(state);
        self.solver_fee.hash(state);
        self.nonce.hash(state);
        self.pre_hooks.hash(state);
        self.post_hooks.hash(state);
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub(crate) struct BlockEvent {
    pub chain_id: u64,
    pub block_number: u64,
}
