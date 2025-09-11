//! Provides implementations to execute a high-threshold asynchronous distributed key generation using
//! the scheme described in [Practical Asynchronous High-threshold Distributed Key Generation and Distributed Polynomial Sampling](https://www.usenix.org/system/files/usenixsecurity23-das.pdf)
//! by Das et al.

use crate::config::GroupConfig;
use crate::scheme::AdkgCliSchemeConfig;
use crate::transcripts::{
    BroadcastMessages, DirectMessages, EncryptedAdkgTranscript, SerializedBytes,
};
use crate::{
    AdkgConfig, AdkgOutputDual, AdkgPubOutput, InMemoryWriter, write_adkg_keys, write_transcript,
};
use adkg::aba::AbaConfig;
use adkg::adkg::{AbaCrainInput, AdkgOutput, ShareWithPoly};
use adkg::helpers::{PartyId, lagrange_points_interpolate_at, u64_from_usize};
use adkg::pke::ec_hybrid_chacha20poly1305;
use adkg::pke::ec_hybrid_chacha20poly1305::{
    HybridCiphertext, MultiHybridCiphertext, NONCE_LENGTH,
};
use adkg::rand::AdkgRng;
use adkg::scheme::DXKR23AdkgScheme;
use adkg::scheme::bls12_381::DXKR23Bls12_381G1Sha256;
use adkg::scheme::bn254::DXKR23Bn254G1Keccak256;
use adkg::vss::acss::AcssConfig;
use anyhow::{Context, anyhow};
use ark_ec::pairing::Pairing;
use ark_ec::{AffineRepr, CurveGroup, Group};
use ark_std::Zero;
use ark_std::iterable::Iterable;
use chacha20poly1305::aead::Aead;
use chacha20poly1305::{AeadCore, ChaCha20Poly1305, Key, KeyInit, Nonce};
use dcipher_network::topic::TopicBasedTransport;
use dcipher_network::topic::dispatcher::TopicDispatcher;
use dcipher_network::transports::replayable::reader::InMemoryReaderTransport;
use dcipher_network::transports::replayable::writer::{InMemoryEntry, InMemoryEntryType};
use dcipher_network::{ReceivedMessage, Recipient, Transport, TransportSender};
use futures_util::StreamExt;
use itertools::Itertools;
use rand::{CryptoRng, Rng, thread_rng};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::ops::Neg;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::oneshot;
use utils::dst::{NamedCurveGroup, NamedDynDigest};
use utils::hash_to_curve::HashToCurve;
use utils::serialize::fq::{FqDeserialize, FqSerialize};
use utils::serialize::point::{PointDeserializeCompressed, PointSerializeCompressed};

const TOPIC_SWAP_G1_TO_G2: &str = "adkg_dxkr23_swap_g1_to_g2";

/// Run adkg for BN254 on G1, and swap ADKG output on G2
pub async fn adkg_dxkr23_bn254_g1_keccak256_out_g2<TBT>(
    adkg_sk: &str,
    adkg_config: AdkgConfig,
    group_config: &GroupConfig,
    scheme_config: AdkgCliSchemeConfig,
    topic_transport: Arc<TBT>,
    writer: Option<InMemoryWriter>,
    rng: impl AdkgRng + 'static,
) -> anyhow::Result<()>
where
    TBT: TopicBasedTransport<Identity = PartyId> + Send + Sync + 'static,
{
    let output_generator_g2 = scheme_config.output_generator;
    let scheme = DXKR23Bn254G1Keccak256::try_from(scheme_config.adkg_config)?;
    adkg_pairing_out_g2::<ark_bn254::Bn254, _, _>(
        adkg_sk,
        adkg_config,
        group_config,
        &output_generator_g2,
        scheme,
        topic_transport,
        writer,
        rng,
    )
    .await
}

