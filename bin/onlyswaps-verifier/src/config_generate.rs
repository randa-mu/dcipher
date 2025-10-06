use alloy::hex;
use alloy::primitives::FixedBytes;
use alloy::transports::http::reqwest::Url;
use anyhow::{Context, anyhow};
use ark_bn254::G2Affine;
use base64::Engine;
use base64::prelude::BASE64_STANDARD;
use config::adkg::{AdkgPublic, AdkgSecret, GroupConfig, PrivateKeyMaterial};
use config::agent::AgentConfig;
use config::keys::Bn254SecretKey;
use config::network::{Libp2pConfig, NetworkConfig};
use config::signing::{CommitteeConfig, MemberConfig};
use std::fs;
use std::net::Ipv4Addr;
use std::num::{NonZero, NonZeroU16};
use std::str::FromStr;
use utils::serialize::point::PointDeserializeCompressed;

use crate::cli::{Environment, GenerateConfigArgs};
use crate::config::{AppConfig, TimeoutConfig};

pub(crate) fn generate_onlyswaps_config(args: GenerateConfigArgs) -> anyhow::Result<()> {
    let app_config = generate_app_config(args)?;
    println!("{}", toml::to_string(&app_config)?);
    Ok(())
}
fn generate_app_config(args: GenerateConfigArgs) -> anyhow::Result<AppConfig> {
    let secret_key: PrivateKeyMaterial =
        toml::from_str(&fs::read_to_string(&args.operator_private)?)
            .context("failed to read operator private key")?;
    let group: GroupConfig = toml::from_str(&fs::read_to_string(&args.group)?)
        .context("failed to read group configuration")?;
    let shared_pub: AdkgPublic = toml::from_str(&fs::read_to_string(&args.adkg_public)?)
        .context("failed to read adkg public key")?;
    let shared_priv: AdkgSecret = toml::from_str(&fs::read_to_string(&args.adkg_private)?)
        .context("failed to read adkg private key")?;

    build_app_config(secret_key, group, shared_pub, shared_priv, args)
}

fn build_app_config(
    secret_key: PrivateKeyMaterial,
    group: GroupConfig,
    shared_pub: AdkgPublic,
    shared_priv: AdkgSecret,
    args: GenerateConfigArgs,
) -> anyhow::Result<AppConfig> {
    let GenerateConfigArgs {
        router_address,
        multiaddr,
        member_id,
        environment,
        ..
    } = args;

    let networks = if environment == Environment::Mainnet {
        create_mainnet_config(router_address)
    } else {
        create_testnet_config(router_address)
    }?;

    let shared_secret_key_hex = parse_shared_secret_key(shared_priv)?;
    let members = create_members_config(shared_pub)?;
    let threshold = calculate_threshold(&group)?;

    let agent = AgentConfig {
        healthcheck_listen_addr: Ipv4Addr::new(0, 0, 0, 0),
        healthcheck_port: 8081,
        log_level: "debug".to_string(),
        log_json: true,
    };
    let libp2p = Libp2pConfig {
        secret_key: secret_key.libp2p_sk,
        multiaddr,
    };
    let committee = CommitteeConfig {
        member_id,
        secret_key: Bn254SecretKey::from_str(shared_secret_key_hex.as_str())?,
        t: threshold.try_into()?,
        n: group.n.try_into()?,
        members,
    };

    Ok(AppConfig {
        agent,
        networks,
        libp2p,
        committee,
        timeout: TimeoutConfig::default(),
    })
}

fn calculate_threshold(group: &GroupConfig) -> anyhow::Result<NonZero<usize>> {
    group
        .t_reconstruction
        .checked_add(1)
        .ok_or(anyhow!("getting threshold overflowed"))
}

fn create_members_config(shared_pub: AdkgPublic) -> anyhow::Result<Vec<MemberConfig<G2Affine>>> {
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
    Ok(members)
}

