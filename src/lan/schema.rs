use serde::{Deserialize, Serialize};
use std::net::{IpAddr, Ipv4Addr};

#[allow(unused_imports)]
pub use crate::common::{
    BridgeCapabilities, BridgeDesired, BridgeFdbEntry, BridgeIdentity, BridgeObserved,
    DhcpServerDesired, DnsForwarderDesired, Ipv4InterfaceDesired, Ipv6InterfaceDesired, LanDesired,
};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LanConfig {
    pub name: String,
    pub description: Option<String>,
    pub bridge: LanBridge,
    pub ipv4: LanIpv4,
    pub dhcp: LanDhcp,
    pub dns: LanDns,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LanBridge {
    pub name: String,
    pub stp: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LanIpv4 {
    pub address: Ipv4Addr,
    pub prefix: u8,
    pub network: Ipv4Addr,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LanDhcp {
    pub enabled: bool,
    pub range_start: Ipv4Addr,
    pub range_end: Ipv4Addr,
    pub lease_time: String,
    pub dns_servers: Vec<IpAddr>,
    pub gateway: Ipv4Addr,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LanDns {
    pub enabled: bool,
    pub upstream: Vec<IpAddr>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct LanObserved {
    pub bridge: String,
    pub operstate: String,
    pub addresses: Vec<Ipv4Addr>,
    pub members: Vec<String>,
}
