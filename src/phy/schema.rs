use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WifiBand {
    Ghz2,
    Ghz5,
    Ghz6,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InterfaceMode {
    Station,
    Ap,
    ApVlan,
    Monitor,
    MeshPoint,
    P2pClient,
    P2pGo,
    P2pDevice,
    Ibss,
    Nan,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PhyStandard {
    Ht,
    Vht,
    He,
    Eht,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BandwidthRelation {
    Equal,
    LessOrEqual,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct BandwidthScope {
    pub relation: BandwidthRelation,
    pub mhz: u16,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ChannelRestriction {
    Disabled,
    NoIr,
    Radar,
    NoOutdoor,
    IndoorOnly,
    PassiveScan,
    NoHt40Plus,
    NoHt40Minus,
    No80Mhz,
    No160Mhz,
    No320Mhz,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChannelCapability {
    pub band: WifiBand,
    pub channel: u16,
    pub frequency_mhz: u32,
    pub disabled: bool,
    pub no_ir: bool,
    pub radar_required: bool,
    pub max_tx_power_dbm: Option<f32>,
    pub widths_mhz: Vec<u16>,

    /// Parsed restrictions from the `iw phy` frequency line.
    ///
    /// Keep the booleans above for common policy checks, but this allows the
    /// parser to preserve additional flags without reshaping the contract.
    pub restrictions: Vec<ChannelRestriction>,

    /// Any unrecognized raw frequency flags.
    pub raw_flags: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LegacyBitrate {
    pub mbps: f32,
    pub short_preamble_supported: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ChannelPolicy {
    Automatic,
    Manual,
    Fixed,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WirelessPhyIdentity {
    pub phy_name: String,
    pub wiphy_index: u32,
    pub path: Option<String>,
    pub driver: Option<String>,
    pub mac: Option<String>,
    pub bus_path: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WirelessPhyCapabilities {
    pub supported_bands: Vec<WifiBand>,
    pub supported_modes: Vec<InterfaceMode>,
    pub supported_channels: Vec<ChannelCapability>,
    pub supports_ap: bool,
    pub supports_multi_bss: bool,
    pub max_ap_interfaces: Option<u32>,
    pub max_interfaces_total: Option<u32>,
    pub max_scan_ssids: Option<u32>,
    pub supports_ht: bool,
    pub supports_vht: bool,
    pub supports_he: bool,
    pub supports_eht: bool,
    pub supports_dfs: bool,
    pub supports_80211k: bool,
    pub supports_80211v: bool,
    pub supports_80211r: bool,
    pub detailed: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WirelessPhyObserved {
    pub present: bool,
    pub country: Option<String>,
    pub regulatory_domain: Option<String>,
    pub active_band: Option<WifiBand>,
    pub active_channel: Option<u16>,
    pub active_frequency_mhz: Option<u32>,
    pub active_width_mhz: Option<u16>,
    pub active_interfaces: Vec<String>,
    pub rfkill_blocked: bool,
    pub tx_power_dbm: Option<f32>,
    pub noise_dbm: Option<f32>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WirelessPhyDesired {
    pub enabled: bool,
    pub country: String,
    pub band: Option<WifiBand>,
    pub channel: Option<u16>,
    pub width_mhz: Option<u16>,
    pub tx_power_dbm: Option<f32>,
    pub channel_policy: ChannelPolicy,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WirelessPhyState {
    pub identity: WirelessPhyIdentity,
    pub capabilities: WirelessPhyCapabilities,
    pub observed: WirelessPhyObserved,
    pub desired: Option<WirelessPhyDesired>,
}

/// Existing config-facing shape.
/// Keep this if other contracts already depend on it, but prefer
/// `WirelessPhyDesired` for new API surfaces.
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

/// Existing lightweight observed shape.
/// Keep this for compatibility, but prefer `WirelessPhyObserved` plus
/// `WirelessPhyCapabilities` for the actual state model.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PhyObserved {
    pub present: bool,
    pub driver: Option<String>,
    pub bands: Vec<String>,
    pub supports_ap: bool,
    pub supports_multi_bss: bool,
}
