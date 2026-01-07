use std::path::Path;

use serde::{Deserialize, Serialize, de::DeserializeOwned};

#[derive(Copy, Clone, Debug)]
pub enum Format {
    Toml,
    Json,
    Json5,
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

impl Format {
    pub fn extension(&self) -> &'static str {
        match self {
            Format::Toml => "toml",
            Format::Json => "json",
            Format::Json5 => "json5",
        }
    }
    pub fn write_file<Data: Serialize, P: AsRef<Path>>(
        self,
        path: P,
        data: Data,
    ) -> Result<(), Error> {
        use std::io::Write;
        let mut file = std::fs::File::create(path)?;
        match self {
            Format::Toml => {
                let toml_data = toml::to_string_pretty(&data)?;
                file.write_all(toml_data.as_bytes())?;
            }
            Format::Json => serde_json::to_writer(file, &data)?,
            Format::Json5 => json5::to_writer(file, &data)?,
        }
        Ok(())
    }
    pub fn read_file<Data: DeserializeOwned, P: AsRef<Path>>(self, path: P) -> Result<Data, Error> {
        use std::io::Read;
        let mut file = std::fs::File::open(path)?;
        match self {
            Format::Toml => {
                let mut content = String::new();
                file.read_to_string(&mut content)?;
                Ok(toml::from_str(&content)?)
            }
            Format::Json => Ok(serde_json::from_reader(file)?),
            Format::Json5 => {
                let mut content = String::new();
                file.read_to_string(&mut content)?;
                Ok(json5::from_str(&content)?)
            }
        }
    }
}
