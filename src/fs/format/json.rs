use super::{Error, Format};
use serde::{Deserialize, Serialize, de::DeserializeOwned};
use std::path::Path;
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
