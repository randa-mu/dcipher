//! Implementations of BV_broadcast and SBV_broadcast.

use crate::aba::crain20::coin::CoinToss;
use crate::aba::crain20::messages::{
    AbaMessage, AuxStage, AuxiliaryMessage, EstimateMessage, View,
};
use crate::aba::crain20::{AbaCrain20Instance, AbaState};
use crate::aba::{Estimate, crain20};
use crate::helpers::PartyId;
use crate::network::broadcast_with_self;
use dcipher_network::TransportSender;
use std::sync::Arc;
use tracing::{Level, error, event};

impl<CT, CK, TS> AbaCrain20Instance<CT, CK, TS>
where
    CT: CoinToss,
    TS: TransportSender<Identity = PartyId> + Clone,
{
    /// Binary-value broadcast described in https://dl.acm.org/doi/10.1145/2785953, Figure 1
    /// Send the current party's estimate to all other nodes with an Estimate message.
    #[tracing::instrument(skip(self))]
    async fn bv_broadcast(&self, r: u8, stage: AuxStage, v: Estimate) {
        // 1: broadcast B_VAL(v) to all
        let msg_est = AbaMessage::Estimate(EstimateMessage {
            round: r,
            stage,
            estimate: v,
        });

        event!(
            Level::DEBUG,
            "Node `{}` at round `{r}` sending {:?} to all",
            self.config.id,
            msg_est
        );
        if let Err(e) =
            broadcast_with_self(&msg_est, &self.config.retry_strategy, &self.sender).await
        {
            error!(
                "Node `{}` failed to broadcast estimate message: {e:?}",
                self.config.id
            )
        }
    }

    /// Synchronized binary-value broadcast described in https://dl.acm.org/doi/10.1145/2785953, Figure 2
    /// Send the current party's estimate to all other nodes with an Estimate message.
    #[tracing::instrument(skip(self, state))]
    pub(super) async fn sbv_broadcast(
        &self,
        r: u8,
        stage: AuxStage,
        v: Estimate,
        state: &Arc<AbaState<CT>>,
    ) -> View {
        // 1: BV_Broadcast(v)
        self.bv_broadcast(r, stage, v).await;

        event!(
            Level::DEBUG,
            "Node `{}` waiting for bin values",
            self.config.id
        );
        let bin_values = loop {
            // 2: wait until bin_values \neq \emptyset
            state.notify_bin_values.notified((r, stage)).await;

            let bin_values = state.bin_values.lock().await;
            let bin_values = &bin_values.get(&r).cloned().unwrap_or_default()[stage];
            if !bin_values.is_empty() {
                event!(
                    Level::DEBUG,
                    "Node `{}` obtained bin_values = `{bin_values:?}`",
                    self.config.id
                );
                break bin_values.clone();
            }
        };

        // 3: Send AUX(w) for w \in bin_values to all
        for w in bin_values.iter() {
            let msg_aux = AbaMessage::Auxiliary(AuxiliaryMessage {
                round: r,
                stage,
                estimate: *w,
            });
            event!(
                Level::DEBUG,
                "Node `{}` sending {:?} to all",
                self.config.id,
                msg_aux
            );

            if let Err(e) =
                broadcast_with_self(&msg_aux, &self.config.retry_strategy, &self.sender).await
            {
                error!(
                    "Node `{}` failed to broadcast aux message: {e:?}",
                    self.config.id
                )
            }
        }

        // 4: wait until \exists a set view s.t.
        //  (1) view \subseteq bin_values, and
        //  (2) contained in AUX(.) messages received from n - t nodes
        let view = loop {
            event!(
                Level::DEBUG,
                "Node `{}` waiting for count_aux notification",
                self.config.id
            );

            // wake up each time after having received n - t aux, or on bin_values update
            crain20::future_select_pin(
                state.notify_count_aux.notified((r, stage)),
                state.notify_bin_values.notified((r, stage)),
            )
            .await;

            let aux_views = state.aux_views.lock().await;
            let bin_values = state.bin_values.lock().await; // warn: two locks, could deadlock
            let aux_views = aux_views.get(&(r, stage)).to_owned().unwrap_or_default();
            let bin_values = &bin_values.get(&r).cloned().unwrap_or_default()[stage];
            let view = self.construct_view(bin_values, &aux_views);
            if let Some(view) = view {
                event!(
                    Level::DEBUG,
                    "Node {} obtained view = `{view:?}`",
                    self.config.id
                );
                break view;
            } else {
                event!(
                    Level::DEBUG,
                    "Node {} received notify_count_aux notification while having no binary estimates / not enough aux",
                    self.config.id
                );
            }
        };
        // 5: return view
        #[allow(clippy::let_and_return)] // for clarity
        view
    }
}
