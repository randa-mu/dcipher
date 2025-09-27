use crate::chain_state::NetworkBus;
use crate::chain_state_pending::Verification;
use crate::retry_runtime::RetryScheduler;
use crate::verification_events::{EventManagement, create_omnievent_management};
use alloy::primitives::FixedBytes;
use alloy::providers::DynProvider;
use async_stream::stream;
use config::network::NetworkConfig;
use futures::{Stream, StreamExt};
use std::sync::Arc;
use tokio::select;

pub struct VerificationBus {
    network_bus: Arc<NetworkBus<DynProvider>>,
    event_management: EventManagement,
    retry_scheduler: RetryScheduler,
}

impl VerificationBus {
    pub async fn new(
        networks: &Vec<NetworkConfig>,
        network_bus: Arc<NetworkBus<DynProvider>>,
        retry_scheduler: RetryScheduler,
    ) -> anyhow::Result<Self> {
        Ok(Self {
            network_bus,
            retry_scheduler,
            event_management: create_omnievent_management(networks).await?,
        })
    }

    pub async fn stream(
        &mut self,
    ) -> anyhow::Result<impl Stream<Item = Verification<FixedBytes<32>>>> {
        let mut live_stream = self
            .event_management
            .omnievent
            .get_ethereum_multi_event_stream(self.event_management.event_ids.clone())
            .await?;

        let pending_verifications = self
            .network_bus
            .fetch_pending_verifications()
            .await
            .unwrap_or_default();

        Ok(stream! {
            let retry_stream = self.retry_scheduler.stream();
            tokio::pin!(retry_stream);

            for verification in pending_verifications {
                tracing::debug!("yielding verif : {:?}", verification);
                yield verification
            }

            loop {
                select! {
                    verification = retry_stream.next() => {
                        if let Some(verification) = verification {
                            yield verification;
                        }
                    },
                    live_event = live_stream.next() => {
                        if let Some(Ok(event)) = live_event {
                            let verification = event.data.try_into().expect("invalid event received");
                            yield verification;
                        }
                    }
                }
            }
        })
    }
}
