use crate::config::AppConfig;
use crate::omnievent::{OmnieventManager, create_event_manager};
use anyhow::anyhow;
use futures::StreamExt;
use omnievent::types::EventId;

pub(crate) struct App {}

impl App {
    pub async fn start(config: &AppConfig) -> anyhow::Result<()> {
        let OmnieventManager {
            registered_by_chain_id,
            omnievent,
        } = create_event_manager(&config.networks).await?;

        let events_ids: Vec<EventId> = registered_by_chain_id
            .values()
            .flat_map(|it| -> Vec<EventId> { it.into() })
            .collect();
        let mut stream = omnievent
            .get_ethereum_multi_event_stream(events_ids)
            .await?;
        while let Some(Ok(event)) = stream.next().await {
            let event_type = registered_by_chain_id
                .get(&event.chain_id)
                .ok_or(anyhow!(
                    "somehow we don't have a stream for chain_id {}",
                    event.chain_id
                ))
                .and_then(|it| it.extract(event.event_id))?;
            println!("got event type: {:?}", event_type);
        }
        anyhow::bail!("event stream died");
    }
}
