//! CLI tool to start ADKG ceremonies

mod adkg_dxkr23;
mod cli;
mod keygen;
#[cfg(feature = "metrics")]
mod metrics;
mod scheme;
mod transcripts;

use crate::adkg_dxkr23::{
    adkg_dxkr23_bls12_381_g1_sha256_out_g2, adkg_dxkr23_bls12_381_g1_sha256_out_g2_rescue,
    adkg_dxkr23_bn254_g1_keccak256_out_g2, adkg_dxkr23_bn254_g1_keccak256_out_g2_rescue,
};
use crate::cli::{AdkgRunCommon, Cli, Commands, Generate, NewScheme, Rescue, RunAdkg};
use crate::keygen::keygen;
use crate::scheme::{AdkgCliSchemeConfig, SupportedAdkgScheme, new_scheme_config};
use crate::transcripts::EncryptedAdkgTranscript;
use adkg::helpers::PartyId;
use adkg::rand::AdkgStdRng;
use anyhow::{Context, anyhow};
use ark_ec::CurveGroup;
use ark_std::rand;
use clap::Parser;
use config::adkg::PrivateKeyMaterial;
use config::adkg::{AdkgNodePk, AdkgPublic, AdkgSecret, GroupConfig};
use dcipher_network::topic::dispatcher::{TopicBasedTransportImpl, TopicDispatcher};
use dcipher_network::transports::libp2p::transport::Libp2pSender;
use dcipher_network::transports::libp2p::{Libp2pNode, Libp2pNodeConfig};
use dcipher_network::transports::replayable::writer;
use dcipher_network::transports::replayable::writer::TransportWriter;
use dcipher_network::transports::replayable::writer::TransportWriterSender;
use libp2p::Multiaddr;
use rand::rngs::OsRng;
use std::fs;
use std::path::PathBuf;
use std::str::FromStr;
use std::sync::Arc;
use std::time::Duration;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use utils::serialize::fq::FqSerialize;
use utils::serialize::point::PointSerializeCompressed;

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

        Commands::Rescue(args) => rescue_adkg(args).await?,
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

/// Various params used by the ADKG
#[derive(Clone, Debug)]
struct AdkgConfig {
    id: PartyId,
    grace_period: Duration,
    timeout: Duration,
    priv_out: PathBuf,
    pub_out: PathBuf,
    transcript_out: Option<PathBuf>,
    scheme_name: String,
}

async fn run_adkg(args: RunAdkg) -> anyhow::Result<()> {
    let RunAdkg {
        common:
            AdkgRunCommon {
                scheme,
                group_file,
                priv_file,
                id,
                priv_out,
                pub_out,
            },
        listen_address,
        timeout,
        grace_period,
        transcript_out,
        #[cfg(feature = "metrics")]
        metrics_params,
    } = args;

    // Parse common inputs
    let (scheme_config, group_config, sk) =
        parse_adkg_common(&scheme, &group_file, &priv_file, &priv_out, &pub_out)?;

    if chrono::Utc::now() >= group_config.aligned_start_datetime()? {
        tracing::warn!("The start date specified in the group configuration is in the past");
    }

    let adkg_config = AdkgConfig {
        id: PartyId(id.get()),
        grace_period,
        timeout,
        priv_out,
        pub_out,
        transcript_out,
        scheme_name: scheme_config.adkg_scheme_name.clone(),
    };
    let adkg_scheme: SupportedAdkgScheme = scheme_config
        .adkg_scheme_name
        .parse()
        .context("adkg scheme not supported")?;
    let rng = AdkgStdRng::new(OsRng);

    // Start metrics server if enabled
    #[cfg(feature = "metrics")]
    adkg_metrics(&metrics_params);

    // Start libp2p transport
    let transports =
        get_libp2p_transports(adkg_config.id, &sk, listen_address, &group_config).await?;

    let adkg_res = match adkg_scheme {
        SupportedAdkgScheme::DXKR23Bn254G1Keccak256 => {
            adkg_dxkr23_bn254_g1_keccak256_out_g2(
                &sk.adkg_sk,
                adkg_config,
                &group_config,
                scheme_config,
                transports.topic_transport.clone(),
                Some(transports.writer),
                rng,
            )
            .await
        }

        SupportedAdkgScheme::DXKR23Bls12_381G1Sha256 => {
            adkg_dxkr23_bls12_381_g1_sha256_out_g2(
                &sk.adkg_sk,
                adkg_config,
                &group_config,
                scheme_config,
                transports.topic_transport.clone(),
                Some(transports.writer),
                rng,
            )
            .await
        }
    };

    if let Err(e) = adkg_res {
        tracing::error!(error = ?e, "ADKG returned an error");
    }

    tracing::info!("Stopping libp2p dispatcher...");
    transports.topic_dispatcher.stop().await;

    tracing::info!("Stopping libp2p transport...");
    if let Err(e) = transports.node.stop().await {
        tracing::error!(error = ?e, "Failed to stop libp2p node");
    }

    Ok(())
}

