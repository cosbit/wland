use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NftTableDesired {
    pub name: String,
    pub family: String,
    pub description: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FirewallProfileDesired {
    pub name: String,
    pub description: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FirewallDesired {
    pub backend: String,
    pub tables: Vec<NftTableDesired>,
    pub profiles: Vec<FirewallProfileDesired>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NatDesired {
    pub enabled: bool,
    pub source_lan: String,
    pub outbound_wan: String,
    pub masquerade: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FirewallConfig {
    pub backend: String,
    pub profile: String,
    pub zones: BTreeMap<String, FirewallZone>,
    pub port_forwards: Vec<FirewallPortForward>,
    pub custom_rules: FirewallCustomRules,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FirewallZone {
    pub zone_type: String,
    pub interfaces: Vec<String>,
    pub input_policy: String,
    pub forward_policy: String,
    pub masquerade_to: Vec<String>,
    pub router_services: Vec<String>,
    pub deny_zones: Vec<String>,
    pub allow_ping: Option<bool>,
    pub allow_dhcp_client: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FirewallPortForward {
    pub name: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FirewallCustomRules {
    pub enabled: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FirewallApplyRecord {
    pub timestamp: String,
    pub result: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FirewallObserved {
    pub backend: String,
    pub state: String,
    pub mode: String,
    pub active_tables: Vec<String>,
    pub ruleset_checksum: Option<String>,
    pub desired_checksum: Option<String>,
    pub drift: bool,
    pub last_apply: Option<FirewallApplyRecord>,
}
