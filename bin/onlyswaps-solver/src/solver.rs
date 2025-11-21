use crate::model::{ChainState, RequestId, Trade, Transfer};
use crate::util::normalise_chain_id;
use alloy::primitives::U256;
use async_trait::async_trait;
use generated::onlyswaps::i_router::IRouter::SwapRequestParametersWithHooks;
use moka::future::Cache;
use onlyswaps_client::FeeEstimate;
use std::collections::HashMap;

#[async_trait]
pub(crate) trait ChainStateProvider {
    async fn fetch_state(&self) -> anyhow::Result<ChainState>;
}

#[async_trait]
pub(crate) trait FeeAdapter {
    async fn fetch_fee(&self, fee: &Transfer) -> anyhow::Result<FeeEstimate>;
}

pub(crate) struct Solver<'a, CSP, FA> {
    states: HashMap<u64, ChainState>,
    chains: &'a HashMap<u64, CSP>,
    fee_estimator: &'a FA,
}

impl<'a, CSP: ChainStateProvider, FA: FeeAdapter> Solver<'a, CSP, FA> {
    pub async fn new(chains: &'a HashMap<u64, CSP>, fee_estimator: &'a FA) -> anyhow::Result<Self> {
        let mut states: HashMap<u64, ChainState> = HashMap::new();

        // fetch the initial state for each chain before we listen for blocks
        for (chain_id, chain) in chains {
            states.insert(*chain_id, chain.fetch_state().await?);
        }

        Ok(Self {
            states,
            chains,
            fee_estimator,
        })
    }

    pub async fn solve(
        &mut self,
        chain_id: u64,
        in_flight: &Cache<RequestId, ()>,
    ) -> anyhow::Result<Vec<Trade>> {
        let chain = self
            .chains
            .get(&chain_id)
            .expect("somehow got event for a non-existent chain");
        let updated_state = chain.fetch_state().await?;
        self.states.insert(chain_id, updated_state);
        let trades = self
            .solve_eligible_trades(chain_id, &self.states, in_flight)
            .await?;

        Ok(trades)
    }

    async fn solve_eligible_trades(
        &self,
        chain_id: u64,
        states: &HashMap<u64, ChainState>,
        in_flight: &Cache<RequestId, ()>,
    ) -> anyhow::Result<Vec<Trade>> {
        let mut trades = Vec::new();
        let mut owned_states = states.clone();
        // we only want the current chain's transactions, as we may have trades in flight for other chains
        let transfers = &states
            .get(&chain_id)
            .expect("somehow we got a block from a chain that doesn't have a state")
            .transfers;

        for transfer in transfers {
            if in_flight.contains_key(&transfer.request_id) {
                continue;
            }

            match self.fee_estimator.fetch_fee(transfer).await {
                Err(e) => {
                    tracing::error!(
                        request_id = %transfer.request_id,
                        src_chain_id = ?transfer.params.srcChainId,
                        dest_chain_id = ?transfer.params.dstChainId,
                        error = ?e,
                        "error fetching fees"
                    )
                }
                Ok(estimated_fees) => {
                    update_trades(transfer, &estimated_fees, &mut trades, &mut owned_states)
                }
            }
        }

        Ok(trades)
    }
}

fn update_trades(
    transfer_request: &Transfer,
    estimated_fees: &FeeEstimate,
    trades: &mut Vec<Trade>,
    states: &mut HashMap<u64, ChainState>,
) {
    let SwapRequestParametersWithHooks {
        dstChainId,
        amountOut,
        solverFee,
        executed,
        ..
    } = transfer_request.params;

    let dest_state = match states.get_mut(&normalise_chain_id(dstChainId)) {
        None => return,
        Some(state) => state,
    };

    if executed {
        tracing::debug!(request_id = %transfer_request.request_id, "skipping - tx already executed");
        return;
    }

    if dest_state
        .already_fulfilled
        .contains(&transfer_request.request_id)
    {
        tracing::debug!(request_id = %transfer_request.request_id, "skipping - tx already fulfilled");
        return;
    }

    if dest_state.native_balance == U256::from(0) {
        tracing::debug!(request_id = %transfer_request.request_id, "skipping - native balance too low");
        return;
    }

    let token_balance = match dest_state
        .token_balances
        .get(&transfer_request.params.tokenOut)
    {
        None => {
            tracing::debug!(request_id = %transfer_request.request_id, "skipping - token balance empty");
            return;
        }
        Some(balance) => balance,
    };
    if *token_balance < amountOut {
        tracing::debug!(request_id = %transfer_request.request_id, "skipping - token balance too low");
        return;
    }

    if amountOut > estimated_fees.amount_out {
        tracing::debug!(request_id = %transfer_request.request_id, "skipping - amount_out higher than recommended by fees API");
        return;
    }

    // just takes a flat fee for the moment
    if solverFee < U256::from(1) {
        tracing::debug!(request_id = %transfer_request.request_id, "skipping - fee too low");
        return;
    }

    // we commit some of our tokens to this trade so the next one doesn't fail
    dest_state
        .token_balances
        .insert(transfer_request.params.tokenOut, token_balance - amountOut);
    trades.push(transfer_request.into())
}

