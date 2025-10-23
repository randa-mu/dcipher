use crate::serde::ShortNumber;
use crate::service::SwapTransactionQueryFilter;
use crate::state::SwapTransaction;
use alloy::primitives::{Address, FixedBytes};

pub fn matches(tx: &SwapTransaction, filter: SwapTransactionQueryFilter) -> bool {
    [
        filter.request_id.is_none_or(|id| is_id(tx, &id)),
        filter
            .chain_id
            .is_none_or(|chain_id| is_chain_id(tx, chain_id)),
        filter.address.is_none_or(|addr| is_address(tx, addr)),
        filter.sender.is_none_or(|sender| is_sender(tx, sender)),
        filter
            .recipient
            .is_none_or(|recipient| is_recipient(tx, recipient)),
        filter.solver.is_none_or(|solver| is_solver(tx, solver)),
        is_requested_time(tx, filter.requested_time_start, filter.requested_time_end),
        is_verified_time(tx, filter.verified_time_start, filter.verified_time_end),
    ]
    .into_iter()
    .all(|x| x)
}

fn is_id(tx: &SwapTransaction, request_id: &FixedBytes<32>) -> bool {
    tx.request_id == *request_id
}
fn is_chain_id(tx: &SwapTransaction, chain_id: ShortNumber) -> bool {
    tx.src_chain_id == chain_id || tx.dest_chain_id == chain_id
}

fn is_address(tx: &SwapTransaction, address: Address) -> bool {
    is_sender(tx, address) || is_recipient(tx, address) || is_solver(tx, address)
}
fn is_sender(tx: &SwapTransaction, address: Address) -> bool {
    tx.sender == address
}

fn is_recipient(tx: &SwapTransaction, address: Address) -> bool {
    tx.recipient == address
}

fn is_solver(tx: &SwapTransaction, address: Address) -> bool {
    tx.solver == Some(address)
}

fn is_requested_time(tx: &SwapTransaction, start: Option<u64>, end: Option<u64>) -> bool {
    let start = start.unwrap_or(0);
    let end = end.unwrap_or(u64::MAX);
    tx.requested_time.0 >= start && tx.requested_time.0 <= end
}

fn is_verified_time(tx: &SwapTransaction, start: Option<u64>, end: Option<u64>) -> bool {
    let start = start.unwrap_or(0);
    let end = end.unwrap_or(u64::MAX);
    match &tx.verified_time {
        None => false,
        Some(t) => t.0 >= start && t.0 <= end,
    }
}