async fn rescue_adkg(args: Rescue) -> anyhow::Result<()> {
    let Rescue {
        common:
            AdkgRunCommon {
                scheme,
                group_file,
                priv_file,
                id,
                priv_out,
                pub_out,
            },
        transcript_files,
    } = args;

    // Parse common inputs
    let (scheme_config, group_config, sk) =
        parse_adkg_common(&scheme, &group_file, &priv_file, &priv_out, &pub_out)?;

    // Parse transcripts
    let transcripts = transcript_files
        .into_iter()
        .map(|transcript_file| {
            fs::read(&transcript_file).with_context(|| {
                format!("failed to read transcript `{}`", transcript_file.display())
            })
        })
        .collect::<Result<Vec<_>, _>>()?;
    if transcripts.len() < group_config.n.get() - group_config.t.get() {
        Err(anyhow!(
            "Not enough transcripts specified: number of transcripts = {}, expected at least n - t = {}",
            transcripts.len(),
            group_config.n.get() - group_config.t.get()
        ))?;
    }

    let adkg_config = AdkgConfig {
        id: PartyId(id.get()),
        grace_period: Duration::from_secs(0), // unused
        timeout: Duration::from_secs(0),      // unused
        priv_out,
        pub_out,
        transcript_out: None,
        scheme_name: scheme_config.adkg_scheme_name.clone(),
    };
    let adkg_scheme_name = scheme_config
        .adkg_scheme_name
        .parse()
        .context("adkg scheme not supported")?;
    let mut rng = AdkgStdRng::new(OsRng);
    let adkg_res = match adkg_scheme_name {
        SupportedAdkgScheme::DXKR23Bn254G1Keccak256 => {
            adkg_dxkr23_bn254_g1_keccak256_out_g2_rescue(
                &sk.adkg_sk,
                adkg_config,
                &group_config,
                scheme_config,
                transcripts,
                &mut rng,
            )
            .await
        }

        SupportedAdkgScheme::DXKR23Bls12_381G1Sha256 => {
            adkg_dxkr23_bls12_381_g1_sha256_out_g2_rescue(
                &sk.adkg_sk,
                adkg_config,
                &group_config,
                scheme_config,
                transcripts,
                &mut rng,
            )
            .await
        }
    };

    if let Err(e) = adkg_res {
        tracing::error!(error = ?e, "ADKG returned an error");
    }

    Ok(())
}

fn parse_adkg_common(
    scheme: &PathBuf,
    group_file: &PathBuf,
    priv_file: &PathBuf,
    priv_out: &PathBuf,
    pub_out: &PathBuf,
) -> anyhow::Result<(AdkgCliSchemeConfig, GroupConfig, PrivateKeyMaterial)> {
    // Deserialize the configs
    let scheme_config: AdkgCliSchemeConfig =
        toml::from_str(&fs::read_to_string(scheme).context("failed to read scheme file")?)
            .context("failed to parse scheme config")?;

    let group_config = GroupConfig::from_str(
        &fs::read_to_string(group_file).context("failed to read group file")?,
    )
    .context("failed to parse group config")?;

    let sk: PrivateKeyMaterial = toml::from_str(
        &fs::read_to_string(priv_file).context("failed to read private key material")?,
    )
    .context("failed to parse private key material")?;

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
    fs::write(priv_out, "").context("failed to write private key file")?;
    fs::write(pub_out, "").context("failed to write public key file")?;

    Ok((scheme_config, group_config, sk))
}