#[cfg(test)]
mod tests {
    use crate::model::{ChainState, Trade, Transfer};
    use crate::solver::{ChainStateProvider, FeeAdapter, Solver};
    use crate::util::test::{generate_address, generate_request_id};
    use alloy::primitives::{Address, U256, address};
    use async_trait::async_trait;
    use generated::onlyswaps::i_router::IRouter::SwapRequestParametersWithHooks;
    use moka::future::Cache;
    use onlyswaps_client::fee_estimator::ChainFeeDetails;
    use onlyswaps_client::{FeeBreakdown, FeeEstimate};
    use speculoos::assert_that;
    use speculoos::vec::VecAssertions;
    use std::collections::HashMap;

    static USER_ADDR: Address = address!("0xdeadbeef6964af9d7eed9e03e53415d37aa96045");
    static TOKEN_ADDR: Address = address!("0xd8da6bf26964af9d7eed9e03e53415d37aa96045");

    #[tokio::test]
    async fn transfers_created_through_solver_create_trades() {
        // given
        let chain_id = 1;
        let transfer_params = create_transfer_params(USER_ADDR, 1, 2, 100);
        let chain_one_state = ChainState {
            token_balances: HashMap::from([(TOKEN_ADDR, U256::from(1))]),
            native_balance: U256::from(1),
            transfers: vec![transfer_params.clone()],
            already_fulfilled: vec![],
        };
        let chain_two_state = ChainState {
            token_balances: HashMap::from([(TOKEN_ADDR, U256::from(1000))]),
            native_balance: U256::from(100),
            transfers: Vec::default(),
            already_fulfilled: vec![],
        };
        let chain_one = StubbedChain {
            state: chain_one_state,
        };
        let chain_two = StubbedChain {
            state: chain_two_state,
        };
        let chains = HashMap::from([(1, chain_one), (2, chain_two)]);
        let fee_estimater = StubbedFees {
            response: create_fee_estimate(transfer_params.params.amountIn).clone(),
        };

        // when
        let mut solver = Solver::new(&chains, &fee_estimater).await.unwrap();
        let trades = solver.solve(chain_id, &Cache::new(1)).await.unwrap();

        // then
        let expected_output_amount = transfer_params.params.amountOut;
        let expected_trade = Trade {
            request_id: transfer_params.request_id,
            nonce: transfer_params.params.nonce,
            token_in_addr: transfer_params.params.tokenIn,
            token_out_addr: transfer_params.params.tokenOut,
            src_chain_id: transfer_params.params.srcChainId,
            dest_chain_id: transfer_params.params.dstChainId,
            sender_addr: transfer_params.params.sender,
            recipient_addr: transfer_params.params.recipient,
            amount_in: expected_output_amount,
            amount_out: expected_output_amount,
            solver_fee: transfer_params.params.solverFee,
            pre_hooks: Vec::new(),
            post_hooks: Vec::new(),
        };
        assert_that!(trades).has_length(1);
        assert_that!(trades[0]).is_equal_to(expected_trade);
    }

