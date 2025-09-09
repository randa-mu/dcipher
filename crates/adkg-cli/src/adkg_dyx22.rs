//! Provides implementations to execute the Practical Asynchronous Distributed Key Generation (DYX+22)
//! described in <https://eprint.iacr.org/2021/1591.pdf>.

use crate::InMemoryWriter;
use crate::config::GroupConfig;
use crate::transcripts::{
    BroadcastMessages, DirectMessages, EncryptedAdkgTranscript, SerializedBytes,
};
use adkg::aba::AbaConfig;
use adkg::adkg::{AbaCrainInput, AdkgOutput, ShareWithPoly};
use adkg::helpers::PartyId;
use adkg::pke::ec_hybrid_chacha20poly1305;
use adkg::pke::ec_hybrid_chacha20poly1305::{
    HybridCiphertext, MultiHybridCiphertext, NONCE_LENGTH,
};
use adkg::rand::AdkgRng;
use adkg::scheme::bls12_381::DYX22Bls12_381G1Sha256;
use adkg::scheme::bn254::DYX22Bn254G1Keccak256;
use adkg::scheme::{AdkgScheme, AdkgSchemeConfig};
use adkg::vss::acss::AcssConfig;
use anyhow::{Context, anyhow};
use ark_ec::{AffineRepr, CurveGroup, Group};
use chacha20poly1305::aead::Aead;
use chacha20poly1305::{AeadCore, ChaCha20Poly1305, Key, KeyInit, Nonce};
use dcipher_network::topic::TopicBasedTransport;
use dcipher_network::topic::dispatcher::TopicDispatcher;
use dcipher_network::transports::replayable::reader::InMemoryReaderTransport;
use dcipher_network::transports::replayable::writer::{InMemoryEntry, InMemoryEntryType};
use itertools::Itertools;
use rand::{CryptoRng, Rng, thread_rng};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::time::Duration;
use utils::dst::{NamedCurveGroup, NamedDynDigest};
use utils::serialize::fq::FqDeserialize;
use utils::serialize::point::{PointDeserializeCompressed, PointSerializeCompressed};

#[allow(clippy::too_many_arguments)]
pub async fn adkg_dyx22_bn254_g1_keccak256<TBT>(
    id: PartyId,
    adkg_sk: &str,
    group_config: &GroupConfig,
    scheme_config: AdkgSchemeConfig,
    adkg_grace_period: Duration,
    adkg_timeout: Duration,
    topic_transport: Arc<TBT>,
    writer: Option<InMemoryWriter>,
    rng: &mut impl AdkgRng,
) -> anyhow::Result<(
    AdkgOutput<<DYX22Bn254G1Keccak256 as AdkgScheme>::Curve>,
    Option<EncryptedAdkgTranscript>,
)>
where
    TBT: TopicBasedTransport<Identity = PartyId>,
{
    let scheme = DYX22Bn254G1Keccak256::try_from(scheme_config)?;
    let sk = <<DYX22Bn254G1Keccak256 as AdkgScheme>::Curve as Group>::ScalarField::deser_base64(
        adkg_sk,
    )?;
    let pks = group_config
        .nodes
        .iter()
        .map(|p| {
            <DYX22Bn254G1Keccak256 as AdkgScheme>::Curve::deser_compressed_base64(
                &p.public_key_material.adkg_pk,
            )
        })
        .collect::<Result<Vec<_>, _>>()?;

    adkg_dyx22(
        id,
        sk,
        pks,
        group_config,
        adkg_grace_period,
        adkg_timeout,
        topic_transport,
        writer,
        scheme,
        rng,
    )
    .await
}

#[allow(clippy::too_many_arguments)]
pub async fn adkg_dyx22_bls12_381_g1_sha256<TBT>(
    id: PartyId,
    adkg_sk: &str,
    group_config: &GroupConfig,
    scheme_config: AdkgSchemeConfig,
    adkg_grace_period: Duration,
    adkg_timeout: Duration,
    topic_transport: Arc<TBT>,
    writer: Option<InMemoryWriter>,
    rng: &mut impl AdkgRng,
) -> anyhow::Result<(
    AdkgOutput<<DYX22Bls12_381G1Sha256 as AdkgScheme>::Curve>,
    Option<EncryptedAdkgTranscript>,
)>
where
    TBT: TopicBasedTransport<Identity = PartyId>,
{
    let scheme = DYX22Bls12_381G1Sha256::try_from(scheme_config)?;
    let sk = <<DYX22Bls12_381G1Sha256 as AdkgScheme>::Curve as Group>::ScalarField::deser_base64(
        adkg_sk,
    )?;
    let pks = group_config
        .nodes
        .iter()
        .map(|p| {
            <DYX22Bls12_381G1Sha256 as AdkgScheme>::Curve::deser_compressed_base64(
                &p.public_key_material.adkg_pk,
            )
        })
        .collect::<Result<Vec<_>, _>>()?;

    adkg_dyx22(
        id,
        sk,
        pks,
        group_config,
        adkg_grace_period,
        adkg_timeout,
        topic_transport,
        writer,
        scheme,
        rng,
    )
    .await
}

