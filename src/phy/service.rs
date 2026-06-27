use anyhow::{anyhow, Context, Result};
use nl80211::Socket;

use crate::phy::schema::{
    ChannelCapability, InterfaceMode, WifiBand, WirelessPhyCapabilities, WirelessPhyIdentity,
    WirelessPhyObserved, WirelessPhyState,
};

pub async fn get_wireless_phy(phy_name: &str) -> Result<WirelessPhyState> {
    let wiphy_index = parse_phy_name(phy_name)?;
    if let Some(state) = query_nl80211_wireless_phy(wiphy_index).await? {
        return Ok(state);
    }
    Ok(empty_wireless_phy_state(phy_name, wiphy_index))
}

pub async fn query_nl80211_wireless_phy(wiphy_index: u32) -> Result<Option<WirelessPhyState>> {
    let mut socket = Socket::connect().context("failed to connect to nl80211")?;
    let _interfaces = socket
        .get_interfaces_info()
        .context("failed to request interfaces")?;
    Ok(Some(empty_wireless_phy_state(
        &format!("phy{}", wiphy_index),
        wiphy_index,
    )))
}

pub fn empty_wireless_phy_state(phy_name: &str, wiphy_index: u32) -> WirelessPhyState {
    WirelessPhyState {
        identity: WirelessPhyIdentity {
            phy_name: phy_name.to_string(),
            wiphy_index,
            path: None,
            driver: None,
            mac: None,
            bus_path: None,
        },
        capabilities: WirelessPhyCapabilities {
            supported_bands: Vec::new(),
            supported_modes: Vec::new(),
            supported_channels: Vec::new(),
            supports_ap: false,
            supports_multi_bss: false,
            max_ap_interfaces: None,
            max_interfaces_total: None,
            max_scan_ssids: None,
            supports_ht: false,
            supports_vht: false,
            supports_he: false,
            supports_eht: false,
            supports_dfs: false,
            supports_80211k: false,
            supports_80211v: false,
            supports_80211r: false,
            detailed: None,
        },
        observed: WirelessPhyObserved {
            present: true,
            country: None,
            regulatory_domain: None,
            active_band: None,
            active_channel: None,
            active_frequency_mhz: None,
            active_width_mhz: None,
            active_interfaces: Vec::new(),
            rfkill_blocked: false,
            tx_power_dbm: None,
            noise_dbm: None,
        },
        desired: None,
    }
}

pub fn parse_phy_name(phy_name: &str) -> Result<u32> {
    let index = phy_name
        .strip_prefix("phy")
        .ok_or_else(|| anyhow!("invalid phy name"))?;
    if index.is_empty() || !index.chars().all(|character| character.is_ascii_digit()) {
        return Err(anyhow!("invalid phy name"));
    }
    index.parse::<u32>().context("invalid phy index")
}

pub fn map_interface_mode(iftype: &str) -> Option<InterfaceMode> {
    match iftype {
        "Station" | "station" => Some(InterfaceMode::Station),
        "Ap" | "ap" => Some(InterfaceMode::Ap),
        "ApVlan" | "ap_vlan" => Some(InterfaceMode::ApVlan),
        "Monitor" | "monitor" => Some(InterfaceMode::Monitor),
        "MeshPoint" | "mesh_point" => Some(InterfaceMode::MeshPoint),
        "P2pClient" | "p2p_client" => Some(InterfaceMode::P2pClient),
        "P2pGo" | "p2p_go" => Some(InterfaceMode::P2pGo),
        "P2pDevice" | "p2p_device" => Some(InterfaceMode::P2pDevice),
        "Ibss" | "ibss" => Some(InterfaceMode::Ibss),
        "Nan" | "nan" => Some(InterfaceMode::Nan),
        _ => None,
    }
}

pub fn map_frequency_to_band(frequency_mhz: u32) -> Option<WifiBand> {
    match frequency_mhz {
        2400..=2499 => Some(WifiBand::Ghz2),
        5000..=5899 => Some(WifiBand::Ghz5),
        5925..=7125 => Some(WifiBand::Ghz6),
        _ => None,
    }
}

pub fn channel_capability(
    band: WifiBand,
    channel: u16,
    frequency_mhz: u32,
    disabled: bool,
    no_ir: bool,
    radar_required: bool,
    max_tx_power_dbm: Option<f32>,
    widths_mhz: Vec<u16>,
) -> ChannelCapability {
    ChannelCapability {
        band,
        channel,
        frequency_mhz,
        disabled,
        no_ir,
        radar_required,
        max_tx_power_dbm,
        widths_mhz,
        restrictions: Vec::new(),
        raw_flags: Vec::new(),
    }
}

pub fn derive_capability_summary(
    supported_modes: &[InterfaceMode],
    supported_channels: &[ChannelCapability],
    supports_multi_bss: bool,
    max_ap_interfaces: Option<u32>,
    max_interfaces_total: Option<u32>,
    max_scan_ssids: Option<u32>,
    supports_80211k: bool,
    supports_80211v: bool,
    supports_80211r: bool,
    supports_ht: bool,
    supports_vht: bool,
    supports_he: bool,
    supports_eht: bool,
) -> WirelessPhyCapabilities {
    let mut supported_bands = Vec::new();
    for channel in supported_channels {
        if !supported_bands.contains(&channel.band) {
            supported_bands.push(channel.band);
        }
    }
    WirelessPhyCapabilities {
        supported_bands,
        supported_modes: supported_modes.to_vec(),
        supported_channels: supported_channels.to_vec(),
        supports_ap: supported_modes.contains(&InterfaceMode::Ap),
        supports_multi_bss,
        max_ap_interfaces,
        max_interfaces_total,
        max_scan_ssids,
        supports_ht,
        supports_vht,
        supports_he,
        supports_eht,
        supports_dfs: supported_channels
            .iter()
            .any(|channel| channel.radar_required || channel.no_ir),
        supports_80211k,
        supports_80211v,
        supports_80211r,
        detailed: None,
    }
}