    #[tokio::test]
    async fn multiple_transfers_create_multiple_trades() {
        // given
        // both transfers use 100
        let transfer_params = create_transfer_params(USER_ADDR, 1, 2, 100);
        let transfer_params_2 = create_transfer_params(USER_ADDR, 1, 2, 100);

        let src_chain_state = ChainState {
            token_balances: HashMap::from([(TOKEN_ADDR, U256::from(0))]),
            native_balance: U256::from(0),
            transfers: vec![transfer_params.clone(), transfer_params_2.clone()],
            already_fulfilled: vec![],
        };
        // on dst_chain, we only have enough balance to cover one tx
        let dst_chain_state = ChainState {
            token_balances: HashMap::from([(TOKEN_ADDR, U256::from(200))]),
            native_balance: U256::from(1000),
            transfers: vec![],
            already_fulfilled: vec![],
        };
        let chain_one = StubbedChain {
            state: src_chain_state,
        };
        let chain_two = StubbedChain {
            state: dst_chain_state,
        };
        let chains = HashMap::from([(1, chain_one), (2, chain_two)]);
        let fee_estimater = StubbedFees {
            response: create_fee_estimate(transfer_params.params.amountIn).clone(),
        };
        let mut solver = Solver::new(&chains, &fee_estimater).await.unwrap();

        // when
        let trades = solver.solve(1, &Cache::new(1)).await.unwrap();

        // then
        assert_that!(trades).has_length(2);
    }

    #[tokio::test]
    async fn transfers_across_multiple_chains_only_create_trades_for_src_chain() {
        // given
        // both transfers use 100
        let transfer_params = create_transfer_params(USER_ADDR, 1, 2, 100);
        let transfer_params_2 = create_transfer_params(USER_ADDR, 1, 2, 100);

        let src_chain_state = ChainState {
            token_balances: HashMap::from([(TOKEN_ADDR, U256::from(100))]),
            native_balance: U256::from(1000),
            transfers: vec![transfer_params.clone()],
            already_fulfilled: vec![],
        };
        // on dst_chain, we only have enough balance to cover one tx
        let dst_chain_state = ChainState {
            token_balances: HashMap::from([(TOKEN_ADDR, U256::from(200))]),
            native_balance: U256::from(1000),
            transfers: vec![transfer_params_2.clone()],
            already_fulfilled: vec![],
        };
        let chain_one = StubbedChain {
            state: src_chain_state,
        };
        let chain_two = StubbedChain {
            state: dst_chain_state,
        };
        let chains = HashMap::from([(1, chain_one), (2, chain_two)]);
        let fee_estimater = StubbedFees {
            response: create_fee_estimate(transfer_params.params.amountIn).clone(),
        };
        let mut solver = Solver::new(&chains, &fee_estimater).await.unwrap();

        // when
        let trades = solver.solve(1, &Cache::new(1)).await.unwrap();

        // then
        assert_that!(trades).has_length(1);
    }

    #[tokio::test]
    async fn no_transfers_creates_no_trades() {
        // given
        let src_chain_state = ChainState {
            token_balances: HashMap::from([(TOKEN_ADDR, U256::from(0))]),
            native_balance: U256::from(0),
            transfers: vec![],
            already_fulfilled: vec![],
        };
        let dst_chain_state = ChainState {
            token_balances: HashMap::from([(TOKEN_ADDR, U256::from(1000))]),
            native_balance: U256::from(0),
            transfers: vec![],
            already_fulfilled: vec![],
        };
        let chain_one = StubbedChain {
            state: src_chain_state,
        };
        let chain_two = StubbedChain {
            state: dst_chain_state,
        };
        let chains = HashMap::from([(1, chain_one), (2, chain_two)]);
        let fee_estimater = StubbedFees {
            response: create_fee_estimate(U256::from(0)).clone(),
        };
        let mut solver = Solver::new(&chains, &fee_estimater).await.unwrap();

        // when
        let trades = solver.solve(1, &Cache::new(1)).await.unwrap();

        // then
        assert_that!(trades).has_length(0);
    }

