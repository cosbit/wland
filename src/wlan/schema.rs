use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WlanConfig {
    pub description: Option<String>,
    pub phy_binding: String,
    pub bss_binding: String,
    pub lan_binding: String,
    pub interface: WlanInterface,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WlanInterface {
    pub name: String,
    pub mode: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WlanObserved {
    pub present: bool,
    pub up: bool,
    pub phy_binding: Option<String>,
    pub bss_binding: Option<String>,
    pub bridge: Option<String>,
}
