use serde::{Deserialize, Serialize};

#[allow(unused_imports)]
pub use crate::common::{
    ChannelCapability, ChannelPolicy, InterfaceMode, WifiBand, WirelessPhyCapabilities,
    WirelessPhyDesired, WirelessPhyIdentity, WirelessPhyObserved,
};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PhyConfig {
    pub description: Option<String>,
    pub country: String,
    pub band: String,
    pub channel: u16,
    pub width: String,
    pub txpower_dbm: i32,
    pub hw_mode: String,
    pub channel_policy: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PhyObserved {
    pub present: bool,
    pub driver: Option<String>,
    pub bands: Vec<String>,
    pub supports_ap: bool,
    pub supports_multi_bss: bool,
}
