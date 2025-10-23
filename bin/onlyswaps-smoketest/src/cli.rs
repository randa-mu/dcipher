use clap::Parser;

#[derive(Parser, Debug)]
pub(crate) struct CliConfig {
    #[arg(
        short = 'c',
        long = "config",
        env = "ONLYSWAPS_SMOKETEST_CONFIG",
        default_value = "~/.config/onlyswaps/smoketest/config.toml"
    )]
    pub config_path: String,
}
