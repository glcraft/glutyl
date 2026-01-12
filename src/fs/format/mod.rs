mod json;
mod json5;
mod toml;

pub use json::JsonFormat;
pub use json5::Json5Format;
pub use toml::TomlFormat;

use std::path::Path;

use serde::{Deserialize, Serialize, de::DeserializeOwned};

pub trait Format {
    fn extension() -> Option<&'static str> {
        None
    }
    fn write_file<Data: Serialize, P: AsRef<Path>>(path: P, data: Data) -> Result<(), Error> {
        use std::io::Write;
        let mut file = std::fs::File::create(path)?;

        let serialized = Self::to_string(&data)?;
        file.write_all(serialized.as_bytes())?;
        Ok(())
    }
    fn read_file<Data: DeserializeOwned, P: AsRef<Path>>(path: P) -> Result<Data, Error> {
        use std::io::Read;
        let mut file = std::fs::File::open(path)?;

        let mut content = String::new();
        file.read_to_string(&mut content)?;
        Ok(Self::from_str(&content)?)
    }

    fn to_string<Data: Serialize>(data: &Data) -> Result<String, Error>;
    fn from_str<'a, Data: Deserialize<'a>>(s: &'a str) -> Result<Data, Error>;
}
#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
    #[error("toml parsing error: {0}")]
    TomlDeserialize(#[from] ::toml::de::Error),
    #[error("toml serialize error: {0}")]
    TomlSerialize(#[from] ::toml::ser::Error),
    #[error("json serialize/deserialize error: {0}")]
    Json(#[from] serde_json::Error),
    #[error("json5 serialize/deserialize error: {0}")]
    Json5(#[from] ::json5::Error),
}