    #[tokio::test]
    async fn no_native_currency_on_dest_chain_doesnt_trade() {
        // given
        let src_chain_state = ChainState {
            token_balances: HashMap::from([(TOKEN_ADDR, U256::from(0))]),
            native_balance: U256::from(0),
            transfers: vec![create_transfer_params(USER_ADDR, 1, 2, 100)],
            already_fulfilled: vec![],
        };
        let dst_chain_state = ChainState {
            token_balances: HashMap::from([(TOKEN_ADDR, U256::from(1000))]),
            native_balance: U256::from(0),
            transfers: vec![],
            already_fulfilled: vec![],
        };
        let chain_one = StubbedChain {
            state: src_chain_state,
        };
        let chain_two = StubbedChain {
            state: dst_chain_state,
        };
        let chains = HashMap::from([(1, chain_one), (2, chain_two)]);
        let fee_estimater = StubbedFees {
            response: create_fee_estimate(U256::from(1)).clone(),
        };
        let mut solver = Solver::new(&chains, &fee_estimater).await.unwrap();

        // when
        let trades = solver.solve(1, &Cache::new(1)).await.unwrap();

        // then
        assert_that!(trades).has_length(0);
    }

    #[tokio::test]
    async fn no_token_balance_doesnt_trade() {
        // given
        let src_chain_state = ChainState {
            token_balances: HashMap::from([(TOKEN_ADDR, U256::from(0))]),
            native_balance: U256::from(0),
            transfers: vec![create_transfer_params(USER_ADDR, 1, 2, 100)],
            already_fulfilled: vec![],
        };
        let dst_chain_state = ChainState {
            token_balances: HashMap::from([(TOKEN_ADDR, U256::from(0))]),
            native_balance: U256::from(1000),
            transfers: vec![],
            already_fulfilled: vec![],
        };
        let chain_one = StubbedChain {
            state: src_chain_state,
        };
        let chain_two = StubbedChain {
            state: dst_chain_state,
        };
        let chains = HashMap::from([(1, chain_one), (2, chain_two)]);
        let fee_estimater = StubbedFees {
            response: create_fee_estimate(U256::from(0)).clone(),
        };
        let mut solver = Solver::new(&chains, &fee_estimater).await.unwrap();

        // when
        let trades = solver.solve(1, &Cache::new(1)).await.unwrap();

        // then
        assert_that!(trades).has_length(0);
    }

    #[tokio::test]
    async fn already_executed_doesnt_create_tx() {
        // given
        let mut transfer_params = create_transfer_params(USER_ADDR, 1, 2, 100);
        transfer_params.params.executed = true;

        let src_chain_state = ChainState {
            token_balances: HashMap::from([(TOKEN_ADDR, U256::from(0))]),
            native_balance: U256::from(0),
            transfers: vec![transfer_params.clone()],
            already_fulfilled: vec![],
        };
        let dst_chain_state = ChainState {
            token_balances: HashMap::from([(TOKEN_ADDR, U256::from(1000))]),
            native_balance: U256::from(1000),
            transfers: vec![],
            already_fulfilled: vec![],
        };
        let chain_one = StubbedChain {
            state: src_chain_state,
        };
        let chain_two = StubbedChain {
            state: dst_chain_state,
        };
        let chains = HashMap::from([(1, chain_one), (2, chain_two)]);
        let fee_estimater = StubbedFees {
            response: create_fee_estimate(transfer_params.params.amountIn).clone(),
        };
        let mut solver = Solver::new(&chains, &fee_estimater).await.unwrap();

        // when
        let trades = solver.solve(1, &Cache::new(1)).await.unwrap();

        // then
        assert_that!(trades).has_length(0);
    }

    #[tokio::test]
    async fn no_fee_gives_no_trade() {
        // given
        let mut transfer_params = create_transfer_params(USER_ADDR, 1, 2, 100);
        transfer_params.params.solverFee = U256::from(0);

        let src_chain_state = ChainState {
            token_balances: HashMap::from([(TOKEN_ADDR, U256::from(0))]),
            native_balance: U256::from(0),
            transfers: vec![transfer_params.clone()],
            already_fulfilled: vec![],
        };
        let dst_chain_state = ChainState {
            token_balances: HashMap::from([(TOKEN_ADDR, U256::from(1000))]),
            native_balance: U256::from(1000),
            transfers: vec![],
            already_fulfilled: vec![],
        };
        let chain_one = StubbedChain {
            state: src_chain_state,
        };
        let chain_two = StubbedChain {
            state: dst_chain_state,
        };
        let chains = HashMap::from([(1, chain_one), (2, chain_two)]);
        let fee_estimater = StubbedFees {
            response: create_fee_estimate(transfer_params.params.amountIn).clone(),
        };
        let mut solver = Solver::new(&chains, &fee_estimater).await.unwrap();

        // when
        let trades = solver.solve(1, &Cache::new(1)).await.unwrap();

        // then
        assert_that!(trades).has_length(0);
    }

