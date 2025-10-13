use crate::chain_state_pending::{RequestId, Verification};
use crate::control_plane::{ControlPlane, ResolvedState, VerificationError};
use crate::retry_runtime::RetrySender;
use crate::signing::SignedVerification;
use futures::Stream;
use std::pin::Pin;
use std::sync::Arc;
use tokio::sync::mpsc::unbounded_channel;
use tokio::task::JoinSet;
use tokio_stream::StreamExt;

pub(crate) struct TaskManager<CP> {
    control_plane: Arc<CP>,
}

impl<CP: ControlPlane> TaskManager<CP>
where
    CP: Send + Sync + 'static,
{
    pub fn new(control_plane: impl Into<Arc<CP>>) -> Self {
        Self {
            control_plane: control_plane.into(),
        }
    }
    pub async fn start(
        &self,
        retry_tx: RetrySender,
        event_stream: Pin<Box<impl Stream<Item = Verification<RequestId>> + Send + 'static>>,
    ) {
        tracing::info!("starting channel manager");
        let mut tasks = JoinSet::new();
        let control_plane = self.control_plane.clone();

        let (tx_verifications, mut rx_verifications) =
            unbounded_channel::<Verification<RequestId>>();
        let (tx_resolve, mut rx_resolve) = unbounded_channel::<ResolvedState>();
        let (tx_sign, mut rx_sign) = unbounded_channel::<ResolvedState>();
        let (tx_submit, mut rx_submit) = unbounded_channel::<SignedVerification>();
        let (tx_done, mut rx_done) = unbounded_channel::<SignedVerification>();
        let (tx_err, mut rx_err) = unbounded_channel::<VerificationError>();

        // 'receive' step
        {
            let tx_verifications = tx_verifications.clone();
            tasks.spawn(async move {
                let mut stream = event_stream;
                while let Some(verification) = stream.next().await {
                    tx_verifications
                        .send(verification)
                        .expect("failed to send verification on channel");
                }
            });
        }

       // 'resolve' step
       {
            let control_plane = control_plane.clone();
            let tx_resolve = tx_resolve.clone();
            let tx_err = tx_err.clone();

            // resolve
            tasks.spawn(async move {
                while let Some(verification) = rx_verifications.recv().await {
                    let control_plane = control_plane.clone();
                    let tx_resolve = tx_resolve.clone();
                    let tx_err = tx_err.clone();

                    tokio::spawn(async move {
                        match control_plane.resolve_state(verification.clone()).await {
                            Ok(v) => tx_resolve.send(v).expect("error writing on channel"),
                            Err(_) => tx_err
                                .send(VerificationError::Resolve(verification))
                                .expect("error writing on channel"),
                        }
                    });
                }
            });
        }

        // 'evaluate' step
        {
            let control_plane = control_plane.clone();
            let tx_err = tx_err.clone();

            tasks.spawn(async move {
                while let Some(valid) = rx_resolve.recv().await {
                    let control_plane = control_plane.clone();
                    let tx_sign = tx_sign.clone();
                    let tx_err = tx_err.clone();

                    tokio::spawn(async move {
                        match control_plane.evaluate_state(valid.clone()).await {
                            Ok(v) => tx_sign.send(v).expect("error writing on channel"),
                            Err(_) => tx_err
                                .send(VerificationError::Evaluate(valid))
                                .expect("error writing on channel"),
                        }
                    });
                }
            });
        }

        // 'sign' step
        {
            let control_plane = control_plane.clone();
            let tx_err = tx_err.clone();

            tasks.spawn(async move {
                while let Some(state) = rx_sign.recv().await {
                    let control_plane = control_plane.clone();
                    let tx_submit = tx_submit.clone();
                    let tx_err = tx_err.clone();

                    tokio::spawn(async move {
                        match control_plane.sign_state(state.clone()).await {
                            Ok(v) => tx_submit.send(v).expect("error writing on channel"),
                            Err(_) => tx_err
                                .send(VerificationError::Sign(state))
                                .expect("error writing on channel"),
                        }
                    });
                }
            });
        }

        // 'submit' step
        {
            let control_plane = control_plane.clone();
            let tx_done = tx_done.clone();
            let tx_err = tx_err.clone();

            tasks.spawn(async move {
                while let Some(signed_verification) = rx_submit.recv().await {
                    let control_plane = control_plane.clone();
                    let tx_done = tx_done.clone();
                    let tx_err = tx_err.clone();

                    tokio::spawn(async move {
                        match control_plane
                            .submit_state(signed_verification.clone())
                            .await
                        {
                            Ok(v) => tx_done.send(v).expect("error writing on channel"),
                            Err(_) => tx_err
                                .send(VerificationError::Submit(signed_verification))
                                .expect("error writing on channel"),
                        }
                    });
                }
            });
        }

        // 'done' step
        tasks.spawn(async move {
            while let Some(state) = rx_done.recv().await {
                tracing::info!(
                    chain_id = state.src_chain_id.to_string(),
                    request_id = state.request_id.to_string(),
                    "verification completed successfully"
                );
            }
        });

        // 'error handling' step
        {
            let control_plane = control_plane.clone();
            let retry_tx = retry_tx.clone();

            tasks.spawn(async move {
                while let Some(err) = rx_err.recv().await {
                    control_plane.handle_error(&err, retry_tx.clone()).await;
                }
            });
        }

        // join all the things so we don't cede control back to the caller
        tracing::info!("tasks all started");
        tasks.join_all().await;
    }
}