/// Run adkg for Bls12-381 on G1, and swap ADKG output on G2
#[allow(clippy::too_many_arguments)]
pub async fn adkg_dxkr23_bls12_381_g1_sha256_out_g2<TBT>(
    adkg_sk: &str,
    adkg_config: AdkgConfig,
    group_config: &GroupConfig,
    scheme_config: AdkgCliSchemeConfig,
    topic_transport: Arc<TBT>,
    writer: Option<InMemoryWriter>,
    rng: impl AdkgRng + 'static,
) -> anyhow::Result<()>
where
    TBT: TopicBasedTransport<Identity = PartyId> + Send + Sync + 'static,
{
    let output_generator_g2 = scheme_config.output_generator;
    let scheme = DXKR23Bls12_381G1Sha256::try_from(scheme_config.adkg_config)?;
    adkg_pairing_out_g2::<ark_bls12_381::Bls12_381, _, _>(
        adkg_sk,
        adkg_config,
        group_config,
        &output_generator_g2,
        scheme,
        topic_transport,
        writer,
        rng,
    )
    .await
}

/// Execute the adkg on g1, and then the swapping protocol to write an adkg output on both g1 & g2
///
/// This protocol is executed in the following stages:
///  1. Execute standard ADKG on G1
///     1a. The ADKG has sent an output through the oneshot channel, continue to 2.
///     1b. The ADKG has timed out, or returned an error => exit now
///  2. Write the priv/pub output to the specified files
///  3. Execute the G1 to G2 swap protocol
///     3.a. The swap protocol completes, write the priv/pub output to the specified files, continue to 4.
///     3.b. The ADKG task is finished, continue to 4. <-- The swap protocol has failed.
///  4. Wait for the ADKG task to complete its grace period
///  5. Write the transcripts to disk
#[allow(clippy::too_many_arguments)]
async fn adkg_pairing_out_g2<'a, E, S, TBT>(
    adkg_sk: &str,
    adkg_config: AdkgConfig,
    group_config: &GroupConfig,
    g2: &str,
    adkg_scheme: S,
    topic_transport: Arc<TBT>,
    writer: Option<InMemoryWriter>,
    mut rng: impl AdkgRng + 'static,
) -> anyhow::Result<()>
where
    E: Pairing,
    E::ScalarField: FqSerialize + FqDeserialize,
    E::G1: PointSerializeCompressed + PointDeserializeCompressed,
    E::G2: PointSerializeCompressed + PointDeserializeCompressed,
    S: DXKR23AdkgScheme<Curve = E::G1>,
    S::Curve: NamedCurveGroup,
    S::Hash: NamedDynDigest,
    S::ABAConfig: AbaConfig<'static, PartyId, Input = AbaCrainInput<S::Curve>>,
    <S::ACSSConfig as AcssConfig<'static, S::Curve, PartyId>>::Output:
        Into<ShareWithPoly<S::Curve>>,
    TBT: TopicBasedTransport<Identity = PartyId> + Send + Sync + 'static,
{
    let sk = E::ScalarField::deser_base64(adkg_sk)?;
    let pks = group_config
        .nodes
        .iter()
        .map(|p| S::Curve::deser_compressed_base64(&p.public_key_material.adkg_pk))
        .collect::<Result<Vec<_>, _>>()?;

    let transport = topic_transport
        .get_transport_for(TOPIC_SWAP_G1_TO_G2)
        .context("failed to obtain transport")?;
    let t_reconstruction = group_config.t_reconstruction.get();
    let g = adkg_scheme.generator_g();
    let g2 = E::G2::deser_compressed_base64(g2)?;

    // Spawn a task to run the adkg in the background.
    let (tx_adkg_out, rx_adkg_out) = oneshot::channel();
    let mut adkg_task: tokio::task::JoinHandle<anyhow::Result<()>> = tokio::spawn({
        let pks = pks.clone();
        let adkg_config = adkg_config.clone();
        let group_config = group_config.to_owned();
        async move {
            adkg_dxkr23(
                sk,
                pks.clone(),
                adkg_config,
                group_config,
                topic_transport,
                adkg_scheme,
                &mut rng,
                tx_adkg_out,
            )
            .await
        }
    });

    // Execute the adkg task until it errors, or we get the adkg output through the oneshot channel
    let adkg_out = tokio::select! {
        join_res = &mut adkg_task => {
            match join_res {
                Ok(Ok(())) => {
                    tracing::error!("ADKG task exited with Ok before returning an output??");
                    anyhow::bail!("Failed to obtain ADKG output: exited early")
                },
                Ok(Err(e)) => {
                    tracing::error!(error = ?e, "ADKG task exited with an error");
                    Err(e).context("Failed to obtain ADKG output: exited early with an error")
                },
                Err(e) => {
                    tracing::error!(error = ?e, "Failed to join ADKG Task");
                    Err(e).context("Failed to join ADKG task")
                }
            }
        }

        adkg_out = rx_adkg_out => {
            match adkg_out {
                Ok(adkg_out) => Ok(adkg_out),
                Err(_) => {
                    anyhow::bail!("Failed to obtain ADKG output: sender channel dropped")
                }
            }
        }
    }?; // Leave now if adkg has failed, nothing that can be done

    // Save the initial adkg output
    let adkg_pub_out = AdkgPubOutput {
        node_pks: adkg_out.node_pks.clone(),
        group_pk: adkg_out.group_pk,
    };
    let mut adkg_dual_out = AdkgOutputDual::<E::G1, E::G2> {
        sk: adkg_out.sk,
        out_pub_source: adkg_pub_out.clone(),
        out_pub_dest: None,
    };
    if let Err(e) = write_adkg_keys(
        &adkg_dual_out,
        &adkg_config.priv_out,
        &adkg_config.pub_out,
        adkg_config.scheme_name.clone(),
        group_config,
    ) {
        tracing::error!(error = ?e, "Failed to save initial adkg output");
    }

    // We have an ADKG output, keep running it in the background, and execute the pairing swap protocol
    tokio::select! {
        join_res = &mut adkg_task => {
            match join_res {
                Ok(Ok(())) => {
                    tracing::error!("Failed to execute g1 to g2 swap within grace period");
                },
                Ok(Err(e)) => {
                    tracing::error!(error = ?e, "ADKG task exited with an error");
                },
                Err(e) => {
                    tracing::error!(error = ?e, "Failed to join ADKG Task");
                }
            }
        }

        adkg_out_g2 = pairing_swap_g1_to_g2::<E, _>(t_reconstruction, adkg_out, &g, &g2, transport) => {
            match adkg_out_g2 {
                Ok(out_g2) => {
                    // We got an adkg output on g2, re-write the output files with the new keys
                    adkg_dual_out.out_pub_dest = Some(AdkgPubOutput {
                        group_pk: out_g2.group_pk,
                        node_pks: out_g2.node_pks,
                    });

                    if let Err(e) = write_adkg_keys(
                        &adkg_dual_out,
                        &adkg_config.priv_out,
                        &adkg_config.pub_out,
                        adkg_config.scheme_name.clone(),
                        group_config,
                    ) {
                        tracing::error!(error = ?e, "Failed to save final adkg output");
                    }
                },
                Err(e) => {
                    tracing::error!(error = ?e, "Failed to swap adkg output from G1 to G2");
                }
            }
        }
    }

    if !adkg_task.is_finished() {
        // adkg still not finished, wait till it completes
        if let Err(e) = adkg_task.await {
            tracing::error!(error = ?e, "ADKG completed with an error");
        }
    }

    // Finally, save a transcript if writer is some
    if let Some(writer) = writer {
        let transcript = encrypt_transcripts(
            adkg_config.id,
            group_config,
            &writer,
            &sk,
            &pks,
            &mut thread_rng(),
        )
        .await
        .map_err(|e| {
            tracing::error!(error = ?e, "Failed to generate ADKG transcript");
            e
        })?;

        if let Some(transcript_out) = &adkg_config.transcript_out {
            if let Err(e) = write_transcript(transcript_out, transcript) {
                tracing::error!(error = ?e, transcript_out = %transcript_out.display(), "Failed to write transcript to file");
            } else {
                tracing::info!(transcript_out = %transcript_out.display(), "Successfully wrote transcript to file");
            }
        }
    }

    Ok(())
}