    #[tokio::test]
    async fn invalid_token_addr_gives_no_trade() {
        // given
        let mut transfer_params = create_transfer_params(USER_ADDR, 1, 2, 100);
        transfer_params.params.tokenOut = generate_address();

        let src_chain_state = ChainState {
            token_balances: HashMap::from([(TOKEN_ADDR, U256::from(0))]),
            native_balance: U256::from(0),
            transfers: vec![transfer_params.clone()],
            already_fulfilled: vec![],
        };
        let dst_chain_state = ChainState {
            token_balances: HashMap::from([(TOKEN_ADDR, U256::from(1000))]),
            native_balance: U256::from(1000),
            transfers: vec![],
            already_fulfilled: vec![],
        };
        let chain_one = StubbedChain {
            state: src_chain_state,
        };
        let chain_two = StubbedChain {
            state: dst_chain_state,
        };
        let chains = HashMap::from([(1, chain_one), (2, chain_two)]);
        let fee_estimater = StubbedFees {
            response: create_fee_estimate(transfer_params.params.amountIn).clone(),
        };
        let mut solver = Solver::new(&chains, &fee_estimater).await.unwrap();

        // when
        let trades = solver.solve(1, &Cache::new(1)).await.unwrap();

        // then
        assert_that!(trades).has_length(0);
    }

    #[tokio::test]
    async fn subsequent_calls_dont_use_same_balance() {
        // given
        // both transfers use 100
        let transfer_params = create_transfer_params(USER_ADDR, 1, 2, 100);
        let transfer_params_2 = create_transfer_params(USER_ADDR, 1, 2, 100);

        let src_chain_state = ChainState {
            token_balances: HashMap::from([(TOKEN_ADDR, U256::from(0))]),
            native_balance: U256::from(0),
            transfers: vec![transfer_params.clone(), transfer_params_2.clone()],
            already_fulfilled: vec![],
        };
        // on dst_chain, we only have enough balance to cover one tx
        let dst_chain_state = ChainState {
            token_balances: HashMap::from([(TOKEN_ADDR, U256::from(150))]),
            native_balance: U256::from(1000),
            transfers: vec![],
            already_fulfilled: vec![],
        };
        let chain_one = StubbedChain {
            state: src_chain_state,
        };
        let chain_two = StubbedChain {
            state: dst_chain_state,
        };
        let chains = HashMap::from([(1, chain_one), (2, chain_two)]);
        let fee_estimater = StubbedFees {
            response: create_fee_estimate(transfer_params.params.amountIn).clone(),
        };
        let mut solver = Solver::new(&chains, &fee_estimater).await.unwrap();

        // when
        let trades = solver.solve(1, &Cache::new(1)).await.unwrap();

        // then
        assert_that!(trades).has_length(1);
    }

    #[tokio::test]
    async fn transfers_that_have_already_been_fulfilled_dont_make_trades() {
        // given
        // both transfers use 100
        let transfer_params = create_transfer_params(USER_ADDR, 1, 2, 100);

        let src_chain_state = ChainState {
            token_balances: HashMap::from([(TOKEN_ADDR, U256::from(0))]),
            native_balance: U256::from(0),
            transfers: vec![transfer_params.clone()],
            already_fulfilled: vec![],
        };
        // on dst_chain, we only have enough balance to cover one tx
        let dst_chain_state = ChainState {
            token_balances: HashMap::from([(TOKEN_ADDR, U256::from(150))]),
            native_balance: U256::from(1000),
            transfers: vec![],
            already_fulfilled: vec![transfer_params.request_id],
        };
        let chain_one = StubbedChain {
            state: src_chain_state,
        };
        let chain_two = StubbedChain {
            state: dst_chain_state,
        };
        let chains = HashMap::from([(1, chain_one), (2, chain_two)]);
        let fee_estimater = StubbedFees {
            response: create_fee_estimate(transfer_params.params.amountIn).clone(),
        };
        let mut solver = Solver::new(&chains, &fee_estimater).await.unwrap();

        // when
        let trades = solver.solve(1, &Cache::new(1)).await.unwrap();

        // then
        assert_that!(trades).has_length(0);
    }

