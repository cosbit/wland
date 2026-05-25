use serde::{Deserialize, Serialize};
use std::net::Ipv4Addr;

use crate::common::schemas::{BridgeConfig, DurationString, Ipv4Config};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LanDesired {
    pub name: String,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    pub bridge: BridgeConfig,
    pub ipv4: Ipv4Config,
    pub dhcp: DhcpServerConfig,
    pub dns: DnsConfig,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub vlan_id: Option<u16>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DhcpServerConfig {
    pub enabled: bool,
    pub range_start: Ipv4Addr,
    pub range_end: Ipv4Addr,
    pub lease_time: DurationString,
    pub dns_servers: Vec<Ipv4Addr>,
    pub gateway: Ipv4Addr,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DnsConfig {
    pub enabled: bool,

    #[serde(default)]
    pub upstream: Vec<Ipv4Addr>,
}
