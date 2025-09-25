use crate::cli::StartArgs;
use crate::config::{AppConfig, ConfigFile};
use crate::eth::NetworkBus;
use crate::events::{EventManagement, create_omnievent_management};
use crate::pending::{RequestId, Verification};
use crate::signing::OnlySwapsSigner;
use crate::threshold::create_bn254_signer;
use crate::transport::create_libp2p_transport;
use agent_utils::healthcheck_server::HealthcheckServer;
use agent_utils::monitoring::init_monitoring;
use config::file::load_mapped_config_file;
use futures::StreamExt;

pub async fn start_daemon(args: StartArgs) -> anyhow::Result<()> {
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
        _ = sigterm.recv() =>
            Ok(()),

        _ = sigint.recv() =>
            Ok(()),

        _ = tokio::signal::ctrl_c() =>
            Ok(()),

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
    let network_bus = NetworkBus::create(&app_config.networks).await?;
    let transport = create_libp2p_transport(app_config)?;
    let dsigner = create_bn254_signer(app_config, transport)?;
    let signer = OnlySwapsSigner::new(network_bus, dsigner);
    tracing::info!(
        multiaddr = app_config.libp2p.multiaddr.to_string(),
        n = app_config.committee.n,
        t = app_config.committee.t,
        "threshold signer created"
    );

    // all contract events and load-on-start state are ingested via a channel and
    // they are serialised out of the channel into the signer. This is actually asyncified once
    // popped off the channel so that different nodes don't get stuck signing different
    // messages when e.g. in catchup
    let (tx, mut rx_verifications) =
        tokio::sync::mpsc::unbounded_channel::<Verification<RequestId>>();

    // start listening for new contract events and write them to the channel
    let EventManagement {
        event_ids,
        omnievent,
    } = create_omnievent_management(&app_config.networks).await?;
    let mut stream = omnievent.get_ethereum_multi_event_stream(event_ids).await?;
    let tx1 = tx.clone();
    tokio::spawn(async move {
        while let Some(Ok(event)) = stream.next().await {
            let verification = event.data.try_into().expect("invalid event received");
            tx1.send(verification)
                .expect("failed to send verification on channel");
        }
    });

    // but also fetch any outstanding verifications that existed before our node started so we
    // can verify them too (and maybe help an existing running node to aggregate a signature, as we
    // may have been offline for other partials).
    let network_bus = NetworkBus::create(&app_config.networks).await?;
    let pending_verifications = network_bus
        .fetch_pending_verifications()
        .await
        .unwrap_or_default();
    for verification in pending_verifications {
        tx.send(verification)?;
    }

    // loop over all the futures, be that incoming verification events or OS shutdown signals.
    // the swap evaluation has a timeout to stop a broken RPC or malformed request
    // blocking the world forever
    while let Some(verification) = rx_verifications.recv().await {
        tracing::info!(
            chain_id = verification.chain_id,
            request_id = verification.request_id.to_string(),
            "attempting verification"
        );
        let signer = signer.clone();
        tokio::spawn(async move {
            match signer.evaluate_and_send(&verification).await {
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
        });
    }
    Ok(())
}
