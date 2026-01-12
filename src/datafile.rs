use serde::{Serialize, de::DeserializeOwned};
use std::{
    marker::PhantomData,
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

pub struct DataFile<Data, Formatter: Format> {
    path: PathBuf,
    data: Data,
    phantom: PhantomData<Formatter>,
}

impl<Data, Formatter> DataFile<Data, Formatter>
where
    Data: DeserializeOwned,
    Formatter: Format,
{
    pub fn read<'a>(path: PathBuf) -> Result<Self, Error> {
        let data = Formatter::read_file(&path)?;
        log::info!("Configuration file loaded successfully");
        // log::debug!("Configuration file content: {data:?}");
        Ok(Self {
            path,
            data,
            phantom: PhantomData,
        })
    }
    pub fn read_or<'a>(path: PathBuf, data: Data) -> Result<Self, Error> {
        let data = if path.exists() {
            Formatter::read_file(&path)?
        } else {
            data
        };
        log::info!("Configuration file loaded successfully");
        // log::debug!("Configuration file content: {data:?}");
        Ok(Self {
            path,
            data,
            phantom: PhantomData,
        })
    }
    pub fn read_or_else<'a>(path: PathBuf, data: impl FnOnce() -> Data) -> Result<Self, Error> {
        let data = if path.exists() {
            Formatter::read_file(&path)?
        } else {
            data()
        };
        log::info!("Configuration file loaded successfully");
        // log::debug!("Configuration file content: {data:?}");
        Ok(Self {
            path,
            data,
            phantom: PhantomData,
        })
    }
    pub fn get_data(&self) -> &Data {
        &self.data
    }
}
impl<Data, Formatter> DataFile<Data, Formatter>
where
    Data: DeserializeOwned + Default,
    Formatter: Format,
{
    pub fn read_or_default<'a>(path: PathBuf) -> Result<Self, Error> {
        let data = if path.exists() {
            Formatter::read_file(&path)?
        } else {
            Default::default()
        };
        log::info!("Configuration file loaded successfully");
        // log::debug!("Configuration file content: {data:?}");
        Ok(Self {
            path,
            data,
            phantom: PhantomData,
        })
    }
}
impl<Data, Formatter> DataFile<Data, Formatter>
where
    Formatter: Format,
{
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
    pub fn new<'a>(path: PathBuf, data: Data) -> Result<Self, Error> {
        log::info!("Configuration file loaded successfully");
        // log::debug!("Configuration file content: {data:?}");
        Ok(Self {
            path,
            data,
            phantom: PhantomData,
        })
    }
}
impl<Data, Formatter> DataFile<Data, Formatter>
where
    Data: DeserializeOwned + Serialize,
    Formatter: Format,
{
    pub fn get_mut_data<'a>(&'a mut self) -> MutData<'a, Data, Formatter> {
        MutData {
            path: self.path.as_path(),
            data: &mut self.data,
            phantom: PhantomData,
        }
    }
}
pub struct MutData<'a, Data, Formatter>
where
    Data: Serialize,
    Formatter: Format,
{
    path: &'a Path,
    data: &'a mut Data,
    phantom: PhantomData<Formatter>,
}

impl<'a, Data, Formatter> Drop for MutData<'a, Data, Formatter>
where
    Data: Serialize,
    Formatter: Format,
{
    fn drop(&mut self) {
        if let Err(e) = Formatter::write_file(&self.path, &self.data) {
            log::error!(
                "unable to write on file {}: {}",
                self.path.to_string_lossy(),
                e
            );
            panic!("unable to write on file {}", self.path.to_string_lossy());
        }
    }
}

impl<'a, Data, Formatter> Deref for MutData<'a, Data, Formatter>
where
    Data: Serialize,
    Formatter: Format,
{
    type Target = Data;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl<'a, Data, Formatter> DerefMut for MutData<'a, Data, Formatter>
where
    Data: Serialize,
    Formatter: Format,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}
