use figment::Figment;
use figment::providers::{Format, Json, Toml};
use serde::Deserialize;
use shellexpand::tilde;
use std::path::Path;

pub fn load_mapped_config_file<'de, ConfigFile, AppConfig>(
    config_path: String,
) -> anyhow::Result<AppConfig>
where
    ConfigFile: Deserialize<'de>,
    AppConfig: TryFrom<ConfigFile, Error = anyhow::Error>,
{
    let config_file: ConfigFile = load_config_file(config_path)?;
    config_file.try_into()
}

pub fn load_config_file<'de, ConfigFile: Deserialize<'de>>(
    config_path: String,
) -> anyhow::Result<ConfigFile> {
    println!("Loading app config from {}", config_path);
    let path_str = tilde(&config_path);
    let path = Path::new(path_str.as_ref());
    let config_file = match path.extension().and_then(|s| s.to_str()) {
        Some("toml") => Figment::new().merge(Toml::file(path)),
        Some("json") => Figment::new().merge(Json::file(path)),
        _ => anyhow::bail!("unsupported config format"),
    };

    Ok(config_file.extract()?)
}

#[cfg(test)]
mod tests {
    use crate::file::load_mapped_config_file;
    use serde::{Deserialize, Serialize};
    use speculoos::assert_that;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[derive(Serialize, Deserialize, Debug, Clone)]
    struct TestFile {
        stuff: String,
    }

    struct TestConfig {
        stuff: Vec<u8>,
    }
    impl TryFrom<TestFile> for TestConfig {
        type Error = anyhow::Error;

        fn try_from(value: TestFile) -> Result<Self, Self::Error> {
            let stuff = hex::decode(value.stuff)?;
            Ok(Self { stuff })
        }
    }

    #[test]
    fn test_load_valid_toml_config() -> anyhow::Result<()> {
        let mut tmp = NamedTempFile::with_suffix(".toml")?;
        let toml_cfg = r#"
        stuff = "deadbeef"
        "#;

        writeln!(tmp, "{}", toml_cfg)?;

        let config_path = tmp.path().to_str().unwrap().to_string();
        let config = load_mapped_config_file::<TestFile, TestConfig>(config_path)?;
        assert_that!(config.stuff).is_equal_to(Vec::from([0xde, 0xad, 0xbe, 0xef]));

        Ok(())
    }

    #[test]
    fn test_load_valid_json_config() -> anyhow::Result<()> {
        let mut tmp = NamedTempFile::with_suffix(".json")?;
        let json_cfg = r#"
        {
            "stuff": "deadbeef"
        }
        "#;

        writeln!(tmp, "{}", json_cfg)?;

        let config_path = tmp.path().to_str().unwrap().to_string();
        let config = load_mapped_config_file::<TestFile, TestConfig>(config_path)?;
        assert_that!(config.stuff).is_equal_to(Vec::from([0xde, 0xad, 0xbe, 0xef]));

        Ok(())
    }

    #[test]
    fn test_load_invalid_toml_config() -> anyhow::Result<()> {
        let mut tmp = NamedTempFile::with_suffix(".toml")?;
        let toml_cfg = r#"
        stuff = "nothex"
        "#;

        writeln!(tmp, "{}", toml_cfg)?;

        let config_path = tmp.path().to_str().unwrap().to_string();
        let result = load_mapped_config_file::<TestFile, TestConfig>(config_path);

        assert!(result.is_err());
        Ok(())
    }
}
