use super::{Error, Format};
use serde::{Deserialize, Serialize};
use std::path::Path;

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
