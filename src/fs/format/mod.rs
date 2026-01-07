use std::path::Path;

use serde::{Serialize, de::DeserializeOwned};

#[derive(Copy, Clone, Debug)]
pub enum Format {
    Toml,
    Json,
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
}

impl Format {
    pub fn extension(&self) -> &'static str {
        match self {
            Format::Toml => "toml",
            Format::Json => "json",
        }
    }
    pub fn write_file<Data: Serialize, P: AsRef<Path>>(
        self,
        path: P,
        data: Data,
    ) -> Result<(), Error> {
        use std::io::Write;
        match self {
            Format::Toml => {
                let toml_data = toml::to_string_pretty(&data)?;
                let mut file = std::fs::File::create(path)?;
                file.write_all(toml_data.as_bytes())?;
            }
            Format::Json => {
                let file = std::fs::File::create(path)?;
                serde_json::to_writer(file, &data)?;
            }
        }
        Ok(())
    }
    pub fn read_file<Data: DeserializeOwned, P: AsRef<Path>>(self, path: P) -> Result<Data, Error> {
        match self {
            Format::Toml => {
                let content = std::fs::read_to_string(path)?;
                Ok(toml::from_str(&content)?)
            }
            Format::Json => {
                let file = std::fs::File::open(path)?;
                Ok(serde_json::from_reader(file)?)
            }
        }
    }
}