/// Execute the DXKR23 HT-ADKG protocol to obtain a shared secret, or until the `adkg_timeout` expires.
/// Once the output is obtained, it is sent through the `out` channel, but the ADKG continues to run for
/// the duration of the `adkg_grace_period`.
#[allow(clippy::too_many_arguments)]
async fn adkg_dxkr23<S, TBT>(
    sk: <<S::Curve as CurveGroup>::Affine as AffineRepr>::ScalarField,
    pks: Vec<S::Curve>,
    adkg_config: AdkgConfig,
    group_config: GroupConfig,
    topic_transport: Arc<TBT>,
    adkg_scheme: S,
    rng: &mut impl AdkgRng,
    out: oneshot::Sender<AdkgOutput<S::Curve>>,
) -> anyhow::Result<()>
where
    S: DXKR23AdkgScheme,
    S::Curve: NamedCurveGroup,
    S::Hash: NamedDynDigest,
    S::ABAConfig: AbaConfig<'static, PartyId, Input = AbaCrainInput<S::Curve>>,
    <S::ACSSConfig as AcssConfig<'static, S::Curve, PartyId>>::Output:
        Into<ShareWithPoly<S::Curve>>,
    TBT: TopicBasedTransport<Identity = PartyId>,
{
    let mut adkg = adkg_scheme.new_adkg(
        adkg_config.id,
        group_config.n,
        group_config.t,
        group_config.t_reconstruction,
        sk,
        pks.clone(),
    )?;

    // Calculate time to sleep before actively executing the adkg
    let sleep_duration = (group_config.start_time - chrono::Utc::now())
        .to_std() // TimeDelta to positive duration
        .unwrap_or_else(|_| Duration::from_secs(0));

    tracing::info!(
        "Sleeping for {} before starting ADKG at {}",
        humantime::format_duration(sleep_duration),
        humantime::format_rfc3339(group_config.start_time.into()),
    );
    tokio::time::sleep(sleep_duration).await;

    // Start the ADKG and wait until we obtain a share, or the timeout occurs
    tracing::info!(
        "Executing ADKG with a timeout of {}",
        humantime::format_duration(adkg_config.timeout)
    );

    let res = tokio::select! {
        output = adkg.start(rng, topic_transport) => {
            let output = match output {
                Ok(adkg_out) => {
                    tracing::info!(used_sessions = ?adkg_out.used_sessions, "Successfully obtained secret key & output from ADKG");
                    if out.send(adkg_out).is_err() {
                        // fails if the receiver side is dropped early
                        tracing::error!("Failed to send ADKG output through sender channel: channel closed");
                    }

                    tracing::info!("Running ADKG until grace period of {}", humantime::format_duration(adkg_config.grace_period));
                    tokio::time::sleep(adkg_config.grace_period).await;
                    Ok(())
                }
                Err(e) => {
                    tracing::error!("failed to obtain output from ADKG: {e:?}");
                    Err(e)
                }
            };

            Ok(output)
        }

        _ = tokio::time::sleep(adkg_config.timeout) => {
            println!("Aborting ADKG due to timeout");
            Err(anyhow!("ADKG has timed out"))
        }
    };

    tracing::warn!("Stopping ADKG...");
    adkg.stop().await;

    Ok(res??)
}