#[allow(clippy::too_many_arguments)]
async fn adkg_dyx22<S, TBT>(
    id: PartyId,
    sk: <<S::Curve as CurveGroup>::Affine as AffineRepr>::ScalarField,
    pks: Vec<S::Curve>,
    group_config: &GroupConfig,
    adkg_grace_period: Duration,
    adkg_timeout: Duration,
    topic_transport: Arc<TBT>,
    writer: Option<InMemoryWriter>,
    adkg_scheme: S,
    rng: &mut impl AdkgRng,
) -> anyhow::Result<(AdkgOutput<S::Curve>, Option<EncryptedAdkgTranscript>)>
where
    S: AdkgScheme,
    S::Curve: NamedCurveGroup,
    S::Hash: NamedDynDigest,
    S::ABAConfig: AbaConfig<'static, PartyId, Input = AbaCrainInput<S::Curve>>,
    <S::ACSSConfig as AcssConfig<'static, S::Curve, PartyId>>::Output:
        Into<ShareWithPoly<S::Curve>>,
    TBT: TopicBasedTransport<Identity = PartyId>,
{
    let mut adkg = adkg_scheme.new_adkg(id, group_config.n, group_config.t, sk, pks.clone())?;

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
        humantime::format_duration(adkg_timeout)
    );

    let res = tokio::select! {
        output = adkg.start(rng, topic_transport) => {
            let output = match output {
                Ok(adkg_out) => {
                    tracing::info!("ADKG has terminated with an Ok output");
                    tracing::info!("Running ADKG until grace period of {}", humantime::format_duration(adkg_grace_period));
                    tokio::time::sleep(adkg_grace_period).await;

                    let transcript = if let Some(writer) = writer {
                        match encrypt_transcripts(id, group_config, &writer, &sk, &pks, &mut thread_rng()).await {
                            Ok(transcript) => Some(transcript),
                            Err(e) => {
                                tracing::error!(error = ?e, "Failed to generate ADKG transcript");
                                None
                            }
                        }
                    } else {
                        None
                    };
                    Ok((adkg_out, transcript))
                }
                Err(e) => {
                    tracing::error!("failed to obtain output from ADKG: {e:?}");
                    Err(e)
                }
            };

            Ok(output)
        }

        _ = tokio::time::sleep(adkg_timeout) => {
            println!("Aborting ADKG due to timeout");
            Err(anyhow!("ADKG has timed out"))
        }
    };

    tracing::warn!("Stopping ADKG...");
    adkg.stop().await;

    Ok(res??)
}

pub async fn adkg_dyx22_bn254_g1_keccak256_rescue(
    id: PartyId,
    adkg_sk: &str,
    group_config: &GroupConfig,
    scheme_config: AdkgSchemeConfig,
    transcripts: Vec<EncryptedAdkgTranscript>,
    rng: &mut impl AdkgRng,
) -> anyhow::Result<AdkgOutput<<DYX22Bn254G1Keccak256 as AdkgScheme>::Curve>> {
    let scheme = DYX22Bn254G1Keccak256::try_from(scheme_config)?;
    adkg_rescue(id, adkg_sk, group_config, transcripts, rng, scheme).await
}

pub async fn adkg_dyx22_bls12_381_g1_sha256_rescue(
    id: PartyId,
    adkg_sk: &str,
    group_config: &GroupConfig,
    scheme_config: AdkgSchemeConfig,
    transcripts: Vec<EncryptedAdkgTranscript>,
    rng: &mut impl AdkgRng,
) -> anyhow::Result<AdkgOutput<<DYX22Bls12_381G1Sha256 as AdkgScheme>::Curve>> {
    let scheme = DYX22Bls12_381G1Sha256::try_from(scheme_config)?;
    adkg_rescue(id, adkg_sk, group_config, transcripts, rng, scheme).await
}

async fn adkg_rescue<S>(
    id: PartyId,
    adkg_sk: &str,
    group_config: &GroupConfig,
    transcripts: Vec<EncryptedAdkgTranscript>,
    rng: &mut impl AdkgRng,
    scheme: S,
) -> anyhow::Result<AdkgOutput<S::Curve>>
where
    S: AdkgScheme,
    S::Curve: NamedCurveGroup,
    <S::Curve as Group>::ScalarField: FqDeserialize,
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
        serde_json::from_slice::<DYX22Transcript>(&t).context("failed to deserialize transcripts")
    });

    // Decrypt the transcripts
    let messages = transcripts.map(|t| -> anyhow::Result<_> {
        let transcript = t?;
        let sender = transcript.id;
        let TranscriptData {
            broadcasts,
            directs,
        } = decrypt_transcript(id, &adkg_sk, adkg_pks.as_slice(), transcript)?;
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
    let topic_transport = topic_dispatcher.start(transport_reader).into();

    // Start the ADKG and wait until we obtain a share, or the timeout occurs
    tracing::info!("Executing rescue ADKG");

    let mut adkg = scheme.new_adkg(
        id,
        group_config.n,
        group_config.t,
        adkg_sk,
        adkg_pks.clone(),
    )?;
    let output = adkg
        .start(rng, topic_transport)
        .await
        .context("Failed to execute ADKG")?;

    tracing::info!("Successfully obtained ADKG output");
    Ok(output)
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
struct DYX22Transcript {
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
    let transcript = serde_json::to_vec(&DYX22Transcript {
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
    transcript_ct: DYX22Transcript,
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
