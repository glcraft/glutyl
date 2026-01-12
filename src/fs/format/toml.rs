use super::{Error, Format};
use serde::{Deserialize, Serialize};

pub struct TomlFormat;

impl Format for TomlFormat {
    fn extension() -> Option<&'static str> {
        Some("toml")
    }

    fn from_str<'a, Data: Deserialize<'a>>(s: &'a str) -> Result<Data, Error> {
        Ok(toml::from_str(&s)?)
    }
    fn to_string<Data: Serialize>(data: &Data) -> Result<String, Error> {
        Ok(toml::to_string_pretty(&data)?)
    }
}