/// Pairing-based DLEQ proof that there exists an s_j s.t. P_1 = [s_j] G_1 \land P_2 = [s_j] G_2,
/// using witness s and proof \pi = P_2.
#[derive(Serialize, Deserialize)]
#[serde(bound(
    serialize = "CG: PointSerializeCompressed",
    deserialize = "CG: PointDeserializeCompressed"
))]
struct AdkgSwapPairingGroupMessage<CG>
where
    CG: CurveGroup,
{
    #[serde(with = "utils::serialize::point::base64")]
    g2_sj: CG,
}

/// Using an ADKG output, swap the public keys to a different generator of the same group.
/// This protocol uses a pairing-based DLEQ proof to swap all the public keys from one group to another.
async fn pairing_swap_g1_to_g2<E, T>(
    t_reconstruction: usize,
    adkg_output: AdkgOutput<E::G1>,
    g1: &E::G1,
    g2: &E::G2,
    mut transport: T,
) -> anyhow::Result<AdkgOutput<E::G2>>
where
    E: Pairing,
    E::G2: CurveGroup + PointSerializeCompressed + PointDeserializeCompressed,
    T: Transport<Identity = PartyId>,
{
    let node_pks = &adkg_output
        .node_pks
        .ok_or(anyhow!("cannot swap group without node pks"))?;

    let sender = transport
        .sender()
        .ok_or(anyhow!("failed to obtain transport sender"))?;
    let mut receiver = transport
        .receiver_stream()
        .ok_or(anyhow!("failed to obtain transport sender"))?;

    // Generate the public key on G2. This also corresponds to a DLEQ proof that can be verified by
    // checking that e([s] G_1, G_2) == e(G_1, [s] G_2) where [s] G_1 is the public key output by the
    // ADKG, G_1, G_2 are public parameters.
    let dleq_m = AdkgSwapPairingGroupMessage {
        g2_sj: *g2 * adkg_output.sk,
    };
    let dleq_m = bson::to_vec(&dleq_m)?;
    if let Err(e) = sender.send(dleq_m, Recipient::AllIncludingSelf).await {
        tracing::error!(error = ?e, "Failed to send dleq group swap message")
    }

    // Collect at least t_reconstruction + 1 valid evals to reconstruct the swapped group public key
    let mut dleq_msgs = BTreeMap::new();
    loop {
        let ReceivedMessage {
            sender, content, ..
        } = match receiver.next().await {
            Some(Ok(msg)) => msg,
            Some(Err(e)) => {
                tracing::error!(error = ?e, "Failed to receive dleq message");
                continue;
            }
            None => {
                anyhow::bail!("Stream closed: no more dleq message to receive")
            }
        };

        let dleq_j: AdkgSwapPairingGroupMessage<E::G2> = match bson::from_slice(&content) {
            Ok(dleq_j) => dleq_j,
            Err(e) => {
                tracing::warn!(error = ?e, "Failed to decode dleq message");
                continue;
            }
        };

        let Some(g1_sj) = node_pks.get(sender.as_index()) else {
            anyhow::bail!("adkg output's node_pks missing some ids")
        };

        // Verify the dleq proof with a pairing operation
        if !E::multi_pairing([*g1, *g1_sj], [dleq_j.g2_sj, g2.neg()]).is_zero() {
            tracing::warn!(?sender, "Failed to verify adkg swap dleq proof");
            continue;
        }

        // Valid keys, insert.
        dleq_msgs.insert(sender, dleq_j);
        #[allow(clippy::int_plus_one)]
        if dleq_msgs.len() >= t_reconstruction + 1 {
            // Enough messages, we can interpolate the remaining public keys, and the group public key
            let points: Vec<_> = dleq_msgs
                .iter()
                .map(|(&j, dleq_j)| (j.into(), dleq_j.g2_sj))
                .collect();

            let group_pk = lagrange_points_interpolate_at(&points, 0);
            let node_pks = node_pks
                .iter()
                .enumerate()
                .map(|(j, _)| {
                    let j_node_idx = j + 1;
                    if let Some(pk_j) = dleq_msgs.get(&PartyId::from(j_node_idx)) {
                        pk_j.g2_sj
                    } else {
                        lagrange_points_interpolate_at(&points, u64_from_usize(j_node_idx))
                    }
                })
                .collect();

            let adkg_out = AdkgOutput {
                sk: adkg_output.sk,
                used_sessions: adkg_output.used_sessions,
                node_pks: Some(node_pks),
                group_pk: Some(group_pk),
            };
            return Ok(adkg_out);
        }
    }
}

