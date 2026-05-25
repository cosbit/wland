use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct DbusConfig {
    pub enabled: bool,
    pub bus_name: String,
    pub object_path: String,
}
