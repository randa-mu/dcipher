mod arguments_parser;

use crate::arguments_parser::{Args, DSignerConfig, NodesConfiguration};
use ark_ec::{AffineRepr, CurveGroup};
use ark_ff::{BigInteger, PrimeField};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::routing::get;
use axum::{Json, Router, extract::State, routing::post};
use dcipher_agents::signer::threshold_signer::{
    AsyncThresholdSigner, ThresholdSigner, lagrange_points_interpolate_at,
};
use dcipher_agents::signer::{AsynchronousSigner, BN254SignatureOnG1Signer};
use pairing_utils::serialize::point::PointSerializeCompressed;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::net::TcpListener;
use tokio_util::sync::CancellationToken;
use tracing_subscriber::FmtSubscriber;

// Request structure for the sign endpoint
#[derive(Deserialize)]
struct SignRequest {
    m: String,
}

// Response structure with the signature
#[derive(Serialize)]
struct SignResponse {
    signature: String,
    dst: String,
}

#[derive(Clone, Serialize)]
struct PublicKey {
    x: [String; 2],
    y: [String; 2],
}

// Application state
struct AppState {
    async_signer: AsyncThresholdSigner<BN254SignatureOnG1Signer>,
    dst: String,
    pk: PublicKey,
}

async fn healthcheck_handler() -> &'static str {
    "healthy"
}

// Handler for the pk (public key) endpoint
async fn pk_handler(State(state): State<Arc<AppState>>) -> Response {
    Json(state.pk.clone()).into_response()
}

// Handler for the sign endpoint
async fn sign_handler(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<SignRequest>,
) -> Response {
    let start = std::time::Instant::now();

    // Call the async signing function
    let sig_res = state.async_signer.async_sign(&payload.m).await;

    let duration = start.elapsed();
    tracing::debug!("Signing operation took {}ms", duration.as_millis());

    if let Ok(sig) = sig_res {
        let base64_sig = sig
            .ser_base64()
            .expect("ser should always work for server-generated content");
        Json(SignResponse {
            signature: base64_sig,
            dst: state.dst.clone(),
        })
        .into_response()
    } else {
        (StatusCode::INTERNAL_SERVER_ERROR, "failed to sign").into_response()
    }
}

fn get_signer(
    config: &Args,
    nodes_config: &NodesConfiguration,
) -> (
    CancellationToken,
    AsyncThresholdSigner<BN254SignatureOnG1Signer>,
    ark_bn254::G2Affine,
) {
    // Parse key
    let sk: ark_bn254::Fr = config.key_config.bls_key.to_owned().into();

    // Get per-nodes config
    let (mut pks, addresses, peer_ids, short_ids): (Vec<_>, Vec<_>, Vec<_>, Vec<_>) = nodes_config
        .nodes
        .iter()
        .cloned()
        .map(|c| (c.bls_pk, c.address, c.peer_id, c.node_id.get()))
        .collect();

    // Add own pk to the list if required
    let mut all_nodes_ids = short_ids.clone();
    if pks.len() == usize::from(config.key_config.n.get() - 1) {
        let pki = ark_bn254::G2Affine::generator() * sk;
        pks.insert(
            usize::from(config.key_config.node_id.get() - 1),
            pki.into_affine(),
        );
        all_nodes_ids.insert(
            usize::from(config.key_config.node_id.get() - 1),
            config.key_config.node_id.get(),
        );
    }

    // Compute group public key
    let ids_pks = pks
        .iter()
        .copied()
        .zip(all_nodes_ids)
        .map(|(pki, i)| (u64::from(i), pki.into_group()))
        .collect::<Vec<_>>();

    let pk = lagrange_points_interpolate_at(&ids_pks, 0).into_affine();

    // Create a threshold signer
    let cs = BN254SignatureOnG1Signer::new(sk, config.key_config.dst.clone().into_bytes());
    let ts = ThresholdSigner::new(
        cs.clone(),
        config.key_config.n.get(),
        config.key_config.t.get(),
        config.key_config.node_id.get(),
        pks,
    )
    .with_eager_signing();

    let (cancel, async_signer) = ts.run(
        config.libp2p.libp2p_key.clone().into(),
        config.libp2p.libp2p_listen_addr.clone(),
        addresses,
        peer_ids,
        short_ids,
    );

    (cancel, async_signer, pk)
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let DSignerConfig {
        config,
        nodes_config,
    } = DSignerConfig::parse()?;

    // Set logging options
    FmtSubscriber::builder()
        .with_max_level(config.log_level)
        .init();

    // Initialize and start a threshold signer
    let (cancel, async_signer, group_pk) = get_signer(&config, &nodes_config.unwrap_or_default());

    // Convert group pk to string
    let (x, y) = group_pk.xy().expect("pk cannot be at infinity");
    let pk = PublicKey {
        x: [
            hex::encode(x.c0.into_bigint().to_bytes_be()),
            hex::encode(x.c1.into_bigint().to_bytes_be()),
        ],
        y: [
            hex::encode(y.c0.into_bigint().to_bytes_be()),
            hex::encode(y.c1.into_bigint().to_bytes_be()),
        ],
    };

    // Initialize application state
    let app_state = Arc::new(AppState {
        async_signer,
        dst: config.key_config.dst,
        pk,
    });

    // Build our application with a single route
    let app = Router::new()
        .route("/sign", post(sign_handler))
        .route("/pk", get(pk_handler))
        .route("/healthcheck", get(healthcheck_handler))
        .with_state(app_state);

    // Run the server
    let listener = TcpListener::bind((config.listen_addr.clone(), config.port)).await?;
    println!("Server listening on {}:{}", config.listen_addr, config.port);

    // Setup some signals
    let mut sigterm = tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())?;
    let mut sigint = tokio::signal::unix::signal(tokio::signal::unix::SignalKind::interrupt())?;

    let res = tokio::select! {
        _ = sigterm.recv() => {
            println!("received SIGTERM, shutting down...");
            Ok(())
        },

        _ = sigint.recv() => {
            println!("received SIGINT, shutting down...");
            Ok(())
        },

        _ = tokio::signal::ctrl_c() => {
            println!("received ctrl+c, shutting down...");
            Ok(())
        },

        err = axum::serve(listener, app) => {
            eprintln!("axum stopped unexpectedly...");
            err // return Result
        },
    };

    cancel.cancel();
    Ok(res?)
}