pub async fn adkg_dxkr23_bn254_g1_keccak256_out_g2_rescue(
    adkg_sk: &str,
    adkg_config: AdkgConfig,
    group_config: &GroupConfig,
    scheme_config: AdkgCliSchemeConfig,
    transcripts: Vec<EncryptedAdkgTranscript>,
    rng: &mut impl AdkgRng,
) -> anyhow::Result<()> {
    let scheme = DXKR23Bn254G1Keccak256::try_from(scheme_config.adkg_config)?;
    adkg_pairing_swap_g1_to_g2_rescue::<ark_bn254::Bn254, _>(
        adkg_sk,
        &scheme_config.output_generator,
        adkg_config,
        group_config,
        transcripts,
        rng,
        scheme,
    )
    .await
}

pub async fn adkg_dxkr23_bls12_381_g1_sha256_out_g2_rescue(
    adkg_sk: &str,
    adkg_config: AdkgConfig,
    group_config: &GroupConfig,
    scheme_config: AdkgCliSchemeConfig,
    transcripts: Vec<EncryptedAdkgTranscript>,
    rng: &mut impl AdkgRng,
) -> anyhow::Result<()> {
    let scheme = DXKR23Bls12_381G1Sha256::try_from(scheme_config.adkg_config)?;
    adkg_pairing_swap_g1_to_g2_rescue::<ark_bls12_381::Bls12_381, _>(
        adkg_sk,
        &scheme_config.output_generator,
        adkg_config,
        group_config,
        transcripts,
        rng,
        scheme,
    )
    .await
}

