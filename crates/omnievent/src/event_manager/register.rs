//! Module for events registration

use crate::event_manager::db::EventsDatabase;
use crate::event_manager::listener::InternalEventStreamRegistration;
use crate::event_manager::{EventManager, EventManagerError, RegisteredEventEntry};
use crate::types::{EventId, NewRegisteredEventSpecError, RegisteredEventSpec};
use alloy::network::{Ethereum, Network};
use alloy::primitives::B256;
use alloy::providers::{GetSubscription, Provider};
use alloy::pubsub::SubscriptionStream;
use alloy::rpc::client::{RpcCall, WeakClient};
use alloy::rpc::json_rpc::RpcRecv;
use alloy::rpc::types::pubsub::{Params, SubscriptionKind};
use alloy::rpc::types::{Filter, Log};
use futures::Stream;
use futures_util::{FutureExt, StreamExt};
use std::pin::Pin;
use std::task::{Context, Poll};
use superalloy::provider::MultiChainProvider;

impl<MP, DB> EventManager<MP, DB>
where
    MP: MultiChainProvider<u64>,
    DB: EventsDatabase,
{
    pub(super) async fn internal_register_ethereum_event(
        &self,
        event_spec: RegisteredEventSpec,
    ) -> Result<EventId, EventManagerError> {
        tracing::debug!("Registering new event");

        // Make sure we're ready to register new events
        let Some(listener_handle) = self.listener_handle.as_ref() else {
            Err(EventManagerError::NotReady)?
        };

        // Do nothing if the event is already registered
        let event_id = event_spec.id;
        if self.active_events_map.read().await.contains_key(&event_id) {
            tracing::debug!("Event already registered");
            return Ok(event_id);
        }

        let stream = create_stream::<_, Ethereum>(&event_spec, &self.multi_provider).await?;

        let reg = InternalEventStreamRegistration::new(
            event_spec.clone(),
            stream.map(move |l| (event_id, l)).boxed(), // boxing :( but we need type erasure due to the closure
        );

        // Save the event in the database
        if let Err(e) = self.events_db.store_event(event_spec).await {
            tracing::error!(event = ?event_id, error = ?e, "Failed to store event in database");
            Err(EventManagerError::Database(e.into()))?
        }

        // Register the stream with the bg task
        if let Err(e) = listener_handle.register_event_stream(reg).await {
            tracing::error!(event = ?event_id, error = ?e, "Failed to register event stream");
            // TODO: Currently, this only happens if the bg task has dropped its receiver
            //  => not recoverable. We may consider just letting it explode here.
            Err(EventManagerError::EventStreamRegistration(e))?
        }

        {
            // Store a new entry in the local active events map
            let mut active_events_map = self.active_events_map.write().await;
            active_events_map.insert(
                event_id,
                RegisteredEventEntry {
                    outgoing_stream: None,
                },
            );
        }

        tracing::info!("New event stored and registered");
        Ok(event_id)
    }
}

#[derive(thiserror::Error, Debug)]
pub enum CreateStreamError {
    #[error("failed to create event")]
    FailedToCreateEvent(#[from] NewRegisteredEventSpecError),

    #[error("unsupported chain")]
    UnsupportedChain,

    #[error("failed to subscribe to logs")]
    RpcWithTransportErrorKind(
        #[from] alloy::transports::RpcError<alloy::transports::TransportErrorKind>,
    ),
}

/// Create a new stream with, optionally, periodic resubscriptions.
pub(crate) async fn create_stream<MP, N>(
    spec: &RegisteredEventSpec,
    multi_provider: &MP,
) -> Result<ReliableSubscriptionStream<Log>, CreateStreamError>
where
    MP: MultiChainProvider<u64>,
    N: Network,
{
    // Obtain a provider for the specified chainid and network
    let Some(provider) = multi_provider.get_provider::<N>(&spec.chain_id) else {
        Err(CreateStreamError::UnsupportedChain)?
    };

    let stream = ReliableSubscriptionStream::try_new_subscription(
        provider,
        (
            SubscriptionKind::Logs,
            Params::Logs(Box::new(Filter::from(spec))),
        ),
        spec.reregistration_delay,
    )
    .await?;

    Ok(stream)
}

pub struct ReliableSubscriptionStream<T: RpcRecv> {
    /// An active subscription
    subscription: SubscriptionStream<T>,

    /// The rpc call used to create a new subscription
    subscription_call: RpcCall<(SubscriptionKind, Params), B256>,

    /// A weak reference to an RPC client used for recreating subscriptions
    client: WeakClient,

    /// A future used when a reconnection is pending
    reconnect_fut:
        Option<<GetSubscription<(SubscriptionKind, Params), T> as IntoFuture>::IntoFuture>,

    /// An interval indicating when to recreate a subscription
    reconnect_interval: Option<tokio::time::Interval>,
}

impl<T: RpcRecv> ReliableSubscriptionStream<T> {
    pub async fn try_new_subscription<N>(
        provider: impl Provider<N>,
        sub_params: (SubscriptionKind, Params),
        interval: Option<std::time::Duration>,
    ) -> Result<Self, CreateStreamError>
    where
        N: Network,
    {
        let rpc_call = provider.client().request("eth_subscribe", sub_params);

        // Create a new subscription for the specified event
        let stream = GetSubscription::new(provider.weak_client(), rpc_call.clone())
            .await?
            .into_stream();

        Ok(Self {
            subscription: stream,
            subscription_call: rpc_call,
            client: provider.weak_client(),
            reconnect_fut: None,
            reconnect_interval: interval.map(|d| tokio::time::interval(d)),
        })
    }
}

impl<T: RpcRecv> Stream for ReliableSubscriptionStream<T> {
    type Item = T;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        // We always prioritize yielding new items from the stream. If the stream is pending, we handle reconnect
        match self.subscription.poll_next_unpin(cx) {
            Poll::Ready(item) => return Poll::Ready(item),
            Poll::Pending => (),
        }

        // Is a reconnection pending?
        if let Some(reconnect_fut) = self.reconnect_fut.as_mut()
            && let Poll::Ready(ready) = reconnect_fut.poll_unpin(cx)
        {
            // Future completed, reset reconnect interval now, and clear future
            if let Some(i) = self.reconnect_interval.as_mut() {
                i.reset()
            }
            self.reconnect_fut = None;

            match ready {
                Ok(subscription) => {
                    tracing::debug!("Replacing subscription with fresh one");
                    self.subscription = subscription.into_stream();

                    // Ready to poll the new subscription stream immediately
                    cx.waker().wake_by_ref();
                }
                Err(e) => {
                    // Reconnection error, continue with the same subscription, retry on the next tick
                    tracing::error!(error = ?e, rpc_call = ?self.subscription_call, "Failed to reconnect stream");
                }
            }

            return Poll::Pending;
        }

        // Is a subscription reconnection required?
        if let Some(interval) = &mut self.reconnect_interval
            && let Poll::Ready(_) = interval.poll_tick(cx)
        {
            tracing::debug!("Re-registering subscription");

            // Yes, init the reconnection future
            self.reconnect_fut = Some(
                GetSubscription::new(self.client.clone(), self.subscription_call.clone())
                    .into_future(),
            );

            // Ready to poll the reconnection future
            cx.waker().wake_by_ref();
        }

        Poll::Pending
    }
}
