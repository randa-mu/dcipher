//! CLI tool to start ADKG ceremonies

mod adkg_dyx20;
mod cli;
mod keygen;
#[cfg(feature = "metrics")]
mod metrics;
mod scheme;
mod transcripts;

use crate::adkg_dyx20::adkg_dyx20_bn254_g1_keccak256;
use crate::cli::{Cli, Commands, Generate, NewScheme, RunAdkg};
use crate::keygen::{PrivateKeyMaterial, PublicKeyMaterial, keygen};
use crate::scheme::new_scheme_config;
use adkg::adkg::AdkgOutput;
use adkg::helpers::PartyId;
use adkg::rand::AdkgStdRng;
use adkg::scheme::bn254::DYX20Bn254G1Keccak256;
use adkg::scheme::{AdkgScheme, AdkgSchemeConfig};
use anyhow::{Context, anyhow};
use ark_ec::CurveGroup;
use ark_std::rand;
use clap::Parser;
use dcipher_network::topic::dispatcher::{TopicBasedTransportImpl, TopicDispatcher};
use dcipher_network::transports::libp2p::transport::Libp2pSender;
use dcipher_network::transports::libp2p::{Libp2pNode, Libp2pNodeConfig};
use dcipher_network::transports::writer;
use dcipher_network::transports::writer::TransportWriter;
use dcipher_network::transports::writer::TransportWriterSender;
use itertools::Itertools;
use libp2p::{Multiaddr, PeerId};
use rand::rngs::OsRng;
use serde::{Deserialize, Serialize};
use std::fs;
use std::num::NonZeroUsize;
use std::path::PathBuf;
use std::str::FromStr;
use std::sync::Arc;
use std::time::Duration;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use utils::serialize::fq::FqSerialize;
use utils::serialize::point::PointSerializeCompressed;

#[derive(Clone, Debug, Serialize, Deserialize)]
struct NodeDetail {
    id: NonZeroUsize,
    #[serde(flatten)]
    public_key_material: PublicKeyMaterial,
    multiaddr: Multiaddr,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct GroupConfig {
    n: NonZeroUsize,
    t: NonZeroUsize,
    start_time: chrono::DateTime<chrono::Utc>,
    nodes: Vec<NodeDetail>,
}

#[derive(Clone, Serialize, Deserialize)]
struct AdkgSecret {
    adkg_scheme_name: String,
    genesis_timestamp: i64,
    sk: String,
}

#[derive(Clone, Serialize, Deserialize)]
struct AdkgPublic {
    adkg_scheme_name: String,
    genesis_timestamp: i64,
    group_pk: String,
    node_pks: Vec<AdkgNodePk>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct AdkgNodePk {
    id: NonZeroUsize,
    pk: String,
    peer_id: PeerId,
    multiaddr: Multiaddr,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Parse the CLI arguments into the Cli struct
    let args = Cli::parse();

    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::from(&args.log_level))
        .with(tracing_subscriber::fmt::layer())
        .init();

    match args.command {
        Commands::NewScheme(args) => new_scheme(args)?,

        Commands::Generate(args) => generate(args)?,

        Commands::Run(args) => run_adkg(args).await?,
    }

