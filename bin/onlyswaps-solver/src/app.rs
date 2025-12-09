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
use anyhow::Context;
use axum::http::{Request, Response};
use config::timeout::TimeoutConfig;
use futures::{Stream, StreamExt};
use moka::future::Cache;
use omnievent::proto_types::omni_event_service_client::OmniEventServiceClient;
use omnievent::proto_types::{
    BlockSafety, EventField, EventOccurrence, RegisterNewEventRequest, StreamEventsRequest,
};
use std::collections::HashMap;
use std::ops::Mul;
use tokio_stream::wrappers::IntervalStream;
use tonic::Streaming;
use tonic::body::Body;
use tower::BoxError;
use tower::util::BoxService;

pub type OmniEventBoxService = BoxService<Request<Body>, Response<Body>, BoxError>;

pub struct App {}
impl App {
    pub async fn start(
        signer: PrivateKeySigner,
        networks: HashMap<u64, Network<DynProvider>>,
        timeout: &TimeoutConfig,
        profitability: &ProfitabilityConfig,
        oes: OmniEventBoxService,
    ) -> anyhow::Result<()> {
        let mut client = OmniEventServiceClient::new(oes);
        let swap_stream = swap_requested_stream(&mut client, &networks).await?;

        // We don't actually care about the detail of the event, we just use it as a ticker
        let event_ticker = swap_stream.filter_map(|res| async {
            match res {
                Ok(event) => Some(SolverEvent::ChainEvent(event.chain_id)),
                Err(e) => {
                    tracing::error!(error = ?e, "Received an error through event stream");
                    None
                }
            }
        });

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

async fn swap_requested_stream(
    client: &mut OmniEventServiceClient<OmniEventBoxService>,
    networks: &HashMap<u64, Network<DynProvider>>,
) -> anyhow::Result<Streaming<EventOccurrence>> {
    let mut event_uuids = Vec::with_capacity(networks.len());
    for (&chain_id, net) in networks.iter() {
        let response = client
            .register_event(RegisterNewEventRequest {
                chain_id,
                address: net.router.address().to_vec().into(),
                event_name: "SwapRequested".to_string(),
                fields: vec![
                    EventField {
                        sol_type: "bytes32".to_string(),
                        indexed: true,
                    },
                    EventField {
                        sol_type: "uint256".to_string(),
                        indexed: true,
                    },
                    EventField {
                        sol_type: "uint256".to_string(),
                        indexed: true,
                    },
                ],
                block_safety: BlockSafety::Latest.into(),
                reregistration_delay: None,
            })
            .await
            .context("failed to register event")?;
        event_uuids.push(response.into_inner().uuid);
    }

    let event_stream = client
        .stream_events(StreamEventsRequest { event_uuids })
        .await
        .context("failed to stream events")?
        .into_inner();

    Ok(event_stream)
}
