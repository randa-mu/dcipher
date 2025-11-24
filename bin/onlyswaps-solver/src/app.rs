use crate::executor::TradeExecutor;
use crate::fee_adapter::DefaultFeeAdapter;
use crate::model::{BlockEvent, RequestId};
use crate::network::Network;
use crate::price_feed::coingecko::CoinGeckoClient;
use crate::profitability::StdProfitabilityEstimator;
use crate::solver::Solver;
use alloy::providers::DynProvider;
use config::timeout::TimeoutConfig;
use futures::StreamExt;
use futures::future::try_join_all;
use futures::stream::select_all;
use moka::future::Cache;
use std::collections::HashMap;
use std::ops::Mul;

pub struct App {}
impl App {
    pub async fn start(
        networks: HashMap<u64, Network<DynProvider>>,
        timeout: &TimeoutConfig,
    ) -> anyhow::Result<()> {
        let mut cg_price_feed = CoinGeckoClient::builder().use_demo_api().build()?;
        cg_price_feed.init_chain_id_mapping().await?;
        let profitability_estimator = StdProfitabilityEstimator::new(cg_price_feed);

        let block_numbers = networks
            .values()
            .map(|network| network.stream_block_numbers());
        let streams = try_join_all(block_numbers).await?;
        let mut stream = Box::pin(select_all(streams));
        let fee_estimator = DefaultFeeAdapter::new();
        let mut solver = Solver::new(&networks, &fee_estimator).await?;
        let executor = TradeExecutor::new(&networks, profitability_estimator);

        // we pull new chain state every block, so inflight requests may not have been
        // completed yet, so we don't want to attempt to execute them again and waste gas.
        // if they're still there after twice the request timeout we can reattempt
        let mut inflight_requests: Cache<RequestId, ()> = Cache::builder()
            .max_capacity(1000)
            .time_to_live(timeout.request_timeout.mul(2))
            .build();

        while let Some(BlockEvent { chain_id, .. }) = stream.next().await {
            let trades = solver.solve(chain_id, &inflight_requests).await?;
            if !trades.is_empty() {
                tracing::info!(
                    chain_id = chain_id,
                    trade_count = trades.len(),
                    "executing trades "
                );
                executor
                    .execute(trades, &mut inflight_requests, timeout)
                    .await;
            }
        }

        anyhow::bail!("stream of blocks ended unexpectedly");
    }
}
