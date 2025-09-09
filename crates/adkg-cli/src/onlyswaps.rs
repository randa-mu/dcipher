use crate::keygen::PrivateKeyMaterial;
use crate::{AdkgPublic, AdkgSecret, GroupConfig};
use alloy::hex;
use alloy::primitives::FixedBytes;
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
        help = "the private key file from the generate-keys step"
    )]
    pub operator_private: PathBuf,

    #[arg(long = "group", help = "the group file used to run the DKG")]
    pub group: PathBuf,

    #[arg(long = "adkg-public", help = "the ADKG public output file")]
    pub adkg_public: PathBuf,

    #[arg(long = "adkg-private", help = "the ADKG private output file")]
    pub adkg_private: PathBuf,

    #[arg(
        long = "multiaddr",
        help = "the multiaddr your node is running at to peer with other nodes"
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

pub(crate) async fn generate_onlyswaps_config(args: GenerateOnlyswapsConfig) -> anyhow::Result<()> {
    // load from all the relevant files
    let secret_key: PrivateKeyMaterial =
        toml::from_str(&fs::read_to_string(&args.operator_private)?)?;
    let group: GroupConfig = toml::from_str(&fs::read_to_string(&args.group)?)?;
    let shared_pub: AdkgPublic = toml::from_str(&fs::read_to_string(&args.adkg_public)?)?;
    let shared_priv: AdkgSecret = toml::from_str(&fs::read_to_string(&args.adkg_private)?)?;

    // reformat a few of the relevant fields
    let contract_addr = args
        .router_address
        .unwrap_or("0x3dD1a497846d060Dce130B67b22E1F9DeE18c051".parse()?);
    let empty_private_key: FixedBytes<32> =
        "0x0000000000000000000000000000000000000000000000000000000000000001".parse()?;
    let shared_secret_key_hex = format!(
        "0x{}",
        hex::encode(BASE64_STANDARD.decode(shared_priv.sk.as_str())?)
    );

    let mut members = Vec::with_capacity(shared_pub.node_pks.len());
    for n in shared_pub.node_pks {
        let config = MemberConfig {
            member_id: NonZeroU16::new(n.id.get() as u16)
                .expect("non-zero usize should always be a valid non-zero u16"),
            bls_pk: G2Affine::deser_compressed_base64(&n.pk)?,
            address: n.multiaddr.clone(),
            peer_id: n.peer_id,
        };
        members.push(config);
    }

    let app_config = AppConfig {
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
            multiaddr: args.multiaddr,
        },
        committee: CommitteeConfig {
            member_id: args.member_id,
            secret_key: Bn254SecretKey::from_str(shared_secret_key_hex.as_str())?,
            t: group.t.try_into()?,
            n: group.n.try_into()?,
            members,
        },
    };

    println!("{}", toml::to_string(&app_config)?);
    Ok(())
}
