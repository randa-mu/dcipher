use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "onlyswaps-verifier")]
#[command(about = "A CLI for managing and running onlyswaps verifier nodes in the dcipher network")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    #[command(about = "Start the onlyswaps verifier agent")]
    Start(StartArgs),
}

#[derive(Parser, Debug)]
pub struct StartArgs {
    #[arg(
        short = 'c',
        long = "config",
        env = "ONLYSWAPS_VERIFIER_CONFIG",
        default_value = "~/.config/onlyswaps/verifier/config.toml",
        help = "the path to your toml config file for managing the agent and network"
    )]
    pub config_path: String,
}
