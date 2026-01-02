use std::path::PathBuf;

#[derive(thiserror::Error, Clone, Debug)]
pub enum Error {
    #[error("unable to find user's home")]
    HomeNotFound,
}
pub struct StandardPaths;

macro_rules! path_join {
    ($($paths:expr),+) => {{
        std::path::PathBuf::new()
        $(
            .join($paths)
        )+
    }};
}

#[cfg(target_os = "macos")]
impl StandardPaths {
    pub fn config_path() -> Result<PathBuf, Error> {
        let mut path = std::env::home_dir().ok_or(Error::HomeNotFound)?;
        Ok(path_join!(path, "Library", "Application Support"))
    }
    pub fn config_with_name(prefix: &str) -> Result<PathBuf, Error> {
        Ok(path_join!(Self::config_path()?, prefix))
    }
    pub fn cache_path() -> Result<PathBuf, Error> {
        let mut path = std::env::home_dir().ok_or(Error::HomeNotFound)?;
        Ok(path_join!(path, "Library", "Caches"))
    }
    pub fn cache_with_name(prefix: &str) -> Result<PathBuf, Error> {
        Ok(path_join!(Self::cache_path()?, prefix))
    }
}
