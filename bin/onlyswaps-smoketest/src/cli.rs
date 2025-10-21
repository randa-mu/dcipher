use clap::Parser;

#[derive(Parser, Debug)]
pub(crate) struct CliConfig {
    #[arg(
        short = 'c',
        long = "config",
        env = "ONLYSWAPS_STATE_API_CONFIG",
        default_value = "~/.config/onlyswaps/state-api/config.toml"
    )]
    pub config_path: String,
}
