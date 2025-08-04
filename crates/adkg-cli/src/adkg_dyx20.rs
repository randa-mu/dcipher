use crate::transcripts::{BroadcastMessages, DirectMessages, EncryptedAdkgTranscript};
use crate::{GroupConfig, InMemoryWriter};
use adkg::adkg::AdkgOutput;
use adkg::helpers::PartyId;
use adkg::pke::ec_hybrid_chacha20poly1305;
use adkg::pke::ec_hybrid_chacha20poly1305::{HybridCiphertext, MultiHybridCiphertext};
use adkg::rand::AdkgRng;
use adkg::scheme::bn254::DYX20Bn254G1Keccak256;
use adkg::scheme::{AdkgScheme, AdkgSchemeConfig};
use anyhow::{Context, anyhow};
use ark_ec::{CurveGroup, Group};
use chacha20poly1305::aead::Aead;
use chacha20poly1305::{AeadCore, ChaCha20Poly1305, KeyInit, Nonce};
use dcipher_network::topic::TopicBasedTransport;
use dcipher_network::transports::writer::{InMemoryEntry, InMemoryEntryType};
use itertools::Itertools;
use rand::{CryptoRng, Rng, thread_rng};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::time::Duration;
use utils::serialize::fq::FqDeserialize;
use utils::serialize::point::{PointDeserializeCompressed, PointSerializeCompressed};

#[allow(clippy::too_many_arguments)]
pub async fn adkg_dyx20_bn254_g1_keccak256<TBT>(
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
    AdkgOutput<<DYX20Bn254G1Keccak256 as AdkgScheme>::Curve>,
    Option<EncryptedAdkgTranscript>,
)>
where
    TBT: TopicBasedTransport<Identity = PartyId>,
{
    let scheme = DYX20Bn254G1Keccak256::try_from(scheme_config)?;
    let sk = <<DYX20Bn254G1Keccak256 as AdkgScheme>::Curve as Group>::ScalarField::deser_base64(
        adkg_sk,
    )?;
    let pks = group_config
        .nodes
        .iter()
        .map(|p| {
            <DYX20Bn254G1Keccak256 as AdkgScheme>::Curve::deser_base64(
                &p.public_key_material.adkg_pk,
            )
        })
        .collect::<Result<Vec<_>, _>>()?;

    let mut adkg = scheme.new_adkg(id, group_config.n, group_config.t, sk, pks.clone())?;

    // Calculate time to sleep before actively executing the adkg
    let sleep_duration = (group_config.start_time - chrono::Utc::now())
        .to_std() // TimeDelta to positive duration
        .context("start_time cannot be in the past")?;

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
                        match encrypt_transcripts(group_config, &writer, &pks, scheme.generator_g(), &mut thread_rng()).await {
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

    Ok(res??) // unwrap both errors (timeout + adkg error)
}

#[serde_with::serde_as]
#[derive(Serialize, Deserialize)]
struct ChaCha20BroadcastCiphertext<
    CG: CurveGroup + PointSerializeCompressed + PointDeserializeCompressed,
> {
    /// one unique ciphertext per participant to store a shared encryption key
    encrypted_key: MultiHybridCiphertext<CG>,

    /// chacha20+poly1305 nonce
    #[serde_as(as = "serde_with::base64::Base64")]
    nonce: Vec<u8>,

    /// a (large) message encrypted with the shared encryption key
    #[serde_as(as = "serde_with::base64::Base64")]
    ciphertext: Vec<u8>,
}

/// An encrypted adkg transcript that can be stored and sent to nodes.
#[serde_with::serde_as]
#[derive(Serialize, Deserialize)]
#[serde(bound(
    serialize = "CG: PointSerializeCompressed",
    deserialize = "CG: PointDeserializeCompressed"
))]
pub struct DYX20EncryptedTranscript<CG: CurveGroup> {
    /// nonce used to encrypt the broadcast messages
    #[serde_as(as = "utils::Base64OrBytes")]
    pub broadcasts_nonce: Vec<u8>,

    /// encrypted key used to encrypt broadcasts
    pub broadcasts_key: MultiHybridCiphertext<CG>,

    /// encrypted broadcasts
    #[serde_as(as = "utils::Base64OrBytes")]
    pub broadcasts: Vec<u8>,

    /// encrypted direct messages
    pub directs: Vec<HybridCiphertext<CG>>,
}

async fn encrypt_transcripts(
    group_config: &GroupConfig,
    writer: &InMemoryWriter,
    pks: &[ark_bn254::G1Projective],
    g: ark_bn254::G1Projective,
    rng: &mut (impl Rng + CryptoRng),
) -> anyhow::Result<EncryptedAdkgTranscript> {
    let mut transcript = writer.take().await;

    // Serialize the broadcast messages
    let broadcasts = serde_json::to_vec(&BroadcastMessages(
        transcript
            .remove(&InMemoryEntryType::Broadcast)
            .unwrap_or_default()
            .into_iter()
            .map(InMemoryEntry::into_new_m)
            .collect(),
    ))
    .context("failed to serialize broadcast messages")?;

    // Serialize direct messages per-party
    let directs = PartyId::iter_all(group_config.n.get()).map(|i| {
        let messages = transcript
            .remove(&InMemoryEntryType::Direct(i))
            .unwrap_or_default()
            .into_iter()
            .map(InMemoryEntry::into_new_m)
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
    let enc_broadcast_key = ec_hybrid_chacha20poly1305::encrypt_multi(
        &vec![broadcasts_key.to_vec(); group_config.n.get()],
        pks,
        &g,
        &mut *rng,
    )
    .context("failed to encrypt broadcast secret key")?;

    // Encrypt the direct messages with a per-party key
    let enc_directs = directs
        .zip_eq(pks)
        .map(|(direct_msg, pki)| -> anyhow::Result<_> {
            Ok(ec_hybrid_chacha20poly1305::encrypt(
                direct_msg?.as_slice(),
                pki,
                &g,
                rng,
            )?)
        })
        .collect::<Result<Vec<_>, _>>()
        .context("failed to encrypt direct messages")?;

    // Serialize the encrypted transcript w/ json for readability
    let transcript = serde_json::to_vec(&DYX20EncryptedTranscript {
        broadcasts_key: enc_broadcast_key,
        broadcasts_nonce: broadcasts_nonce.to_vec(),
        broadcasts: enc_broadcasts,
        directs: enc_directs,
    })
    .context("failed to serialize encrypted transcript")?;
    Ok(transcript)
}