async fn adkg_pairing_swap_g1_to_g2_rescue<E, S>(
    adkg_sk: &str,
    g2: &str,
    adkg_config: AdkgConfig,
    group_config: &GroupConfig,
    transcripts: Vec<EncryptedAdkgTranscript>,
    rng: &mut impl AdkgRng,
    scheme: S,
) -> anyhow::Result<()>
where
    E: Pairing,
    E::ScalarField: FqSerialize + FqDeserialize,
    E::G1: HashToCurve + PointSerializeCompressed + PointDeserializeCompressed,
    E::G2: PointSerializeCompressed + PointDeserializeCompressed,
    S: DXKR23AdkgScheme<Curve = E::G1>,
    S::Curve: NamedCurveGroup,
    S::Hash: NamedDynDigest,
    S::ABAConfig: AbaConfig<'static, PartyId, Input = AbaCrainInput<S::Curve>>,
    <S::ACSSConfig as AcssConfig<'static, S::Curve, PartyId>>::Output:
        Into<ShareWithPoly<S::Curve>>,
{
    let adkg_sk = <S::Curve as Group>::ScalarField::deser_base64(adkg_sk)?;
    let adkg_pks = group_config
        .nodes
        .iter()
        .map(|p| S::Curve::deser_compressed_base64(&p.public_key_material.adkg_pk))
        .collect::<Result<Vec<_>, _>>()?;

    // Deserialize the transcripts
    let transcripts = transcripts.into_iter().map(|t| {
        serde_json::from_slice::<DXKR23Transcript>(&t).context("failed to deserialize transcripts")
    });

    // Decrypt the transcripts
    let messages = transcripts.map(|t| -> anyhow::Result<_> {
        let transcript = t?;
        let sender = transcript.id;
        let TranscriptData {
            broadcasts,
            directs,
        } = decrypt_transcript(adkg_config.id, &adkg_sk, adkg_pks.as_slice(), transcript)?;
        Ok((sender, broadcasts, directs))
    });

    // Split into broadcast and direct messages
    let (broadcasts, directs): (Vec<_>, Vec<_>) = messages
        .map_ok(|(sender, broadcasts, directs)| {
            (
                (
                    sender,
                    broadcasts
                        .0
                        .into_iter()
                        .map(|entry| entry.into_new_m::<Vec<u8>>())
                        .collect(),
                ),
                (
                    sender,
                    directs
                        .messages
                        .into_iter()
                        .map(|entry| entry.into_new_m::<Vec<u8>>())
                        .collect(),
                ),
            )
        })
        .collect::<Result<_, _>>()?;

    let transport_reader = InMemoryReaderTransport::from_entries(broadcasts, directs);
    let mut topic_dispatcher = TopicDispatcher::new();
    let topic_transport = topic_dispatcher.start(transport_reader);
    let transport = topic_transport
        .get_transport_for(TOPIC_SWAP_G1_TO_G2)
        .context("failed to obtain transport")?;

    // Start the ADKG and wait until we obtain a share, or the timeout occurs
    tracing::info!("Executing rescue ADKG");

    let mut adkg = scheme.new_adkg(
        adkg_config.id,
        group_config.n,
        group_config.t,
        group_config.t_reconstruction,
        adkg_sk,
        adkg_pks.clone(),
    )?;
    let adkg_out = adkg
        .start(rng, topic_transport.into())
        .await
        .context("Failed to execute ADKG")?;
    tracing::info!("Successfully obtained ADKG output");

    let adkg_pub_out = AdkgPubOutput {
        node_pks: adkg_out.node_pks.clone(),
        group_pk: adkg_out.group_pk,
    };
    let mut adkg_dual_out = AdkgOutputDual::<E::G1, E::G2> {
        sk: adkg_out.sk,
        out_pub_source: adkg_pub_out.clone(),
        out_pub_dest: None,
    };

    // Save the initial adkg output
    if let Err(e) = write_adkg_keys(
        &adkg_dual_out,
        &adkg_config.priv_out,
        &adkg_config.pub_out,
        adkg_config.scheme_name.clone(),
        group_config,
    ) {
        tracing::error!(error = ?e, "Failed to save initial adkg output");
    }

    // Swap adkg output to g2
    let g1 = scheme.generator_g();
    let g2 = E::G2::deser_compressed_base64(g2)?;
    let adkg_out_g2 = pairing_swap_g1_to_g2::<E, _>(
        group_config.t_reconstruction.get(),
        adkg_out,
        &g1,
        &g2,
        transport,
    )
    .await
    .context("failed to replay swap g1 to g2 protocol")?;

    // We got an adkg output on g2, re-write the output files with the new keys
    adkg_dual_out.out_pub_dest = Some(AdkgPubOutput {
        group_pk: adkg_out_g2.group_pk,
        node_pks: adkg_out_g2.node_pks,
    });

    if let Err(e) = write_adkg_keys(
        &adkg_dual_out,
        &adkg_config.priv_out,
        &adkg_config.pub_out,
        adkg_config.scheme_name.clone(),
        group_config,
    ) {
        tracing::error!(error = ?e, "Failed to save final adkg output");
    }

    Ok(())
}

