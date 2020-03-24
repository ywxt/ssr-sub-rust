use super::{LocalConfig, ServerConfig};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
pub struct ConfigFile {
    pub local_config: LocalConfig,
    pub server_config: HashMap<String, Vec<ServerConfig>>,
}
