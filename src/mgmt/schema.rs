use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ManagementEndpoint {
    pub enabled: bool,
    pub listen: String,
    pub port: u16,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ManagementSshDesired {
    pub enabled: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ManagementDesired {
    pub ui: ManagementEndpoint,
    pub api: ManagementEndpoint,
    pub ssh: ManagementSshDesired,
}
