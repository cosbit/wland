use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ClientIdentity {
    pub mac: String,
    pub hostname: Option<String>,
    pub vendor_oui: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ClientObserved {
    pub identity: ClientIdentity,
    pub ip_addresses: Vec<String>,
    pub connected_via: Option<String>,
    pub lan_binding: Option<String>,
    pub bss_binding: Option<String>,
    pub signal_dbm: Option<i32>,
    pub rx_rate_mbps: Option<f32>,
    pub tx_rate_mbps: Option<f32>,
    pub associated: bool,
    pub dhcp_lease_active: bool,
    pub last_seen_at: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WifiClientObserved {
    pub identity: ClientIdentity,
    pub ip_addresses: Vec<String>,
    pub connected_via: Option<String>,
    pub lan_binding: Option<String>,
    pub bss_binding: Option<String>,
    pub signal_dbm: Option<i32>,
    pub rx_rate_mbps: Option<f32>,
    pub tx_rate_mbps: Option<f32>,
    pub associated: bool,
    pub dhcp_lease_active: bool,
    pub last_seen_at: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ClientPolicyDesired {
    pub mac: String,
    pub name: Option<String>,
    pub blocked: bool,
    pub reserved_ip: Option<String>,
    pub allowed_lans: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ClientsRuntime {
    pub timestamp: Option<String>,
    pub stations: BTreeMap<String, ClientStation>,
    pub leases: BTreeMap<String, ClientLease>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ClientStation {
    pub mac: String,
    pub ip: Option<String>,
    pub hostname: Option<String>,
    pub bss: Option<String>,
    pub wlan: Option<String>,
    pub lan: Option<String>,
    pub signal_dbm: Option<i32>,
    pub rx_bytes: Option<u64>,
    pub tx_bytes: Option<u64>,
    pub associated_for: Option<String>,
    pub state: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ClientLease {
    pub mac: String,
    pub hostname: Option<String>,
    pub lan: Option<String>,
    pub expires_in: Option<String>,
}
