use crate::keygen::PrivateKeyMaterial;
use crate::{AdkgPublic, AdkgSecret, GroupConfig};
use alloy::hex;
use alloy::primitives::FixedBytes;
use anyhow::Context;
use ark_bn254::G2Affine;
use base64::Engine;
use base64::prelude::BASE64_STANDARD;
use clap::Parser;
use config::agent::AgentConfig;
use config::app::{AppConfig, Libp2pConfig};
use config::keys::{Bn254SecretKey, Libp2pKeyWrapper};
use config::network::NetworkConfig;
use config::signing::{CommitteeConfig, MemberConfig};
use libp2p::Multiaddr;
use std::fs;
use std::net::Ipv4Addr;
use std::num::NonZeroU16;
use std::path::PathBuf;
use std::str::FromStr;
use std::time::Duration;
use url::Url;
use utils::serialize::point::PointDeserializeCompressed;

#[derive(Parser, Debug)]
pub struct GenerateOnlyswapsConfig {
    #[arg(
        long = "private",
        help = "your longterm private key file from the generate-keys step"
    )]
    pub operator_private: PathBuf,

    #[arg(long = "group", help = "the group file used to run the DKG")]
    pub group: PathBuf,

    #[arg(
        long = "public-share",
        help = "the public keyshare file generated during the ADKG"
    )]
    pub adkg_public: PathBuf,

    #[arg(
        long = "private-share",
        help = "the private keyshare file generated during the ADKG"
    )]
    pub adkg_private: PathBuf,

    #[arg(
        long = "multiaddr",
        help = "the multiaddr your node shoudl bind locally to receive packets from peers. It may differ from the one you shared with them."
    )]
    pub multiaddr: Multiaddr,

    #[arg(
        long = "member-id",
        help = "the index of your node in the final committee"
    )]
    pub member_id: NonZeroU16,

    #[arg(long, help = "the address of the router contract on each chain")]
    pub router_address: Option<FixedBytes<20>>,
}

pub(crate) fn generate_onlyswaps_config(args: GenerateOnlyswapsConfig) -> anyhow::Result<()> {
    let secret_key: PrivateKeyMaterial =
        toml::from_str(&fs::read_to_string(&args.operator_private)?)
            .context("failed to read operator private key")?;
    let group: GroupConfig = toml::from_str(&fs::read_to_string(&args.group)?)
        .context("failed to read group configuration")?;
    let shared_pub: AdkgPublic = toml::from_str(&fs::read_to_string(&args.adkg_public)?)
        .context("failed to read adkg public key")?;
    let shared_priv: AdkgSecret = toml::from_str(&fs::read_to_string(&args.adkg_private)?)
        .context("failed to read adkg private key")?;

    let app_config = build_app_config(
        secret_key,
        group,
        shared_pub,
        shared_priv,
        args.router_address,
        args.multiaddr,
        args.member_id,
    )?;
    println!("{}", toml::to_string(&app_config)?);
    Ok(())
}

fn build_app_config(
    secret_key: PrivateKeyMaterial,
    group: GroupConfig,
    shared_pub: AdkgPublic,
    shared_priv: AdkgSecret,
    router_address: Option<FixedBytes<20>>,
    multiaddr: Multiaddr,
    member_id: NonZeroU16,
) -> anyhow::Result<AppConfig> {
    // reformat a few of the relevant fields
    let contract_addr =
        router_address.unwrap_or("0x3dD1a497846d060Dce130B67b22E1F9DeE18c051".parse()?);
    let empty_private_key: FixedBytes<32> =
        "0x0000000000000000000000000000000000000000000000000000000000000001".parse()?;
    let shared_secret_key_hex = format!(
        "0x{}",
        hex::encode(
            BASE64_STANDARD
                .decode(shared_priv.sk.as_str())
                .context("failed to decode shared private key - is it valid base64?")?
        )
    );

    let mut members = Vec::with_capacity(shared_pub.node_pks.len());
    for n in shared_pub.node_pks {
        let config = MemberConfig {
            member_id: NonZeroU16::new(n.id.get() as u16)
                .expect("non-zero usize should always be a valid non-zero u16"),
            bls_pk: G2Affine::deser_compressed_base64(&n.pk)
                .context("a node's BLS public key was not a valid compressed base64 point")?,
            address: n.multiaddr.clone(),
            peer_id: n.peer_id,
        };
        members.push(config);
    }

    Ok(AppConfig {
        agent: AgentConfig {
            healthcheck_listen_addr: Ipv4Addr::new(0, 0, 0, 0),
            healthcheck_port: 8081,
            log_level: "debug".to_string(),
            log_json: true,
        },
        networks: vec![
            NetworkConfig {
                chain_id: 43113,
                rpc_url: Url::parse("wss://avalanche-fuji-c-chain-rpc.publicnode.com")?,
                router_address: contract_addr,
                private_key: empty_private_key,
                should_write: false,
                request_timeout: Duration::from_secs(5),
            },
            NetworkConfig {
                chain_id: 84532,
                rpc_url: Url::parse("wss://base-sepolia-rpc.publicnode.com")?,
                router_address: contract_addr,
                private_key: empty_private_key,
                should_write: false,
                request_timeout: Duration::from_secs(5),
            },
        ],
        libp2p: Libp2pConfig {
            secret_key: Libp2pKeyWrapper(secret_key.libp2p_sk),
            multiaddr,
        },
        committee: CommitteeConfig {
            member_id,
            secret_key: Bn254SecretKey::from_str(shared_secret_key_hex.as_str())?,
            t: group.t.try_into()?,
            n: group.n.try_into()?,
            members,
        },
    })
}

