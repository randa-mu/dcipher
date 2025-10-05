use crate::chain_state::NetworkBus;
use crate::chain_state_pending::{RequestId, Verification};
use crate::chain_state_resolver::ChainStateResolver;
use crate::cli::StartArgs;
use crate::config::{AppConfig, ConfigFile};
use crate::evaluator::Evaluator;
use crate::retry_runtime::RetryScheduler;
use crate::signing::{NetworkedSigner, OnlySwapsSigner, VerifiedSwap};
use crate::transport::create_libp2p_transport;
use crate::verification_bus::VerificationBus;
use agent_utils::healthcheck_server::HealthcheckServer;
use agent_utils::monitoring::init_monitoring;
use config::file::load_mapped_config_file;
use std::sync::Arc;
use std::time::Duration;
use tokio_stream::StreamExt;

pub async fn start_verifier(args: StartArgs) -> anyhow::Result<()> {
    let app_config = load_mapped_config_file::<ConfigFile, AppConfig>(args.config_path)?;
    let healthcheck_server = HealthcheckServer::new(
        app_config.agent.healthcheck_listen_addr,
        app_config.agent.healthcheck_port,
    )
    .await?;
    init_monitoring(&app_config.agent)?;

    // listen for OS signals or any of the tasks closing and shut down either gracefully
    // or noisily with errors
    let mut sigterm = tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())?;
    let mut sigint = tokio::signal::unix::signal(tokio::signal::unix::SignalKind::interrupt())?;
    tokio::select! {
        _ = sigterm.recv() => Ok(()),
        _ = sigint.recv() => Ok(()),
        _ = tokio::signal::ctrl_c() => Ok(()),

        res = healthcheck_server.start() =>  {
           match res {
                Ok(()) => anyhow::bail!("healthcheck stopped unexpectedly without an error"),
                Err(e) => anyhow::bail!("healthcheck stopped unexpectedly: {}", e),
           }
        }

        res = run_onlyswaps(&app_config) => {
           match res {
                Ok(()) => anyhow::bail!("swap loop stopped unexpectedly without an error"),
                Err(e) => anyhow::bail!("swap loop stopped unexpectedly: {}", e),
           }
        }
    }
}

async fn run_onlyswaps(app_config: &AppConfig) -> anyhow::Result<()> {
    // the `network_bus` manages access to all the chains at once for pulling state or submitting txs
    let network_bus =
        Arc::new(NetworkBus::create(&app_config.networks, &app_config.timeout).await?);

    // the `retry_scheduler` receives `Verification`s that have failed and schedules them at a later time
    // with respect to the retry duration
    let retry_duration = Duration::from_secs(12);
    let retry_scheduler = RetryScheduler::new(retry_duration);
    let retry_scheduler_tx = retry_scheduler.tx();

    // the `verification_bus` combines recent historical events, live events, and retried events
    // to forward to the conditional evaluation, and then signing steps.
    let mut verification_bus =
        VerificationBus::new(&app_config.networks, network_bus.clone(), retry_scheduler).await?;

    // the `resolver` fetches the current src and dest states from a given request_id so we can evaluate
    // whether a swap has truly been completed
    let resolver = ChainStateResolver::new(network_bus.clone());

    // the `signer` encapsulates everything related to gossiping, verifying, and aggregating partial
    // signatures using libp2p.
    let transport = create_libp2p_transport(app_config)?;
    let networked_signer = NetworkedSigner::new(app_config, transport)?;
    let signer = OnlySwapsSigner::new(networked_signer);
    tracing::info!(
        multiaddr = app_config.libp2p.multiaddr.to_string(),
        n = app_config.committee.n,
        t = app_config.committee.t,
        "threshold signer created"
    );

    // all contract events and load-on-start state are ingested via a channel and
    // are serialised out of the channel into the signer. This is actually asyncified once
    // popped off the channel so that different nodes don't get stuck signing different
    // messages when e.g. in catchup
    let (tx, mut rx_verifications) =
        tokio::sync::mpsc::unbounded_channel::<Verification<RequestId>>();

    tokio::spawn({
        let tx = tx.clone();
        async move {
            let stream = verification_bus
                .stream()
                .await
                .expect("verifications failed to connect");
            tokio::pin!(stream);
            while let Some(verification) = stream.next().await {
                tx.send(verification)
                    .expect("failed to send verification on channel");
            }
        }
    });

    // loop over all the verification jobs. The swap evaluation has a timeout to stop a broken RPC
    // or malformed requests blocking the world forever (allegedly)
    //
    // the loop is as follows:
    // - receive
    // - resolve
    // - evaluate
    // - sign
    // - submit
    while let Some(verification) = rx_verifications.recv().await {
        tracing::info!(
            chain_id = verification.chain_id,
            request_id = verification.request_id.to_string(),
            "attempting verification"
        );

        // the `evaluator` does the actual evaluation of a (src_chain_state, dest_chain_state) combo.
        // errors in evaluation are kicked back to the `RetryScheduler` to decide when to pop them back
        // onto the `verification_bus`
        let state = resolver.resolve_state(&verification).await?;
        match Evaluator::evaluate(state) {
            Err(e) => {
                tracing::debug!(e = ?e, "evaluation was false");
                retry_scheduler_tx
                    .send(verification)
                    .await
                    .expect("failed to send on retry channel");
            }
            Ok(valid_state) => {
                let solver = valid_state.transfer_receipt.solver;
                let src_chain_id = valid_state.swap_params.srcChainId;
                let signature = signer
                    .sign(&solver, &valid_state.swap_params)
                    .await
                    .expect("signing failed somehow");

                let verified_swap = VerifiedSwap {
                    src_chain_id,
                    request_id: verification.request_id,
                    solver,
                    signature,
                };
                match network_bus.submit_verification(&verified_swap).await {
                    Err(e) => {
                        tracing::error!(
                            chain_id = verification.chain_id,
                            request_id = verification.request_id.to_string(),
                            error = e.to_string(),
                            "verification returned an error"
                        );
                    }
                    Ok(_) => {
                        tracing::info!(
                            chain_id = verification.chain_id,
                            request_id = verification.request_id.to_string(),
                            "verification completed successfully"
                        );
                    }
                }
            }
        }
    }
    Ok(())
}