#[serde_with::serde_as]
#[derive(Serialize, Deserialize)]
struct ChaCha20BroadcastCiphertext {
    /// one unique ciphertext per participant to store a shared encryption key
    encrypted_key: MultiHybridCiphertext,

    /// chacha20+poly1305 nonce
    #[serde_as(as = "utils::Base64OrBytes")]
    nonce: Vec<u8>,

    /// a (large) message encrypted with the shared encryption key
    #[serde_as(as = "utils::Base64OrBytes")]
    ciphertext: Vec<u8>,
}

/// An encrypted adkg transcript that can be stored and sent to nodes.
/// Authenticity of the transcript is obtained by relying on hybrid encryption w/ static public keys.
#[serde_with::serde_as]
#[derive(Serialize, Deserialize)]
struct DXKR23Transcript {
    /// identifier of the party who created the transcript
    id: PartyId,

    /// nonce used to encrypt the broadcast messages
    #[serde_as(as = "utils::Base64OrBytes")]
    broadcasts_nonce: [u8; NONCE_LENGTH],

    /// encrypted key used to encrypt broadcasts
    broadcasts_key_ct: MultiHybridCiphertext,

    /// encrypted broadcasts
    #[serde_as(as = "utils::Base64OrBytes")]
    broadcasts_ct: Vec<u8>,

    /// encrypted direct messages
    directs_cts: Vec<HybridCiphertext>,
}

struct TranscriptData {
    /// broadcast message sent to all parties
    broadcasts: BroadcastMessages<Vec<u8>>,

    /// messages sent to a specific party
    directs: DirectMessages<Vec<u8>>,
}

