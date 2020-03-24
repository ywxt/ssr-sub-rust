use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct LocalConfig {
    pub local_address: String,
    pub local_port: String,
    pub udp: bool,
}