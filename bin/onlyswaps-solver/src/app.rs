use crate::config::ProfitabilityConfig;
use crate::executor::TradeExecutor;
use crate::fee_adapter::DefaultFeeAdapter;
use crate::model::{RequestId, SolverEvent};
use crate::network::Network;
use crate::price_feed::coingecko::CoinGeckoClient;
use crate::profitability::{
    AlwaysProfitable, ErasedProfitabilityEstimator, StdProfitabilityEstimator,
};
use crate::solver::Solver;
use alloy::providers::DynProvider;
use alloy::signers::local::PrivateKeySigner;
use config::timeout::TimeoutConfig;
use futures::{Stream, StreamExt};
use moka::future::Cache;
use std::collections::HashMap;
use std::ops::Mul;
use tokio_stream::wrappers::IntervalStream;

pub struct App {}
impl App {
    pub async fn start(
        signer: PrivateKeySigner,
        networks: HashMap<u64, Network<DynProvider>>,
        timeout: &TimeoutConfig,
        profitability: &ProfitabilityConfig,
    ) -> anyhow::Result<()> {
        let pe = match profitability {
            ProfitabilityConfig::AlwaysProfitable => {
                ErasedProfitabilityEstimator::from_estimator(AlwaysProfitable)
            }
            ProfitabilityConfig::CheckWithCoinGecko { api_key, pro_api } => {
                let mut builder = CoinGeckoClient::builder();
                if let Some(api_key) = api_key {
                    builder = builder.api_key(api_key.to_owned());
                }
                if !pro_api {
                    builder = builder.use_demo_api()
                }

                let mut cg_price_feed = builder.build()?;
                cg_price_feed.init_chain_id_mapping().await?;
                ErasedProfitabilityEstimator::from_estimator(StdProfitabilityEstimator::new(
                    cg_price_feed,
                ))
            }
        };

        let chain_ticker = per_chain_ticker(networks.values()).map(SolverEvent::Poll);
        let mut stream = Box::pin(futures::stream::select(event_ticker, chain_ticker));
        let fee_estimator = DefaultFeeAdapter::new();
        let mut solver = Solver::new(&networks, &fee_estimator).await?;
        let executor = TradeExecutor::new(signer, &networks, pe).await?;

        // we pull new chain state every block, so inflight requests may not have been
        // completed yet, so we don't want to attempt to execute them again and waste gas.
        // if they're still there after twice the request timeout we can reattempt
        let mut inflight_requests: Cache<RequestId, ()> = Cache::builder()
            .max_capacity(1000)
            .time_to_live(timeout.request_timeout.mul(2))
            .build();

        while let Some(event) = stream.next().await {
            let chain_id = event.chain_id();
            let trades = solver.solve(chain_id, &inflight_requests).await?;
            if !trades.is_empty() {
                tracing::info!(chain_id, trade_count = trades.len(), "executing trades ");
                executor
                    .execute(trades, &mut inflight_requests, timeout)
                    .await;
            }
        }

        anyhow::bail!("stream of blocks ended unexpectedly");
    }
}

fn per_chain_ticker<'a, P>(
    networks: impl IntoIterator<Item = &'a Network<P>>,
) -> impl Stream<Item = u64> + Unpin + 'a
where
    P: 'a,
{
    futures::stream::select_all(networks.into_iter().map(move |net| {
        let interval = tokio::time::interval(net.poll_interval);
        IntervalStream::new(interval).map(|_| net.chain_id)
    }))
}