    #[tokio::test]
    async fn transfer_that_exist_in_cache_dont_make_trades() {
        // given
        // transfer use 100
        let transfer_params = create_transfer_params(USER_ADDR, 1, 2, 100);

        let src_chain_state = ChainState {
            token_balances: HashMap::from([(TOKEN_ADDR, U256::from(0))]),
            native_balance: U256::from(0),
            transfers: vec![transfer_params.clone()],
            already_fulfilled: vec![],
        };
        let dst_chain_state = ChainState {
            token_balances: HashMap::from([(TOKEN_ADDR, U256::from(200))]),
            native_balance: U256::from(1000),
            transfers: vec![],
            already_fulfilled: vec![],
        };
        // we create a cache that already has the request_id in it
        let cache = Cache::new(1);
        let id = transfer_params.clone().request_id;
        cache.insert(id, ()).await;
        let chain_one = StubbedChain {
            state: src_chain_state,
        };
        let chain_two = StubbedChain {
            state: dst_chain_state,
        };
        let chains = HashMap::from([(1, chain_one), (2, chain_two)]);
        let fee_estimater = StubbedFees {
            response: create_fee_estimate(transfer_params.params.amountIn).clone(),
        };
        let mut solver = Solver::new(&chains, &fee_estimater).await.unwrap();

        // when
        let trades = solver.solve(1, &cache).await.unwrap();

        // then
        assert_that!(trades).has_length(0);
    }

    #[tokio::test]
    async fn transfers_for_chains_not_existing_in_map_dont_make_trades() {
        let transfer_params = create_transfer_params(USER_ADDR, 1, 2, 100);
        let some_other_token = address!("0x00006bf26964af9d7eed9e03e53415d37aa96045");
        let src_chain_state = ChainState {
            token_balances: HashMap::from([(some_other_token, U256::from(200))]),
            native_balance: U256::from(0),
            transfers: vec![transfer_params.clone()],
            already_fulfilled: vec![],
        };

        let dst_chain_state = ChainState {
            token_balances: HashMap::from([(some_other_token, U256::from(200))]),
            native_balance: U256::from(1000),
            transfers: vec![],
            already_fulfilled: vec![],
        };

        let chain_one = StubbedChain {
            state: src_chain_state,
        };
        let chain_two = StubbedChain {
            state: dst_chain_state,
        };
        let chains = HashMap::from([(1, chain_one), (2, chain_two)]);
        let fee_estimater = StubbedFees {
            response: create_fee_estimate(transfer_params.params.amountIn).clone(),
        };
        let mut solver = Solver::new(&chains, &fee_estimater).await.unwrap();

        // when
        let trades = solver.solve(1, &Cache::new(1)).await.unwrap();

        // then
        assert_that!(trades).has_length(0);
    }

    #[tokio::test]
    async fn transfers_with_mix_of_tokens_make_trades() {
        //given
        let some_other_token = address!("0x00006bf26964af9d7eed9e03e53415d37aa96045");
        let transfer_params = create_transfer_params(USER_ADDR, 1, 2, 100);
        let mut transfer_params_2 = create_transfer_params(USER_ADDR, 1, 2, 100);
        transfer_params_2.params.tokenOut = some_other_token;

        let src_chain_state = ChainState {
            token_balances: HashMap::from([(TOKEN_ADDR, U256::from(0))]),
            native_balance: U256::from(0),
            transfers: vec![transfer_params.clone(), transfer_params_2.clone()],
            already_fulfilled: vec![],
        };
        // on dst_chain, we only have enough balance to cover one tx
        let dst_chain_state = ChainState {
            token_balances: HashMap::from([(TOKEN_ADDR, U256::from(200))]),
            native_balance: U256::from(1000),
            transfers: vec![],
            already_fulfilled: vec![],
        };
        let chain_one = StubbedChain {
            state: src_chain_state,
        };
        let chain_two = StubbedChain {
            state: dst_chain_state,
        };
        let chains = HashMap::from([(1, chain_one), (2, chain_two)]);
        let fee_estimater = StubbedFees {
            response: create_fee_estimate(transfer_params.params.amountIn).clone(),
        };
        let mut solver = Solver::new(&chains, &fee_estimater).await.unwrap();
        // when
        let trades = solver.solve(1, &Cache::new(1)).await.unwrap();

        // then
        assert_that!(trades).has_length(1);
    }

