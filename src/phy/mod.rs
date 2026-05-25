use anyhow::{anyhow, Context, Result};

pub mod cli;
pub mod schema;
pub use schema::*;

pub async fn get_wireless_phy(phy_name: &str) -> Result<WirelessPhyState> {
    let wiphy_index = parse_phy_name(phy_name)?;
    Ok(empty_wireless_phy_state(phy_name, wiphy_index))
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
    let index = phy_name.strip_prefix("phy").ok_or_else(|| anyhow!("invalid phy name"))?;
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
    ChannelCapability { band, channel, frequency_mhz, disabled, no_ir, radar_required, max_tx_power_dbm, widths_mhz, restrictions: Vec::new(), raw_flags: Vec::new() }
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
        supports_dfs: supported_channels.iter().any(|channel| channel.radar_required || channel.no_ir),
        supports_80211k,
        supports_80211v,
        supports_80211r,
        detailed: None,
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_phy_names() {
        assert_eq!(parse_phy_name("phy0").unwrap(), 0);
        assert_eq!(parse_phy_name("phy12").unwrap(), 12);
    }

    #[test]
    fn rejects_invalid_phy_names() {
        assert!(parse_phy_name("wiphy0").is_err());
        assert!(parse_phy_name("phy").is_err());
        assert!(parse_phy_name("phyx").is_err());
    }

    #[test]
    fn maps_frequency_to_band() {
        assert_eq!(map_frequency_to_band(2412), Some(WifiBand::Ghz2));
        assert_eq!(map_frequency_to_band(5180), Some(WifiBand::Ghz5));
        assert_eq!(map_frequency_to_band(5975), Some(WifiBand::Ghz6));
        assert_eq!(map_frequency_to_band(8000), None);
    }

    #[test]
    fn maps_interface_modes() {
        assert_eq!(map_interface_mode("Ap"), Some(InterfaceMode::Ap));
        assert_eq!(map_interface_mode("p2p_go"), Some(InterfaceMode::P2pGo));
        assert_eq!(map_interface_mode("unknown"), None);
    }

    #[test]
    fn builds_channel_capabilities() {
        let channel = channel_capability(WifiBand::Ghz5, 36, 5180, true, false, true, Some(15.0), vec![20, 40]);
        assert_eq!(channel.band, WifiBand::Ghz5);
        assert!(channel.disabled);
        assert!(channel.radar_required);
        assert_eq!(channel.widths_mhz, vec![20, 40]);
    }

    #[test]
    fn derives_capability_summary() {
        let supported_channels = vec![
            channel_capability(WifiBand::Ghz2, 1, 2412, false, false, false, None, vec![20]),
            channel_capability(WifiBand::Ghz5, 36, 5180, false, false, true, None, vec![20, 40]),
        ];
        let summary = derive_capability_summary(
            &[InterfaceMode::Station, InterfaceMode::Ap],
            &supported_channels,
            true,
            Some(4),
            Some(8),
            Some(20),
            true,
            false,
            false,
            true,
            false,
            false,
            false,
        );
        assert!(summary.supports_ap);
        assert!(summary.supports_dfs);
        assert_eq!(summary.supported_bands, vec![WifiBand::Ghz2, WifiBand::Ghz5]);
        assert_eq!(summary.max_ap_interfaces, Some(4));
        assert!(summary.supports_80211k);
    }
}
