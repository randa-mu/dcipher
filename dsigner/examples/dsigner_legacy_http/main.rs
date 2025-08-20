mod arguments_parser;

use crate::arguments_parser::{Args, DSignerConfig, NodesConfiguration};
use anyhow::Context;
use ark_ec::{AffineRepr, CurveGroup};
use ark_ff::{BigInteger, PrimeField};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::routing::get;
use axum::{Json, Router, extract::State, routing::post};
use base64::Engine;
use base64::prelude::BASE64_STANDARD;
use dcipher_network::transports::libp2p::{Libp2pNode, Libp2pNodeConfig};
use dcipher_signer::bls::{AsyncThresholdSigner, BlsPairingSigner, BlsThresholdSigner};
use dcipher_signer::dsigner::{
    ApplicationAnyArgs, ApplicationArgs, BlsSignatureAlgorithm, BlsSignatureCurve,
    BlsSignatureHash, DSignerScheme, SignatureAlgorithm, SignatureRequest,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::net::TcpListener;
use tokio_util::sync::CancellationToken;
use tower_http::trace::TraceLayer;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use utils::serialize::point::PointDeserializeCompressed;

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
struct PublicKey(String);

// Application state
struct AppState {
    async_signer: AsyncThresholdSigner,
    alg: SignatureAlgorithm,
    args: ApplicationArgs,
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

    let request = SignatureRequest {
        alg: state.alg,
        args: state.args.clone(),
        m: payload.m.into(),
    };

    // Call the async signing function
    let sig_res = state.async_signer.async_sign(request).await;

    let duration = start.elapsed();
    tracing::debug!("Signing operation took {}ms", duration.as_millis());

    if let Ok(sig) = sig_res {
        let base64_sig = BASE64_STANDARD.encode(sig);
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
) -> anyhow::Result<(Libp2pNode<u16>, CancellationToken, AsyncThresholdSigner)> {
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
    let pks_g2 = pks
        .iter()
        .copied()
        .zip(all_nodes_ids)
        .map(|(pki, i)| (i, pki))
        .collect::<HashMap<_, _>>();

    // Create a threshold signer
    let cs = BlsPairingSigner::<ark_bn254::Bn254>::new(sk);
    let ts = BlsThresholdSigner::new_with_cache_size(
        cs.clone(),
        config.key_config.n.get(),
        config.key_config.t.get(),
        config.key_config.node_id.get(),
        Default::default(), // no pk on g1
        pks_g2,
        config.lru_cache_size,
    )
    .with_eager_signing();

    // Create a libp2p transport and start it
    let mut libp2p_node = Libp2pNodeConfig::new(
        config.libp2p.libp2p_key.clone().into(),
        config.key_config.node_id.get(),
        addresses,
        peer_ids,
        short_ids,
    )
    .run(config.libp2p.libp2p_listen_addr.clone())?;

    let (ts_stopper, async_signer) = ts.run(
        libp2p_node
            .get_transport()
            .expect("newly created node should have a transport"),
    );

    Ok((libp2p_node, ts_stopper, async_signer))
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let DSignerConfig {
        config,
        nodes_config,
    } = DSignerConfig::parse()?;

    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::from(&config.log_level))
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Initialize and start a threshold signer
    let (libp2p_node, ts_cancel, async_signer) =
        get_signer(&config, &nodes_config.unwrap_or_default())?;

    let alg = SignatureAlgorithm::Bls(BlsSignatureAlgorithm {
        curve: BlsSignatureCurve::Bn254G1,
        hash: BlsSignatureHash::Keccak256,
    });
    let args = ApplicationArgs::Any(ApplicationAnyArgs {
        dst_suffix: config.key_config.dst_suffix,
    });

    let params = async_signer
        .verification_parameters(&alg, &args)
        .context("failed to obtain dsigner verification parameters")?;
    let group_pk: ark_bn254::G2Affine = PointDeserializeCompressed::deser(&params.public_key)
        .context("failed to deserialize dsigner public key")?;

    // Convert group pk to string
    let (&x, &y) = group_pk.xy().context("dsigner public key at infinity")?;
    let pk_ser = [
        x.c1.into_bigint().to_bytes_be(),
        x.c0.into_bigint().to_bytes_be(),
        y.c1.into_bigint().to_bytes_be(),
        y.c0.into_bigint().to_bytes_be(),
    ]
    .concat();
    let pk = PublicKey(hex::encode(pk_ser));

    // Initialize application state
    let app_state = Arc::new(AppState {
        async_signer,
        dst: String::from_utf8(params.dst.to_vec())?,
        alg,
        args,
        pk,
    });

    // Build our application with a single route
    let app = Router::new()
        .route("/sign", post(sign_handler))
        .route("/pk", get(pk_handler))
        .route("/healthcheck", get(healthcheck_handler))
        .layer(TraceLayer::new_for_http())
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

    if let Err(e) = libp2p_node.stop().await {
        tracing::error!(error = ?e, "Failed to stop libp2p node");
    }
    ts_cancel.cancel();
    Ok(res?)
}