async fn encrypt_transcripts<CG>(
    id: PartyId,
    group_config: &GroupConfig,
    writer: &InMemoryWriter,
    adkg_sk: &CG::ScalarField, // the secret sk such that g * sk == pks[id]
    adkg_pks: &[CG],
    rng: &mut (impl Rng + CryptoRng),
) -> anyhow::Result<EncryptedAdkgTranscript>
where
    CG: CurveGroup + PointSerializeCompressed,
{
    let mut transcript = writer.take().await;

    // Serialize the broadcast messages
    let broadcasts = serde_json::to_vec(&BroadcastMessages(
        transcript
            .remove(&InMemoryEntryType::Broadcast)
            .unwrap_or_default()
            .into_iter()
            .map(InMemoryEntry::into_new_m::<SerializedBytes>)
            .collect(),
    ))
    .context("failed to serialize broadcast messages")?;

    // Serialize direct messages per-party
    let directs = PartyId::iter_all(group_config.n.get()).map(|i| {
        let messages = transcript
            .remove(&InMemoryEntryType::Direct(i))
            .unwrap_or_default()
            .into_iter()
            .map(InMemoryEntry::into_new_m::<SerializedBytes>)
            .collect();

        serde_json::to_vec(&DirectMessages {
            recipient: i,
            messages,
        })
        .context("failed to serialize direct messages")
    });

    // Generate nonce and key for the broadcast messages
    let broadcasts_nonce: Nonce = ChaCha20Poly1305::generate_nonce(&mut *rng);
    let broadcasts_key = ChaCha20Poly1305::generate_key(&mut *rng);

    // Encrypt the broadcast once with the secret key
    let enc_broadcasts = ChaCha20Poly1305::new(&broadcasts_key)
        .encrypt(&broadcasts_nonce, broadcasts.as_slice())
        .map_err(|_| anyhow!("failed to encrypt broadcasts"))?;

    // Encrypt the secret key n times, i.e., one ciphertext per party
    let enc_broadcast_key = ec_hybrid_chacha20poly1305::encrypt_multi_static(
        adkg_sk,
        adkg_pks[id],
        &vec![broadcasts_key.to_vec(); adkg_pks.len()],
        adkg_pks,
        &mut *rng,
    )
    .context("failed to encrypt broadcast secret key")?;

    // Encrypt the direct messages with a per-party key
    let enc_directs = directs
        .zip_eq(adkg_pks)
        .map(|(direct_msg, pki)| -> anyhow::Result<_> {
            Ok(ec_hybrid_chacha20poly1305::encrypt_with_sk(
                adkg_sk,
                &adkg_pks[id],
                direct_msg?.as_slice(),
                pki,
                rng,
            )?)
        })
        .collect::<Result<Vec<_>, _>>()
        .context("failed to encrypt direct messages")?;

    // Serialize the encrypted transcript w/ json for readability
    let transcript = serde_json::to_vec(&DXKR23Transcript {
        id,
        broadcasts_key_ct: enc_broadcast_key,
        broadcasts_nonce: broadcasts_nonce.into(),
        broadcasts_ct: enc_broadcasts,
        directs_cts: enc_directs,
    })
    .context("failed to serialize encrypted transcript")?;
    Ok(transcript)
}

fn decrypt_transcript<CG>(
    receiver_id: PartyId,
    adkg_sk: &CG::ScalarField, // the secret sk such that g * sk == pks[id]
    adkg_pks: &[CG],
    transcript_ct: DXKR23Transcript,
) -> anyhow::Result<TranscriptData>
where
    CG: CurveGroup + PointSerializeCompressed,
{
    let sender_id = transcript_ct.id;
    let sender_pk = adkg_pks[sender_id];

    // Decrypt the broadcasts_key
    let broadcasts_key = transcript_ct
        .broadcasts_key_ct
        .decrypt_one(
            receiver_id.as_index(),
            adkg_sk,
            &adkg_pks[receiver_id],
            &sender_pk,
        )
        .context("failed to decrypt broadcasts ciphertext")?;

    // Decrypt broadcast messages
    let broadcasts_key: Key = <[u8; 32]>::try_from(broadcasts_key)
        .map_err(|_| anyhow!("invalid broadcasts key"))?
        .into();
    let broadcasts_nonce = transcript_ct.broadcasts_nonce.into();
    let broadcasts = ChaCha20Poly1305::new(&broadcasts_key)
        .decrypt(&broadcasts_nonce, transcript_ct.broadcasts_ct.as_slice())
        .map_err(|_| anyhow!("failed to decrypt broadcasts"))?;
    let broadcasts: BroadcastMessages =
        serde_json::from_slice(&broadcasts).context("failed to deserialize broadcast messages")?;

    // Decrypt direct messages
    let directs = transcript_ct.directs_cts[receiver_id]
        .decrypt(adkg_sk, &adkg_pks[receiver_id], &sender_pk)
        .context("failed to decrypt direct messages")?;
    let directs: DirectMessages =
        serde_json::from_slice(&directs).context("failed to deserialize direct messages")?;

    Ok(TranscriptData {
        broadcasts: broadcasts.into(),
        directs: directs.into(),
    })
}
