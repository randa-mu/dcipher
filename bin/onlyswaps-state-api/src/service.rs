use crate::serde::ShortNumber;
use crate::state::{AppState, SwapTransaction};
use alloy::primitives::{Address, FixedBytes};
use serde::Deserialize;
use tokio::sync::watch;

pub(crate) trait StateService: Send + Sync {
    fn get_transactions(
        &self,
        filter: SwapTransactionQueryFilter,
    ) -> anyhow::Result<Vec<SwapTransaction>>;
}

#[derive(Debug, Default, Deserialize)]
pub(crate) struct SwapTransactionQueryFilter {
    pub request_id: Option<FixedBytes<32>>,
    pub chain_id: Option<ShortNumber>,
    pub address: Option<Address>,
    pub sender: Option<Address>,
    pub recipient: Option<Address>,
    pub solver: Option<Address>,
}
pub(crate) struct ChannelStateService {
    rx: watch::Receiver<AppState>,
}

impl ChannelStateService {
    pub fn new(rx: watch::Receiver<AppState>) -> Self {
        Self { rx }
    }
}

impl StateService for ChannelStateService {
    fn get_transactions(
        &self,
        filter: SwapTransactionQueryFilter,
    ) -> anyhow::Result<Vec<SwapTransaction>> {
        let mut state = self.rx.borrow().clone();

        if let Some(id) = filter.request_id {
            by_id(&mut state.transactions, &id)
        }
        if let Some(chain_id) = filter.chain_id {
            by_chain_id(&mut state.transactions, chain_id)
        }
        if let Some(address) = filter.address {
            by_address(&mut state.transactions, address)
        }
        if let Some(sender) = filter.sender {
            by_sender(&mut state.transactions, sender)
        }
        if let Some(recipient) = filter.recipient {
            by_recipient(&mut state.transactions, recipient)
        }
        if let Some(solver) = filter.solver {
            by_solver(&mut state.transactions, solver)
        }

        Ok(state.transactions)
    }
}

fn by_id(txs: &mut Vec<SwapTransaction>, request_id: &FixedBytes<32>) {
    txs.retain(|t| t.request_id == *request_id);
}
fn by_chain_id(txs: &mut Vec<SwapTransaction>, chain_id: ShortNumber) {
    txs.retain(|t| t.src_chain_id == chain_id || t.dest_chain_id == chain_id);
}

fn by_address(txs: &mut Vec<SwapTransaction>, address: Address) {
    txs.retain(|t| {
        t.sender == address
            || t.recipient == address
            || t.solver.filter(|s| s == &address).is_some()
    });
}
fn by_sender(txs: &mut Vec<SwapTransaction>, address: Address) {
    txs.retain(|t| t.sender == address)
}

fn by_recipient(txs: &mut Vec<SwapTransaction>, address: Address) {
    txs.retain(|t| t.recipient == address)
}

