use serde::{Deserialize, Serialize};

#[allow(unused_imports)]
pub use crate::common::contracts::{
    Ipv4InterfaceDesired, Ipv6InterfaceDesired, NatDesired, WanAddressMethod, WanDesired,
};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WanConfig {
    pub description: Option<String>,
    pub interface: WanInterface,
    pub ipv4: WanIpConfig,
    pub ipv6: WanIpConfig,
    pub nat: WanNat,
    pub firewall_profile: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WanInterface {
    pub name: String,
    pub interface_type: String,
    pub mac: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WanIpConfig {
    pub method: String,
    pub address: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WanNat {
    pub enabled: bool,
    pub masquerade: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct WanObserved {
    pub operstate: String,
    pub carrier: bool,
    pub mac: Option<String>,
    pub ipv4: Vec<String>,
    pub default_route: bool,
}