fn parse_shared_secret_key(shared_priv: AdkgSecret) -> anyhow::Result<String> {
    let bytes = BASE64_STANDARD
        .decode(shared_priv.sk.as_str())
        .context("failed to decode shared private key - is it valid base64?")?;

    Ok(format!("0x{}", hex::encode(bytes)))
}

fn create_mainnet_config(
    router_address: Option<FixedBytes<20>>,
) -> anyhow::Result<Vec<NetworkConfig>> {
    let contract_addr =
        router_address.unwrap_or("0x4cB630aAEA9e152db83A846f4509d83053F21078".parse()?);
    Ok(vec![
        NetworkConfig {
            chain_id: 43114,
            rpc_url: Url::parse("wss://api.avax.network/ext/bc/C/ws")?,
            router_address: contract_addr,
            private_key: EMPTY_PRIVATE_KEY,
            should_write: false,
        },
        NetworkConfig {
            chain_id: 8453,
            rpc_url: Url::parse("wss://base-rpc.publicnode.com")?,
            router_address: contract_addr,
            private_key: EMPTY_PRIVATE_KEY,
            should_write: false,
        },
    ])
}

fn create_testnet_config(
    router_address: Option<FixedBytes<20>>,
) -> anyhow::Result<Vec<NetworkConfig>> {
    let contract_addr =
        router_address.unwrap_or("0x4cB630aAEA9e152db83A846f4509d83053F21078".parse()?);
    Ok(vec![
        NetworkConfig {
            chain_id: 43113,
            rpc_url: Url::parse("wss://avalanche-fuji-c-chain-rpc.publicnode.com")?,
            router_address: contract_addr,
            private_key: EMPTY_PRIVATE_KEY,
            should_write: false,
        },
        NetworkConfig {
            chain_id: 84532,
            rpc_url: Url::parse("wss://base-sepolia-rpc.publicnode.com")?,
            router_address: contract_addr,
            private_key: EMPTY_PRIVATE_KEY,
            should_write: false,
        },
    ])
}
const EMPTY_PRIVATE_KEY: FixedBytes<32> = FixedBytes([0u8; 32]);

#[cfg(test)]
mod tests {
    use crate::cli::{Environment, GenerateConfigArgs};
    use crate::config_generate::{generate_app_config, generate_onlyswaps_config};
    use alloy::primitives::{Address, U160};
    use libp2p::Multiaddr;
    use speculoos::assert_that;
    use speculoos::iter::MappingIterAssertions;
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

        generate_onlyswaps_config(GenerateConfigArgs {
            operator_private: priv_file.path().to_path_buf(),
            group: group_file.path().to_path_buf(),
            adkg_public: adkg_public.path().to_path_buf(),
            adkg_private: adkg_private.path().to_path_buf(),
            multiaddr,
            router_address: Some(router_address.0),
            member_id: NonZeroU16::new(member_id).unwrap(),
            environment: Environment::Testnet,
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

        generate_onlyswaps_config(GenerateConfigArgs {
            operator_private: priv_file.path().to_path_buf(),
            group: group_file.path().to_path_buf(),
            adkg_public: adkg_public.path().to_path_buf(),
            adkg_private: adkg_private.path().to_path_buf(),
            multiaddr,
            router_address: None,
            member_id: NonZeroU16::new(member_id).unwrap(),
            environment: Environment::Testnet,
        })?;

        Ok(())
    }

    #[test]
    fn passing_mainnet_uses_mainnet_chain_id() -> anyhow::Result<()> {
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

        let output = generate_app_config(GenerateConfigArgs {
            operator_private: priv_file.path().to_path_buf(),
            group: group_file.path().to_path_buf(),
            adkg_public: adkg_public.path().to_path_buf(),
            adkg_private: adkg_private.path().to_path_buf(),
            multiaddr,
            router_address: Some(router_address.0),
            member_id: NonZeroU16::new(member_id).unwrap(),
            environment: Environment::Mainnet,
        })?;

        assert_that!(output.networks).matching_contains(|v| v.chain_id == 8453);
        assert_that!(output.networks).matching_contains(|v| v.chain_id == 43114);

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
