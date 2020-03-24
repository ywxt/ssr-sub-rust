use super::LocalConfig;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ServerConfig {
    pub server: String,
    pub port: u16,
    pub protocol: String,
    pub method: String,
    pub obfs: String,
    pub password: String,
    pub obfs_param: Option<String>,
    pub proto_param: Option<String>,
    pub remarks: Option<String>,
    pub group: Option<String>,
    #[serde(flatten)]
    pub local_config: LocalConfig,
}