use super::{LocalConfig, ServerConfig};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::fs::File;
use crate::error;
use std::io;
use std::io::Write;

#[derive(Serialize, Deserialize, Debug)]
pub struct ConfigFile {
    pub local_config: LocalConfig,
    pub server_config: HashMap<String, Vec<ServerConfig>>,
}

impl ConfigFile {

    #[allow(dead_code)]
    pub const DEFAULT_PATH : &'static str = ".ssr-sub/config.json";

    pub  fn load<P:AsRef<Path>>(path:&P) -> error::Result<Self> {
        let file= File::open(path)?;
        let buffer = io::BufReader::new(file);
        let config : ConfigFile = serde_json::from_reader(buffer)?;
        Ok(config)
    }
    /// load from `$HOME/`[`Self::DEFAULT_PATH`]
    pub fn load_default() ->error::Result<Self> {
        let path = Self::get_default_file_path()?;
        Self::load(&path)
    }

    pub fn save<P:AsRef<Path>>(&self,path:&P) -> error::Result<()> {
        let mut file = File::create(path)?;
        let config = serde_json::to_vec_pretty(self)?;
        file.write(&config)?;
        Ok(())
    }

    /// save to `$HOME/`[`Self::DEFAULT_PATH`]
    pub fn save_default(&self) -> error::Result<()> {
        let path = Self::get_default_file_path()?;
        self.save(&path)
    }

    fn get_default_file_path()->io::Result<PathBuf> {
        let mut path = dirs::home_dir().ok_or(io::Error::from(io::ErrorKind::NotFound))?;
        path.push(Self::DEFAULT_PATH);
        Ok(path)
    }

}