    #[tokio::test]
    async fn transfer_with_higher_amount_out_than_fee_makes_no_trade() {
        // given
        let transfer_params = create_transfer_params(USER_ADDR, 1, 2, 100);
        // fee estimate suggests we should get 100000 out, not 100, so the solver should reject it
        let mut fee = create_fee_estimate(transfer_params.params.amountIn + U256::from(100));
        fee.amount_out = U256::from(1);
        let fee_estimater = StubbedFees {
            response: fee.clone(),
        };

        let src_chain_state = ChainState {
            token_balances: HashMap::from([(TOKEN_ADDR, U256::from(0))]),
            native_balance: U256::from(0),
            transfers: vec![transfer_params.clone()],
            already_fulfilled: vec![],
        };
        let dst_chain_state = ChainState {
            token_balances: HashMap::from([(TOKEN_ADDR, U256::from(1000))]),
            native_balance: U256::from(1000),
            transfers: vec![],
            already_fulfilled: vec![],
        };
        let chain_one = StubbedChain {
            state: src_chain_state,
        };
        let chain_two = StubbedChain {
            state: dst_chain_state,
        };
        let chains = HashMap::from([(1, chain_one), (2, chain_two)]);
        let mut solver = Solver::new(&chains, &fee_estimater).await.unwrap();

        // when
        let trades = solver.solve(1, &Cache::new(1)).await.unwrap();

        // then
        assert_that!(trades).has_length(0);
    }

    fn create_transfer_params(
        sender: Address,
        src_chain_id: u64,
        dest_chain_id: u64,
        amount: u64,
    ) -> Transfer {
        Transfer {
            request_id: generate_request_id(),
            params: SwapRequestParametersWithHooks {
                srcChainId: U256::from(src_chain_id),
                dstChainId: U256::from(dest_chain_id),
                sender,
                recipient: sender,
                tokenIn: TOKEN_ADDR,
                tokenOut: TOKEN_ADDR,
                amountIn: U256::from(amount),
                amountOut: U256::from(amount),
                verificationFee: U256::from(2),
                solverFee: U256::from(1),
                nonce: U256::from(100),
                executed: false,
                requestedAt: U256::from(12345),
                preHooks: Vec::new(),
                postHooks: Vec::new(),
            },
        }
    }

    fn create_fee_estimate(amount_in: U256) -> FeeEstimate {
        FeeEstimate {
            src: ChainFeeDetails {
                currency: "RUSD".to_string(),
                decimals: 18,
                swap_fee: "1000000".to_string(),
                relay_fee: "100000".to_string(),
                bps: 50,
                bps_divisor: 10_000,
            },
            dest: ChainFeeDetails {
                currency: "FUSD".to_string(),
                decimals: 18,
                swap_fee: "1000000".to_string(),
                relay_fee: "100000".to_string(),
                bps: 50,
                bps_divisor: 10_000,
            },
            fees: FeeBreakdown {
                solver: U256::from(111),
                network: U256::from(111),
                total: U256::from(111),
            },
            transfer_amount: amount_in,
            approval_amount: amount_in,
            amount_out: amount_in,
            timestamp: 0,
        }
    }

    struct StubbedChain {
        state: ChainState,
    }

    #[async_trait]
    impl ChainStateProvider for StubbedChain {
        async fn fetch_state(&self) -> anyhow::Result<ChainState> {
            Ok(self.state.clone())
        }
    }

    #[derive(Clone)]
    struct StubbedFees {
        response: FeeEstimate,
    }
    #[async_trait]
    impl FeeAdapter for StubbedFees {
        async fn fetch_fee(&self, _: &Transfer) -> anyhow::Result<FeeEstimate> {
            Ok(self.response.clone())
        }
    }
}
