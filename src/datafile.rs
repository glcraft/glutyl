use serde::{Serialize, de::DeserializeOwned};
use std::{
    ops::{Deref, DerefMut},
    path::{Path, PathBuf},
};

use crate::fs::format::{Error as FormatError, Format};

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Configuration folder issue: {0}")]
    ConfigFolderNotFound(#[from] crate::fs::Error),
    #[error("format error: {0}")]
    Format(#[from] FormatError),
}

pub struct DataFile<Data> {
    path: PathBuf,
    format: Format,
    data: Data,
}

impl<Data> DataFile<Data>
where
    Data: DeserializeOwned,
{
    pub fn read<'a>(path: PathBuf, format: Format) -> Result<Self, Error> {
        let data = format.read_file(&path)?;
        log::info!("Configuration file loaded successfully");
        // log::debug!("Configuration file content: {data:?}");
        Ok(Self {
            path,
            format: format,
            data,
        })
    }
    pub fn read_or<'a>(path: PathBuf, format: Format, data: Data) -> Result<Self, Error> {
        let data = if path.exists() {
            format.read_file(&path)?
        } else {
            data
        };
        log::info!("Configuration file loaded successfully");
        // log::debug!("Configuration file content: {data:?}");
        Ok(Self {
            path,
            format: format,
            data,
        })
    }
    pub fn read_or_else<'a>(
        path: PathBuf,
        format: Format,
        data: impl FnOnce() -> Data,
    ) -> Result<Self, Error> {
        let data = if path.exists() {
            format.read_file(&path)?
        } else {
            data()
        };
        log::info!("Configuration file loaded successfully");
        // log::debug!("Configuration file content: {data:?}");
        Ok(Self {
            path,
            format: format,
            data,
        })
    }
    pub fn get_data(&self) -> &Data {
        &self.data
    }
}
impl<Data> DataFile<Data>
where
    Data: DeserializeOwned + Default,
{
    pub fn read_or_default<'a>(path: PathBuf, format: Format) -> Result<Self, Error> {
        let data = if path.exists() {
            format.read_file(&path)?
        } else {
            Default::default()
        };
        log::info!("Configuration file loaded successfully");
        // log::debug!("Configuration file content: {data:?}");
        Ok(Self {
            path,
            format: format,
            data,
        })
    }
}
impl<Data> DataFile<Data> {
    // fn path<'a>(init: &ConfigInit<'a>) -> Result<PathBuf, Error> {
    //     log::info!("Configuration file for {init:?}");
    //     let res = match &name {
    //         ConfigName::CustomPath(path) => path.clone(),
    //         ConfigName::Named {
    //             app: None,
    //             filename,
    //         } => crate::fs::StandardPaths::path()?.join(filename),
    //         ConfigName::Named {
    //             app: Some(app),
    //             filename,
    //         } => crate::fs::StandardPaths::config_with_name(app)?.join(filename),
    //     };
    //     if log::log_enabled!(log::Level::Info) {
    //         log::info!(
    //             "Configuration file location: {}",
    //             res.as_os_str().to_string_lossy()
    //         );
    //     }
    //     Ok(res)
    // }
    pub fn new<'a>(path: PathBuf, format: Format, data: Data) -> Result<Self, Error> {
        log::info!("Configuration file loaded successfully");
        // log::debug!("Configuration file content: {data:?}");
        Ok(Self {
            path,
            format: format,
            data,
        })
    }
}
impl<Data> DataFile<Data>
where
    Data: DeserializeOwned + Serialize,
{
    pub fn get_mut_data<'a>(&'a mut self) -> MutData<'a, Data> {
        MutData {
            path: self.path.as_path(),
            format: self.format,
            data: &mut self.data,
        }
    }
}
pub struct MutData<'a, Data>
where
    Data: Serialize,
{
    path: &'a Path,
    format: Format,
    data: &'a mut Data,
}

impl<'a, Data> Drop for MutData<'a, Data>
where
    Data: Serialize,
{
    fn drop(&mut self) {
        if let Err(e) = self.format.write_file(self.path, &self.data) {
            log::error!(
                "unable to write on file {}: {}",
                self.path.to_string_lossy(),
                e
            );
            panic!("unable to write on file {}", self.path.to_string_lossy());
        }
    }
}

impl<'a, Data> Deref for MutData<'a, Data>
where
    Data: Serialize,
{
    type Target = Data;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl<'a, Data> DerefMut for MutData<'a, Data>
where
    Data: Serialize,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}
