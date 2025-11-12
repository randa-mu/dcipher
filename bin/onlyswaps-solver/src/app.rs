use crate::executor::TradeExecutor;
use crate::model::{BlockEvent, RequestId};
use crate::network::Network;
use crate::solver::Solver;
use alloy::providers::DynProvider;
use futures::StreamExt;
use futures::future::try_join_all;
use futures::stream::select_all;
use moka::future::Cache;
use std::collections::HashMap;
use std::time::Duration;

pub struct App {}
impl App {
    pub async fn start(networks: HashMap<u64, Network<DynProvider>>) -> anyhow::Result<()> {
        let block_numbers = networks
            .values()
            .map(|network| network.stream_block_numbers());
        let streams = try_join_all(block_numbers).await?;
        let mut stream = Box::pin(select_all(streams));
        let mut solver = Solver::from(&networks).await?;
        let executor = TradeExecutor::new(&networks);

        // we pull new chain state every block, so inflight requests may not have been
        // completed yet, so we don't want to attempt to execute them again and waste gas.
        // if they're still there after 120s we can reattempt
        let mut inflight_requests: Cache<RequestId, ()> = Cache::builder()
            .max_capacity(1000)
            .time_to_live(Duration::from_secs(120))
            .build();

        while let Some(BlockEvent { chain_id, .. }) = stream.next().await {
            let trades = solver.fetch_state(chain_id, &inflight_requests).await?;
            if !trades.is_empty() {
                tracing::info!(
                    chain_id = chain_id,
                    trade_count = trades.len(),
                    "executing trades "
                );
                executor.execute(trades, &mut inflight_requests).await;
            }
        }

        anyhow::bail!("stream of blocks ended unexpectedly");
    }
}