fn by_solver(txs: &mut Vec<SwapTransaction>, address: Address) {
    txs.retain(|t| t.solver.filter(|s| s == &address).is_some())
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloy::primitives::{Address, FixedBytes, U256, address, fixed_bytes};
    use tokio::sync::watch;

    #[test]
    fn filter_by_id() {
        let id1 =
            fixed_bytes!("0x1111111111111111111111111111111111111111111111111111111111111111");
        let id2 =
            fixed_bytes!("0x2222222222222222222222222222222222222222222222222222222222222222");

        let txs = vec![
            create_tx(
                id1,
                1u64.into(),
                2u64.into(),
                address!("0x17B3cAb3cD7502C6b85ed2E11Fd5988AF76C1111"),
                address!("0x17B3cAb3cD7502C6b85ed2E11Fd5988AF76C2222"),
                None,
            ),
            create_tx(
                id2,
                1u64.into(),
                2u64.into(),
                address!("0x17B3cAb3cD7502C6b85ed2E11Fd5988AF76C3333"),
                address!("0x17B3cAb3cD7502C6b85ed2E11Fd5988AF76C4444"),
                None,
            ),
        ];
        let service = create_service(txs);

        let filter = SwapTransactionQueryFilter {
            request_id: Some(id1),
            ..Default::default()
        };

        let result = service.get_transactions(filter).unwrap();
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].request_id, id1);
    }

    #[test]
    fn filter_by_chain_id() {
        let txs = vec![
            create_tx(
                FixedBytes::default(),
                1u64.into(),
                2u64.into(),
                address!("0x17B3cAb3cD7502C6b85ed2E11Fd5988AF76C1111"),
                address!("0x17B3cAb3cD7502C6b85ed2E11Fd5988AF76C2222"),
                None,
            ),
            create_tx(
                FixedBytes::default(),
                3u64.into(),
                3u64.into(), // same src/dest
                address!("0x17B3cAb3cD7502C6b85ed2E11Fd5988AF76C3333"),
                address!("0x17B3cAb3cD7502C6b85ed2E11Fd5988AF76C4444"),
                None,
            ),
        ];
        let service = create_service(txs);

        let filter = SwapTransactionQueryFilter {
            chain_id: Some(3u64.into()),
            ..Default::default()
        };

        let result = service.get_transactions(filter).unwrap();
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].src_chain_id, 3u64.into());
    }

    #[test]
    fn filter_by_sender() {
        let sender = address!("0x17B3cAb3cD7502C6b85ed2E11Fd5988AF76Caaaa");
        let other = address!("0x17B3cAb3cD7502C6b85ed2E11Fd5988AF76Cbbbb");
        let txs = vec![
            create_tx(
                FixedBytes::default(),
                1u64.into(),
                2u64.into(),
                sender,
                other,
                None,
            ),
            create_tx(
                FixedBytes::default(),
                1u64.into(),
                2u64.into(),
                other,
                sender,
                None,
            ),
        ];
        let service = create_service(txs);

        let filter = SwapTransactionQueryFilter {
            sender: Some(sender),
            ..Default::default()
        };

        let result = service.get_transactions(filter).unwrap();
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].sender, sender);
    }

    #[test]
    fn filter_by_recipient() {
        let recipient = address!("0x17B3cAb3cD7502C6b85ed2E11Fd5988AF76Ccccc");
        let other = address!("0x17B3cAb3cD7502C6b85ed2E11Fd5988AF76Cdddd");
        let txs = vec![
            create_tx(
                FixedBytes::default(),
                1u64.into(),
                2u64.into(),
                other,
                recipient,
                None,
            ),
            create_tx(
                FixedBytes::default(),
                1u64.into(),
                2u64.into(),
                recipient,
                other,
                None,
            ),
        ];
        let service = create_service(txs);

        let filter = SwapTransactionQueryFilter {
            recipient: Some(recipient),
            ..Default::default()
        };

        let result = service.get_transactions(filter).unwrap();
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].recipient, recipient);
    }

    #[test]
    fn filter_by_solver() {
        let solver = address!("0x17B3cAb3cD7502C6b85ed2E11Fd5988AF76Ceeee");
        let txs = vec![
            create_tx(
                FixedBytes::default(),
                1u64.into(),
                2u64.into(),
                address!("0x17B3cAb3cD7502C6b85ed2E11Fd5988AF76Caaaa"),
                address!("0x17B3cAb3cD7502C6b85ed2E11Fd5988AF76Cbbbb"),
                Some(solver),
            ),
            create_tx(
                FixedBytes::default(),
                1u64.into(),
                2u64.into(),
                address!("0x17B3cAb3cD7502C6b85ed2E11Fd5988AF76Ccccc"),
                address!("0x17B3cAb3cD7502C6b85ed2E11Fd5988AF76Cdddd"),
                None,
            ),
        ];
        let service = create_service(txs);

        let filter = SwapTransactionQueryFilter {
            solver: Some(solver),
            ..Default::default()
        };

        let result = service.get_transactions(filter).unwrap();
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].solver, Some(solver));
    }

    #[test]
    fn filter_by_address_matches_any_field() {
        let addr = address!("0x17B3cAb3cD7502C6b85ed2E11Fd5988AF76Cffff");
        let txs = vec![
            create_tx(
                FixedBytes::default(),
                1u64.into(),
                2u64.into(),
                addr,
                address!("0x17B3cAb3cD7502C6b85ed2E11Fd5988AF76C1111"),
                None,
            ),
            create_tx(
                FixedBytes::default(),
                1u64.into(),
                2u64.into(),
                address!("0x17B3cAb3cD7502C6b85ed2E11Fd5988AF76C2222"),
                addr,
                None,
            ),
            create_tx(
                FixedBytes::default(),
                1u64.into(),
                2u64.into(),
                address!("0x17B3cAb3cD7502C6b85ed2E11Fd5988AF76C3333"),
                address!("0x17B3cAb3cD7502C6b85ed2E11Fd5988AF76C4444"),
                Some(addr),
            ),
            create_tx(
                FixedBytes::default(),
                1u64.into(),
                2u64.into(),
                address!("0x17B3cAb3cD7502C6b85ed2E11Fd5988AF76C5555"),
                address!("0x17B3cAb3cD7502C6b85ed2E11Fd5988AF76C6666"),
                None,
            ),
        ];
        let service = create_service(txs);

        let filter = SwapTransactionQueryFilter {
            address: Some(addr),
            ..Default::default()
        };

        let result = service.get_transactions(filter).unwrap();
        assert_eq!(result.len(), 3);
    }

    fn create_tx(
        request_id: FixedBytes<32>,
        src_chain_id: ShortNumber,
        dest_chain_id: ShortNumber,
        sender: Address,
        recipient: Address,
        solver: Option<Address>,
    ) -> SwapTransaction {
        SwapTransaction {
            request_id,
            src_chain_id,
            dest_chain_id,
            sender,
            recipient,
            solver,
            amount_in: U256::ZERO.into(),
            amount_out: U256::ZERO.into(),
            verification_fee: U256::ZERO.into(),
            solver_fee: U256::ZERO.into(),
            state: "GreaT".to_string(),
            requested_time: U256::ZERO.into(),
            solved_time: Some(U256::ZERO.into()),
            verified_time: Some(U256::ZERO.into()),
        }
    }

    fn create_service(transactions: Vec<SwapTransaction>) -> ChannelStateService {
        let (_, rx) = watch::channel(AppState { transactions });
        ChannelStateService::new(rx)
    }
}