#[cfg(feature = "metrics")]
fn adkg_metrics(metrics_params: &cli::MetricsParams) {
    tokio::task::spawn(metrics::start_metrics_api(
        metrics_params.metrics_listen_addr,
        metrics_params.metrics_port,
    ));
}

#[derive(Clone, Debug)]
struct AdkgPubOutput<CG> {
    pub group_pk: Option<CG>,
    pub node_pks: Option<Vec<CG>>,
}

/// The adkg output with public outputs on the source group of the ADKG, and on a destination group
/// used for signatures.
#[derive(Clone)]
struct AdkgOutputDual<CGSource, CGDest>
where
    CGSource: CurveGroup,
    CGDest: CurveGroup<ScalarField = CGSource::ScalarField>,
{
    sk: CGSource::ScalarField,
    out_pub_source: AdkgPubOutput<CGSource>,
    out_pub_dest: Option<AdkgPubOutput<CGDest>>,
}

/// Write the adkg transcript
fn write_transcript(
    transcript_out: &PathBuf,
    transcript: EncryptedAdkgTranscript,
) -> anyhow::Result<()> {
    if let Err(e) = fs::write(transcript_out, transcript) {
        Err(e).context("failed to write ADKG transcript")?
    }

    Ok(())
}

/// Write the ADKG outputs in priv / pub files.
fn write_adkg_keys<CGSource, CGDest>(
    outputs: &AdkgOutputDual<CGSource, CGDest>,
    priv_out: &PathBuf,
    pub_out: &PathBuf,
    adkg_scheme_name: String,
    group_config: &GroupConfig,
) -> anyhow::Result<()>
where
    CGSource: CurveGroup + PointSerializeCompressed,
    CGSource::ScalarField: FqSerialize,
    CGDest: CurveGroup<ScalarField = CGSource::ScalarField> + PointSerializeCompressed,
{
    let genesis_timestamp = group_config.start_time.timestamp();
    let secret = AdkgSecret {
        adkg_scheme_name: adkg_scheme_name.clone(),
        genesis_timestamp,
        sk: outputs
            .sk
            .ser_base64()
            .context("failed to serialize secret key")?,
    };
    fs::write(
        priv_out,
        toml::to_string_pretty(&secret).context("failed to serialize adkg secret")?,
    )
    .context("failed to write secret key file")?;

    /// Serialize public parameters
    fn ser_pub_params<CG: PointSerializeCompressed>(
        outputs: &AdkgPubOutput<CG>,
        group_config: &GroupConfig,
    ) -> anyhow::Result<(String, Vec<AdkgNodePk>)> {
        if outputs.node_pks.is_none() || outputs.group_pk.is_none() {
            Err(anyhow!("group_pk / node pks is None"))?;
        }
        let group_pk = outputs
            .group_pk
            .as_ref()
            .unwrap()
            .ser_compressed_base64()
            .context("failed to serialize group pk")?;
        let node_pks = outputs
            .node_pks
            .as_ref()
            .unwrap()
            .iter()
            .zip(group_config.nodes.iter())
            .map(|(node_pk, n)| {
                Ok(AdkgNodePk {
                    id: n.id,
                    peer_id: n.public_key_material.peer_id,
                    multiaddr: n.multiaddr.clone(),
                    pk: node_pk
                        .ser_compressed_base64()
                        .context("failed to serialize group pk")?,
                })
            })
            .collect::<anyhow::Result<_>>()?;

        Ok((group_pk, node_pks))
    }

    // Serialize adkg source group, or get defaults if it fails
    let (group_pk_source, node_pks_source) = ser_pub_params(&outputs.out_pub_source, group_config)
        .unwrap_or_else(|e| {
            tracing::error!(error = ?e, "Failed to serialize public output of adkg source group");
            Default::default()
        });

    // Serialize adkg destination group, or get defaults if it fails
    let (group_pk, node_pks) = outputs
        .out_pub_dest
        .as_ref()
        .map(|out| ser_pub_params(out, group_config))
        .unwrap_or(Ok(Default::default()))
        .unwrap_or_else(|e| {
            tracing::error!(error = ?e, "Failed to serialize public output of adkg destination group");
            Default::default()
        });

    let public = AdkgPublic {
        adkg_scheme_name,
        genesis_timestamp,
        group_pk,
        group_pk_source,
        node_pks,
        node_pks_source,
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
