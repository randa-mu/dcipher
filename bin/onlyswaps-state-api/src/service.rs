use crate::filter::matches;
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

#[derive(Debug, Default, Deserialize, Clone)]
pub(crate) struct SwapTransactionQueryFilter {
    pub request_id: Option<FixedBytes<32>>,
    pub chain_id: Option<ShortNumber>,
    pub address: Option<Address>,
    pub sender: Option<Address>,
    pub recipient: Option<Address>,
    pub solver: Option<Address>,
    pub requested_time_start: Option<u64>,
    pub requested_time_end: Option<u64>,
    pub verified_time_start: Option<u64>,
    pub verified_time_end: Option<u64>,
    pub limit: Option<usize>,
    pub offset: Option<usize>,
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
        let state = self.rx.borrow();
        let offset = filter.offset.unwrap_or(0);
        let limit = filter.limit.unwrap_or(100);

        let result = state
            .transactions
            .iter()
            .filter(|tx| matches(tx, filter.clone()))
            .skip(offset)
            .take(limit)
            .cloned()
            .collect();

        Ok(result)
    }
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

    #[test]
    fn filter_by_limit_smaller_all() {
        let addr = address!("0x17B3cAb3cD7502C6b85ed2E11Fd5988AF76Cffff");
        let tx1 = create_tx(
            FixedBytes::default(),
            1u64.into(),
            2u64.into(),
            addr,
            address!("0x17B3cAb3cD7502C6b85ed2E11Fd5988AF76C1111"),
            None,
        );
        let txs = vec![
            tx1.clone(),
            create_tx(
                FixedBytes::default(),
                2u64.into(),
                3u64.into(),
                address!("0x17B3cAb3cD7502C6b85ed2E11Fd5988AF76C2222"),
                addr,
                None,
            ),
            create_tx(
                FixedBytes::default(),
                4u64.into(),
                5u64.into(),
                address!("0x17B3cAb3cD7502C6b85ed2E11Fd5988AF76C3333"),
                address!("0x17B3cAb3cD7502C6b85ed2E11Fd5988AF76C4444"),
                Some(addr),
            ),
            create_tx(
                FixedBytes::default(),
                6u64.into(),
                7u64.into(),
                address!("0x17B3cAb3cD7502C6b85ed2E11Fd5988AF76C5555"),
                address!("0x17B3cAb3cD7502C6b85ed2E11Fd5988AF76C6666"),
                None,
            ),
        ];

        let service = create_service(txs);

        // take the first
        let filter1 = SwapTransactionQueryFilter {
            limit: Some(1),
            ..Default::default()
        };
        let result = service.get_transactions(filter1).unwrap();
        assert_eq!(result.len(), 1);

        // take them alllll
        let filter_all = SwapTransactionQueryFilter {
            limit: Some(20),
            ..Default::default()
        };
        let result_all = service.get_transactions(filter_all).unwrap();
        assert_eq!(result_all.len(), 4);

        // window including some tx but not the first
        let filter_window = SwapTransactionQueryFilter {
            limit: Some(2),
            offset: Some(1),
            ..Default::default()
        };
        let result_window = service.get_transactions(filter_window).unwrap();
        assert_eq!(result_window.len(), 2);
        assert_ne!(result_window[0], tx1);
        assert_ne!(result_window[1], tx1);

        // window too long returns none and doesn't blow up
        let filter_long_offset = SwapTransactionQueryFilter {
            limit: Some(2),
            offset: Some(100),
            ..Default::default()
        };
        let result_window = service.get_transactions(filter_long_offset).unwrap();
        assert_eq!(result_window.len(), 0);
    }

    #[test]
    fn filter_by_requested_time_range() {
        let mut txs = vec![
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
                1u64.into(),
                2u64.into(),
                address!("0x17B3cAb3cD7502C6b85ed2E11Fd5988AF76C3333"),
                address!("0x17B3cAb3cD7502C6b85ed2E11Fd5988AF76C4444"),
                None,
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
        txs[0].requested_time = 10u64.into();
        txs[1].requested_time = 20u64.into();
        txs[2].requested_time = 30u64.into();

        let service = create_service(txs);

        // inclusive range 10..=20
        let filter = SwapTransactionQueryFilter {
            requested_time_start: Some(10u64),
            requested_time_end: Some(20u64),
            ..Default::default()
        };

        let result = service.get_transactions(filter).unwrap();
        assert_eq!(result.len(), 2);
        assert_eq!(result[0].requested_time, 10u64.into());
        assert_eq!(result[1].requested_time, 20u64.into());
    }

    #[test]
    fn filter_by_verified_time_range() {
        let mut txs = vec![
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
                1u64.into(),
                2u64.into(),
                address!("0x17B3cAb3cD7502C6b85ed2E11Fd5988AF76C3333"),
                address!("0x17B3cAb3cD7502C6b85ed2E11Fd5988AF76C4444"),
                None,
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
        txs[0].verified_time = Some(5u64.into());
        txs[1].verified_time = Some(15u64.into());
        txs[2].verified_time = Some(25u64.into());

        let service = create_service(txs);

        // inclusive range 5..=15
        let filter = SwapTransactionQueryFilter {
            verified_time_start: Some(5u64),
            verified_time_end: Some(15u64),
            ..Default::default()
        };

        let result = service.get_transactions(filter).unwrap();
        assert_eq!(result.len(), 2);
        assert_eq!(result[0].verified_time, Some(5u64.into()));
        assert_eq!(result[1].verified_time, Some(15u64.into()));
    }

    #[test]
    fn filter_requested_time_defaults_to_full_range() {
        let mut txs = vec![
            create_tx(
                FixedBytes::default(),
                1u64.into(),
                2u64.into(),
                address!("0x17B3cAb3cD7502C6b85ed2E11Fd5988AF76Caaaa"),
                address!("0x17B3cAb3cD7502C6b85ed2E11Fd5988AF76Cbbbb"),
                None,
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
        txs[0].requested_time = 50u64.into();
        txs[1].requested_time = 100u64.into();
        let service = create_service(txs);

        // With no range set, both are included
        let filter = SwapTransactionQueryFilter::default();
        let result = service.get_transactions(filter).unwrap();
        assert_eq!(result.len(), 2);
    }

    #[test]
    fn filter_verified_time_start_only() {
        let mut txs = vec![
            create_tx(
                FixedBytes::default(),
                1u64.into(),
                2u64.into(),
                address!("0x17B3cAb3cD7502C6b85ed2E11Fd5988AF76Caaaa"),
                address!("0x17B3cAb3cD7502C6b85ed2E11Fd5988AF76Cbbbb"),
                None,
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
        txs[0].verified_time = Some(10u64.into());
        txs[1].verified_time = Some(20u64.into());
        let service = create_service(txs);

        // start only (>= 15)
        let filter = SwapTransactionQueryFilter {
            verified_time_start: Some(15u64),
            ..Default::default()
        };
        let result = service.get_transactions(filter).unwrap();
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].verified_time, Some(20u64.into()));
    }

    #[test]
    fn filter_verified_time_end_only() {
        let mut txs = vec![
            create_tx(
                FixedBytes::default(),
                1u64.into(),
                2u64.into(),
                address!("0x17B3cAb3cD7502C6b85ed2E11Fd5988AF76Caaaa"),
                address!("0x17B3cAb3cD7502C6b85ed2E11Fd5988AF76Cbbbb"),
                None,
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
        txs[0].verified_time = Some(10u64.into());
        txs[1].verified_time = Some(20u64.into());
        let service = create_service(txs);

        // end only (<= 15)
        let filter = SwapTransactionQueryFilter {
            verified_time_end: Some(15u64),
            ..Default::default()
        };
        let result = service.get_transactions(filter).unwrap();
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].verified_time, Some(10u64.into()));
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
            token_in: address!("0x1b0f6cf6f3185872a581bd2b5a738eb52ccd4d76"),
            token_out: address!("0x1b0f6cf6f3185872a581bd2b5a738eb52ccd4d76"),
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