#[cfg(test)]
mod tests {
    use crate::onlyswaps::{GenerateOnlyswapsConfig, generate_onlyswaps_config};
    use alloy::primitives::{Address, U160};
    use libp2p::Multiaddr;
    use std::io::Write;
    use std::num::NonZeroU16;
    use std::str::FromStr;
    use tempfile::NamedTempFile;

    #[test]
    fn valid_keys_and_groups_outputs_valid_toml() -> anyhow::Result<()> {
        let mut priv_file = NamedTempFile::new()?;
        let mut group_file = NamedTempFile::new()?;
        let mut adkg_public = NamedTempFile::new()?;
        let mut adkg_private = NamedTempFile::new()?;
        let multiaddr = Multiaddr::from_str("/ip4/127.0.0.1/tcp/5150")?;
        let member_id = 1;
        let router_address = Address::from(U160::from(16));

        let _ = priv_file.write(PRIV_KEY_FILE_CONTENT.as_bytes())?;
        let _ = group_file.write(GROUP_FILE_CONTENT.as_bytes())?;
        let _ = adkg_public.write(ADKG_PUBLIC_CONTENT.as_bytes())?;
        let _ = adkg_private.write(ADKG_PRIVATE_CONTENT.as_bytes())?;

        generate_onlyswaps_config(GenerateOnlyswapsConfig {
            operator_private: priv_file.path().to_path_buf(),
            group: group_file.path().to_path_buf(),
            adkg_public: adkg_public.path().to_path_buf(),
            adkg_private: adkg_private.path().to_path_buf(),
            multiaddr,
            router_address: Some(router_address.0),
            member_id: NonZeroU16::new(member_id).unwrap(),
        })?;

        Ok(())
    }

    #[test]
    fn empty_contract_address_uses_default() -> anyhow::Result<()> {
        let mut priv_file = NamedTempFile::new()?;
        let mut group_file = NamedTempFile::new()?;
        let mut adkg_public = NamedTempFile::new()?;
        let mut adkg_private = NamedTempFile::new()?;
        let multiaddr = Multiaddr::from_str("/ip4/127.0.0.1/tcp/5150")?;
        let member_id = 1;

        let _ = priv_file.write(PRIV_KEY_FILE_CONTENT.as_bytes())?;
        let _ = group_file.write(GROUP_FILE_CONTENT.as_bytes())?;
        let _ = adkg_public.write(ADKG_PUBLIC_CONTENT.as_bytes())?;
        let _ = adkg_private.write(ADKG_PRIVATE_CONTENT.as_bytes())?;

        generate_onlyswaps_config(GenerateOnlyswapsConfig {
            operator_private: priv_file.path().to_path_buf(),
            group: group_file.path().to_path_buf(),
            adkg_public: adkg_public.path().to_path_buf(),
            adkg_private: adkg_private.path().to_path_buf(),
            multiaddr,
            router_address: None,
            member_id: NonZeroU16::new(member_id).unwrap(),
        })?;

        Ok(())
    }

