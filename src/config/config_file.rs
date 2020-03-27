use super::{LocalConfig, ServerConfig};
use crate::error;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::Write;
use std::path::{Path, PathBuf};

#[derive(Serialize, Deserialize, Debug)]
pub struct ConfigFile {
    pub local_config: LocalConfig,
    pub server_config: HashMap<String, Vec<ServerConfig>>,
}

impl ConfigFile {
    #[allow(dead_code)]
    pub const DEFAULT_NAME: &'static str = "config.json";
    #[allow(dead_code)]
    const APP_INFO: app_dirs2::AppInfo = app_dirs2::AppInfo {
        name: "SsrSub",
        author: "ywxt",
    };

    pub fn load(path: &impl AsRef<Path>) -> error::Result<Self> {
        let file = File::open(path)?;
        let buffer = io::BufReader::new(file);
        let config: ConfigFile = serde_json::from_reader(buffer)?;
        Ok(config)
    }
    /// load from follows:
    ///
    /// - Windows: "%APPDATA%\SuperDev\CoolApp\config.json"
    ///
    ///     (e.g.: "C:\Users\Rusty\AppData\Roaming\SuperDev\CoolApp\config.json")
    ///
    /// - macOS: "$HOME/Library/Application Support/CoolApp/config.json"
    ///
    ///     (e.g.: "/Users/Rusty/Library/Application Support/CoolApp/config.json")
    ///
    /// - *nix: "$HOME/.config/CoolApp/config.json" (or "$XDG_CONFIG_HOME/CoolApp/config.json", if defined)
    ///
    ///     (e.g.: "/home/rusty/.config/CoolApp/config.json")
    pub fn load_default() -> error::Result<Self> {
        let path = Self::get_default_file_path()?;
        Self::load(&path)
    }

    pub fn save(&self, path: &impl AsRef<Path>) -> error::Result<()> {
        let mut file = File::create(path)?;
        let config = serde_json::to_vec_pretty(self)?;
        file.write(&config)?;
        Ok(())
    }

    /// save to follows:
    ///
    /// - Windows: "%APPDATA%\SuperDev\CoolApp\config.json"
    ///
    ///     (e.g.: "C:\Users\Rusty\AppData\Roaming\SuperDev\CoolApp\config.json")
    ///
    /// - macOS: "$HOME/Library/Application Support/CoolApp/config.json"
    ///
    ///     (e.g.: "/Users/Rusty/Library/Application Support/CoolApp/config.json")
    ///
    /// - *nix: "$HOME/.config/CoolApp/config.json" (or "$XDG_CONFIG_HOME/CoolApp/config.json", if defined)
    ///
    ///     (e.g.: "/home/rusty/.config/CoolApp/config.json")
    pub fn save_default(&self) -> error::Result<()> {
        let path = Self::get_default_file_path()?;
        self.save(&path)
    }

    fn get_default_file_path() -> io::Result<PathBuf> {
        let mut path = app_dirs2::app_root(app_dirs2::AppDataType::UserConfig, &Self::APP_INFO)
            .map_err(|_| io::Error::from(io::ErrorKind::NotFound))?;
        path.push(Self::DEFAULT_NAME);
        Ok(path)
    }
}
