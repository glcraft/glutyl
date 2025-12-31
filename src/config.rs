use serde::{Serialize, de::DeserializeOwned};
use std::path::PathBuf;

pub struct Config<Data>
where
    Data: DeserializeOwned,
{
    path: PathBuf,
    data: Data,
}

impl<Data> Config<Data>
where
    Data: DeserializeOwned,
{
    pub fn from_config_dir(appname: &str) -> anyhow::Result<Self> {
        let path = match path {
            Some(v) => v,

            #[cfg(unix)]
            None => xdg::BaseDirectories::with_prefix("comptanalyzer")
                .get_config_file("config.kdl")
                .ok_or_else( || anyhow::anyhow!("Impossible de récupérer un chemin viable pour la configuration. Utilisez l'option --config pour assigner un fichier de configuration."))?,
            #[cfg(windows)]
            None => {
                let mut path = std::env::var_os("APPDATA").map(PathBuf::from).or_else(|| std::env::home_dir().map(|home| {home.push("AppData"); home})).ok_or_else(|| anyhow::anyhow!("Impossible de récupérer un chemin viable pour la configuration. Utilisez l'option --config pour assigner un fichier de configuration."))?;
                path.push("comptanalyzer");
                path.push("config.kdl");
                path
            }
        };
        let path_str = path.as_os_str().to_string_lossy();
        log::info!("Configuration file location: {path_str}");
        if !path.exists() {
            return Err(anyhow::anyhow!(
                "Le fichier de configuration n'existe pas à l'emplacement {path_str}."
            ));
        }
        let content = &std::fs::read_to_string(&path)?;
        // log::info!("content: {content}");
        let data = facet_kdl::from_str(&content)?;
        Ok(Self { path, data })
    }
    pub fn get_data(&self) -> &ConfigData {
        &self.data
    }
}
