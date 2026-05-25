use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BssConfig {
    pub ssid: String,
    pub description: Option<String>,
    pub security: BssSecurity,
    pub lan_assignment: String,
    pub policy: BssPolicy,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BssSecurity {
    pub mode: String,
    pub secret_ref: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BssPolicy {
    pub client_isolation: bool,
    pub mac_filter_mode: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BssObserved {
    pub state: String,
    pub ssid: String,
    pub clients: u32,
}
