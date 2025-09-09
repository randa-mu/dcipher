use clap::Parser;

#[derive(Parser, Debug)]
pub(crate) struct CliConfig {
    #[arg(
        short = 'c',
        long = "config",
        env = "ONLYSWAPS_VERIFIER_CONFIG",
        default_value = "~/.verifier/config.json"
    )]
    pub config_path: String,
}
