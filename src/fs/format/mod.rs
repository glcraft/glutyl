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

pub struct JsonFormat;

impl Format for JsonFormat {
    fn write_file<Data: Serialize, P: AsRef<Path>>(path: P, data: Data) -> Result<(), Error> {
        let file = std::fs::File::create(path)?;
        serde_json::to_writer(file, &data)?;
        Ok(())
    }

    fn extension() -> Option<&'static str> {
        Some("json")
    }

    fn read_file<Data: DeserializeOwned, P: AsRef<Path>>(path: P) -> Result<Data, Error> {
        let file = std::fs::File::open(path)?;
        Ok(serde_json::from_reader(file)?)
    }
    fn from_str<'a, Data: Deserialize<'a>>(s: &'a str) -> Result<Data, Error> {
        Ok(serde_json::from_str(s)?)
    }
    fn to_string<Data: Serialize>(data: &Data) -> Result<String, Error> {
        Ok(serde_json::to_string(data)?)
    }
}

pub struct Json5Format;

impl Format for Json5Format {
    fn write_file<Data: Serialize, P: AsRef<Path>>(path: P, data: Data) -> Result<(), Error> {
        let file = std::fs::File::create(path)?;
        json5::to_writer(file, &data)?;
        Ok(())
    }

    fn extension() -> Option<&'static str> {
        Some("json5")
    }

    fn from_str<'a, Data: Deserialize<'a>>(s: &'a str) -> Result<Data, Error> {
        Ok(json5::from_str(s)?)
    }
    fn to_string<Data: Serialize>(data: &Data) -> Result<String, Error> {
        Ok(serde_json::to_string(data)?)
    }
}

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
    #[error("toml parsing error: {0}")]
    TomlDeserialize(#[from] toml::de::Error),
    #[error("toml serialize error: {0}")]
    TomlSerialize(#[from] toml::ser::Error),
    #[error("json serialize/deserialize error: {0}")]
    Json(#[from] serde_json::Error),
    #[error("json5 serialize/deserialize error: {0}")]
    Json5(#[from] json5::Error),
}
