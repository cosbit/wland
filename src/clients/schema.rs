use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

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
