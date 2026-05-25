use serde::{Deserialize, Serialize};

use crate::common::schemas::{AddressMethod, InterfaceRef, Ipv4Config, Ipv6Config};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WanNatConfig {
    pub enabled: bool,
    pub masquerade: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WanDesired {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    pub interface: InterfaceRef,
    pub ipv4: Ipv4Config,
    pub ipv6: Ipv6Config,
    pub nat: WanNatConfig,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub firewall_profile: Option<String>,
}