    Ok(())
}

fn new_scheme(args: NewScheme) -> anyhow::Result<()> {
    let NewScheme {
        scheme_id,
        app_name,
        scheme_out,
    } = args;

    let scheme_config = new_scheme_config(scheme_id, app_name)?;
    let scheme_config_toml = toml::to_string_pretty(&scheme_config)?;

    if let Some(out) = scheme_out {
        fs::write(&out, scheme_config_toml).context("Failed to write scheme config file")?;
        println!("Scheme configuration saved to {}", out.display());
    } else {
        println!("Scheme configuration:\n");
        println!("{scheme_config_toml}");
    }

    Ok(())
}

fn generate(args: Generate) -> anyhow::Result<()> {
    let Generate {
        scheme,
        priv_out,
        pub_out,
    } = args;

    let scheme_config = toml::from_str(&fs::read_to_string(&scheme)?)?;
    let (sk, pk) = keygen(scheme_config)?;
    let sk_toml = toml::to_string_pretty(&sk)?;
    let pk_toml = toml::to_string_pretty(&pk)?;

    fs::write(&priv_out, sk_toml).context("Failed to write private key file")?;
    println!("Private key material saved to {}", priv_out.display());

    if let Some(out) = pub_out {
        fs::write(&out, pk_toml).context("Failed to write public key file")?;
        println!("Public key material saved to {}", out.display());
    } else {
        println!("Public key material:\n");
        println!("{pk_toml}");
    }

    Ok(())
}

async fn run_adkg(args: RunAdkg) -> anyhow::Result<()> {
    let RunAdkg {
        scheme,
        group_file,
        priv_file,
        id,
        listen_address,
        timeout,
        grace_period,
        priv_out,
        pub_out,
        transcript_out,
        #[cfg(feature = "metrics")]
        metrics_params,
    } = args;

    // Make sure priv_out / pub_out do not exist
    if priv_out.exists() {
        Err(anyhow!(
            "priv_out file already exists, refusing to overwrite"
        ))?
    }
    if pub_out.exists() {
        Err(anyhow!(
            "pub_out file already exists, refusing to overwrite"
        ))?
    }

    // Make sure priv_out / pub_out are writable
    fs::write(&priv_out, "").context("failed to write private key file")?;
    fs::write(&pub_out, "").context("failed to write public key file")?;

    // Deserialize the configs
    let scheme_config: AdkgSchemeConfig =
        toml::from_str(&fs::read_to_string(&scheme).context("failed to read scheme file")?)
            .context("failed to parse scheme config")?;

    let group_config = GroupConfig::from_str(
        &fs::read_to_string(&group_file).context("failed to read group file")?,
    )
    .context("failed to parse group config")?;

    let sk: PrivateKeyMaterial = toml::from_str(
        &fs::read_to_string(priv_file).context("failed to read private key material")?,
    )
    .context("failed to parse private key material")?;

    let id = PartyId(id.get());
    let adkg_scheme_name = scheme_config.adkg_scheme_name.clone();
    let mut rng = AdkgStdRng::new(OsRng);

    // Start metrics server if enabled
    #[cfg(feature = "metrics")]
    adkg_metrics(&metrics_params);

    // Start libp2p transport
    let transports = get_libp2p_transports(id, &sk, listen_address, &group_config).await?;

    let output = match scheme_config.adkg_scheme_name.as_str() {
        <DYX20Bn254G1Keccak256 as AdkgScheme>::NAME => {
            adkg_dyx20_bn254_g1_keccak256(
                id,
                &sk.adkg_sk,
                &group_config,
                scheme_config,
                grace_period,
                timeout,
                transports.topic_transport.clone(),
                Some(transports.writer),
                &mut rng,
            )
            .await
        }

        _ => Err(anyhow!("Unsupported adkg scheme"))?,
    };

    tracing::info!("Stopping libp2p dispatcher...");
    transports.topic_dispatcher.stop().await;

    tracing::info!("Stopping libp2p transport...");
    if let Err(e) = transports.node.stop().await {
        tracing::error!(error = ?e, "Failed to stop libp2p node");
    }

    match output {
        Ok((adkg_out, opt_transcript)) => {
            tracing::info!(used_sessions = ?adkg_out.used_sessions, "Successfully obtained secret key from ADKG");
            write_adkg_keys(
                adkg_out,
                &priv_out,
                &pub_out,
                adkg_scheme_name,
                &group_config,
            )?;

            if let Some(transcript_out) = transcript_out {
                if let Some(transcript) = opt_transcript {
                    if let Err(e) = fs::write(&transcript_out, transcript) {
                        tracing::error!(error = ?e, transcript_out = %transcript_out.display(), "Failed to write transcript to file");
                    } else {
                        tracing::info!(transcript_out = %transcript_out.display(), "Successfully wrote transcript to file");
                    }
                } else {
                    tracing::error!(
                        "transcript_out file specified, but ADKG returned an empty transcript.."
                    );
                }
            }
        }
        Err(e) => {
            tracing::info!(error = ?e, "Failed to execute ADKG");
            Err(e)?
        }
    }

    Ok(())
}

#[cfg(feature = "metrics")]
fn adkg_metrics(metrics_params: &cli::MetricsParams) {
    tokio::task::spawn(metrics::start_metrics_api(
        metrics_params.metrics_listen_addr,
        metrics_params.metrics_port,
    ));
}

/// Write the ADKG outputs in priv / pub files.
fn write_adkg_keys<CG>(
    output: AdkgOutput<CG>,
    priv_out: &PathBuf,
    pub_out: &PathBuf,
    adkg_scheme_name: String,
    group_config: &GroupConfig,
) -> anyhow::Result<()>
where
    CG: CurveGroup + PointSerializeCompressed,
    CG::ScalarField: FqSerialize,
{
    let genesis_timestamp = group_config.start_time.timestamp();
    let secret = AdkgSecret {
        adkg_scheme_name: adkg_scheme_name.clone(),
        genesis_timestamp,
        sk: output
            .sk
            .ser_base64()
            .context("failed to serialize secret key")?,
    };
    fs::write(
        priv_out,
        toml::to_string_pretty(&secret).context("failed to serialize adkg secret")?,
    )
    .context("failed to write secret key file")?;

    if output.node_pks.is_none() || output.group_pk.is_none() {
        Err(anyhow!("group_pk / node pks is None"))?;
    }
    let group_pk = output
        .group_pk
        .unwrap()
        .ser_base64()
        .context("failed to serialize group pk")?;
    let node_pks = output
        .node_pks
        .unwrap()
        .into_iter()
        .zip(group_config.nodes.iter())
        .map(|(node_pk, n)| {
            Ok(AdkgNodePk {
                id: n.id,
                peer_id: n.public_key_material.peer_id,
                multiaddr: n.multiaddr.clone(),
                pk: node_pk
                    .ser_base64()
                    .context("failed to serialize group pk")?,
            })
        })
        .collect::<anyhow::Result<_>>()?;
    let public = AdkgPublic {
        adkg_scheme_name,
        genesis_timestamp,
        group_pk,
        node_pks,
    };
    fs::write(
        pub_out,
        toml::to_string_pretty(&public).context("failed to serialize adkg public keys")?,
    )
    .context("failed to write public key file")?;

    Ok(())
}

type TopicTransport = TopicBasedTransportImpl<
    TransportWriterSender<writer::InMemoryWriter<PartyId, Vec<u8>>, Libp2pSender<PartyId>>,
>;

type InMemoryWriter = writer::InMemoryWriter<PartyId, Vec<u8>>;

struct Libp2pTransports {
    node: Libp2pNode<PartyId>,
    writer: InMemoryWriter,
    topic_dispatcher: TopicDispatcher,
    topic_transport: Arc<TopicTransport>,
}

async fn get_libp2p_transports(
    id: PartyId,
    sk: &PrivateKeyMaterial,
    listen_addr: Multiaddr,
    group_config: &GroupConfig,
) -> anyhow::Result<Libp2pTransports> {
    // Make sure that the identifiers are unique
    let (peer_addrs, peer_ids, short_ids): (Vec<_>, Vec<_>, Vec<_>) = group_config
        .nodes
        .iter()
        .map(|p| {
            (
                p.multiaddr.to_owned(),
                p.public_key_material.peer_id,
                PartyId(p.id.get()),
            )
        })
        .collect();

    let mut node = Libp2pNodeConfig::new(sk.libp2p_sk.clone(), id, peer_addrs, peer_ids, short_ids)
        .run(listen_addr)
        .map_err(|e| {
            tracing::error!("Failed to start libp2p network: {e:?}");
            e
        })?;

    // Start libp2p transport
    tracing::info!("Starting libp2p networking");
    let transport = node
        .get_transport()
        .ok_or(anyhow!("failed to get topic transport"))?;

    // Always create a writer for now, even if it ends up not being used if transcripts are disabled.
    // In the future, we probably want to return an enum dispatched type implementing [`TopicTransport`]
    // to support returning different implementations of [`TopicTransport`] at run-time, since [`TopicTransport`]
    // is not dyn-compatible.
    let transport_writer = TransportWriter::new_in_memory(transport);
    let writer = transport_writer.writer().to_owned();
    let mut topic_dispatcher = TopicDispatcher::new();
    let topic_transport = topic_dispatcher.start(transport_writer).into();

    tracing::info!("Waiting a few seconds for networking to settle...");
    tokio::time::sleep(Duration::from_secs(2)).await;

    Ok(Libp2pTransports {
        node,
        topic_transport,
        topic_dispatcher,
        writer,
    })
}

impl FromStr for GroupConfig {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut group_config: Self = toml::from_str(s).context("failed to parse group config")?;

