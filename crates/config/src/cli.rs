use serde::de::DeserializeOwned;
use serde::{Deserialize, Deserializer};
use shellexpand::tilde;
use std::fs;
use std::str::FromStr;

// turns a filepath into its deserialised contents.
// Can be used with clap or parsed directly
#[derive(Debug, Clone)]
pub struct FileArg<T>(pub T);

impl<'de, T> Deserialize<'de> for FileArg<T>
where
    T: DeserializeOwned,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        FileArg::from_str(&s).map_err(serde::de::Error::custom)
    }
}

impl<T> FromStr for FileArg<T>
where
    T: DeserializeOwned,
{
    type Err = anyhow::Error;

    fn from_str(str: &str) -> Result<Self, Self::Err> {
        let path = tilde(str);
        let contents = fs::read(path.as_ref())?;
        let arg: T = toml::from_slice(&contents)?;
        Ok(FileArg(arg))
    }
}