    const PRIV_KEY_FILE_CONTENT: &str = r#"
adkg_sk = "AGzRJmmnPZ+VnsNVLbZyjuk369Y+zOLxXrm4B/kcKyw="
libp2p_sk = "CAESQGRWsEjWcr1qGPz/X3rPomvRPSYXoOEKCBGC19aF3DyAfHkRgFT/HVqC840jXoEM/C34EkpM6VOpvcZvMZI1SxE="
        "#;
    const GROUP_FILE_CONTENT: &str = r#"
n = 4
t = 1
t_reconstruction = 2
start_time = "2025-09-10T10:19:30Z"

[[nodes]]
id = 1
multiaddr = "/ip4/127.0.0.1/tcp/8001"
adkg_pk = "2lteV12hFdTFkD+uvcf6PC4T/OtZEkIcT0IZvmGphfk="
peer_id = "12D3KooWJCFoAiTaoiiqsemhYpFaS9yLb5FW2Jo4c4TViurpVFRz"

[[nodes]]
id = 2
multiaddr = "/ip4/127.0.0.1/tcp/8002"
adkg_pk = "4IMMoQ0SW91XjBKZsP7c1IPZvc1Tw7oRFPYGhaxvq4w="
peer_id = "12D3KooWJenvgPUcXwZGs311iQH7sNhskZm6EWssdcQGY5RpBZaD"

[[nodes]]
id = 3
multiaddr = "/ip4/127.0.0.1/tcp/8003"
adkg_pk = "3UhDVuhn6zql3nOtmUd6F6EVdfHefu54dQkToxwrdM0="
peer_id = "12D3KooWKKEDrbCXr8uTWUj8sQ3KzafR3A5TkkDBp91ieF6YkkDQ"

[[nodes]]
id = 4
multiaddr = "/ip4/127.0.0.1/tcp/8004"
adkg_pk = "jmX7ICkcxFvNNfGmx85pigHe4U6qoG+vt+vd9MyvZbE="
peer_id = "12D3KooWRJPmAZLrUdM7nUsmdisDNgDLWPedMMCEtD2pQosLU31h"
        "#;
    const ADKG_PUBLIC_CONTENT: &str = r#"
adkg_scheme_name = "DXKR23-Bn254G1-Keccak256"
genesis_timestamp = 1757499570
group_pk = "ok1mwyByNNb82/pizDn+KGZxoPPIMpFtYEE02cTauWAn45XisiVRSpqO0gKCpgnSqcUAq1RkDS0tc02SbKGOzw=="

[[node_pks]]
id = 1
pk = "yr9ORCL6qqe1XiJPJ+SkPy5lYo6dhwECo3dK7i0Db0Qd0IM9As1ePHAx88uXgjAT4+xDrKrsW9rjRsErPygiVQ=="
peer_id = "12D3KooWJCFoAiTaoiiqsemhYpFaS9yLb5FW2Jo4c4TViurpVFRz"
multiaddr = "/ip4/127.0.0.1/tcp/8001"

[[node_pks]]
id = 2
pk = "gJP9lJVz72EycZTVIbNC7fvPulyfwu5XmOxZhtS/r8slXYwMB0O4lUkiT78VTdiAdGA3man8j4hQDI/nRTbvQw=="
peer_id = "12D3KooWJenvgPUcXwZGs311iQH7sNhskZm6EWssdcQGY5RpBZaD"
multiaddr = "/ip4/127.0.0.1/tcp/8002"

[[node_pks]]
id = 3
pk = "oj3mtJVsf/3Kc/l3G0gB/tQggvFa8o9KnYlsCRSG+JkVi/pxo73hsLtSQ+szT9LH7TBjgfYc5+NuuSDdm6OsUA=="
peer_id = "12D3KooWKKEDrbCXr8uTWUj8sQ3KzafR3A5TkkDBp91ieF6YkkDQ"
multiaddr = "/ip4/127.0.0.1/tcp/8003"

[[node_pks]]
id = 4
pk = "zSFEDijpgjEJvw8E/8XlkXKAbKTo1aMCp6Zg6pYGpFEZvPbLUr9qzJB0+uRiClI+qSvk0Q2jNX3Gs3nG55GQSg=="
peer_id = "12D3KooWRJPmAZLrUdM7nUsmdisDNgDLWPedMMCEtD2pQosLU31h"
multiaddr = "/ip4/127.0.0.1/tcp/8004"
        "#;
    const ADKG_PRIVATE_CONTENT: &str = r#"
adkg_scheme_name = "DXKR23-Bn254G1-Keccak256"
genesis_timestamp = 1757499570
sk = "Bjowi2bicZE2bpN1OOIvpbJ582giKchaaMF7TrM2Vvc="
        "#;
}