        if group_config.n < group_config.t {
            Err(anyhow!("n cannot be smaller than t"))?;
        }

        if group_config.nodes.len() != group_config.n.get() {
            Err(anyhow!("number of nodes does not match n"))?;
        }

        if let Some(id) = group_config.nodes.iter().map(|n| n.id).duplicates().next() {
            Err(anyhow!("found node id {id} more than once"))?;
        }

        if let Some(peer_id) = group_config
            .nodes
            .iter()
            .map(|n| &n.public_key_material.peer_id)
            .duplicates()
            .next()
        {
            Err(anyhow!("found peer_id {peer_id} more than once"))?;
        }

        if let Some(adkg_pk) = group_config
            .nodes
            .iter()
            .map(|n| &n.public_key_material.adkg_pk)
            .duplicates()
            .next()
        {
            Err(anyhow!("found adkg_pk {adkg_pk} more than once"))?;
        }

        if let Some(multiaddr) = group_config
            .nodes
            .iter()
            .map(|n| &n.multiaddr)
            .duplicates()
            .next()
        {
            Err(anyhow!("found multiaddr {multiaddr} more than once"))?;
        }

        if chrono::Utc::now() >= group_config.start_time {
            Err(anyhow!("start time cannot be in the past"))?;
        }

        // Align the group config to a unix timestamp ending in 00
        let timestamp = group_config.start_time.timestamp();
        let timestamp_mod = timestamp % 100;
        let next_timestamp = if timestamp_mod == 0 {
            timestamp
        } else {
            timestamp + (100 - timestamp_mod)
        };

        group_config.start_time =
            chrono::DateTime::<chrono::Utc>::from_timestamp(next_timestamp, 0)
                .ok_or(anyhow!("failed to align unix timestamp"))?;

        // Sort the nodes
        group_config.nodes.sort_by(|p1, p2| p1.id.cmp(&p2.id));

        Ok(group_config)
    }
}
