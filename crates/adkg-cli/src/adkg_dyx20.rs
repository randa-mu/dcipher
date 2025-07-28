use crate::GroupConfig;
use adkg::adkg::AdkgOutput;
use adkg::helpers::PartyId;
use adkg::rand::AdkgRng;
use adkg::scheme::bn254::DYX20Bn254G1Keccak256;
use adkg::scheme::{AdkgScheme, AdkgSchemeConfig};
use anyhow::{Context, anyhow};
use ark_ec::Group;
use dcipher_network::topic::TopicBasedTransport;
use std::sync::Arc;
use std::time::Duration;
use utils::serialize::fq::FqDeserialize;
use utils::serialize::point::PointDeserializeCompressed;

#[allow(clippy::too_many_arguments)]
pub async fn adkg_dyx20_bn254_g1_keccak256<TBT>(
    id: PartyId,
    adkg_sk: &str,
    group_config: &GroupConfig,
    scheme_config: AdkgSchemeConfig,
    adkg_grace_period: Duration,
    adkg_timeout: Duration,
    topic_transport: Arc<TBT>,
    rng: &mut impl AdkgRng,
) -> anyhow::Result<AdkgOutput<<DYX20Bn254G1Keccak256 as AdkgScheme>::Curve>>
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
        .collect::<Result<_, _>>()?;

    let mut adkg = scheme.new_adkg(id, group_config.n, group_config.t, sk, pks)?;

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
            match &output {
                Ok(_) => {
                    tracing::info!("ADKG has terminated with an Ok output");
                    tracing::info!("Running ADKG until grace period of {}", humantime::format_duration(adkg_grace_period));
                    tokio::time::sleep(adkg_grace_period).await;
                }
                Err(e) => {
                    tracing::error!("failed to obtain output from ADKG: {e:?}");
                }
            }

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
