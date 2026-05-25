#![allow(dead_code)]

use serde::{Deserialize, Serialize};
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DeviceKind {
    EthernetPort,
    WirelessPhy,
    WlanInterface,
    Bridge,
    VlanInterface,
    SwitchChip,
    SwitchPort,
    Loopback,
    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DeviceOwnership {
    Unmanaged,
    ObservedOnly,
    WlandManaged,
    ExternallyManaged,
    Conflicted,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AdminState {
    Up,
    Down,
    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OperState {
    Up,
    Down,
    Dormant,
    LowerLayerDown,
    NotPresent,
    Unknown,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DeviceIdentity {
    pub id: String,
    pub kind: DeviceKind,
    pub name: String,
    pub ifindex: Option<u32>,
    pub mac: Option<String>,
    pub driver: Option<String>,
    pub bus_path: Option<String>,
    pub sysfs_path: Option<String>,
    pub stable_path: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DeviceMetadata {
    pub description: Option<String>,
    pub ownership: DeviceOwnership,
    pub discovered_at: String,
    pub last_seen_at: String,
    pub tags: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DeviceObservedBase {
    pub admin_state: AdminState,
    pub oper_state: OperState,
    pub carrier: Option<bool>,
    pub mtu: Option<u32>,
    pub addresses: Vec<String>,
    pub routes: Vec<String>,
    pub master: Option<String>,
    pub lower_devices: Vec<String>,
    pub upper_devices: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DeviceContract<Caps, Desired, Observed> {
    pub identity: DeviceIdentity,
    pub metadata: DeviceMetadata,
    pub capabilities: Caps,
    pub desired: Option<Desired>,
    pub observed: Observed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DriftSeverity {
    None,
    Info,
    Warning,
    Critical,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DriftEvent {
    pub device_id: String,
    pub device_kind: DeviceKind,
    pub severity: DriftSeverity,
    pub field: String,
    pub desired: Option<String>,
    pub observed: Option<String>,
    pub message: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ApplyActionKind {
    CreateDevice,
    DeleteDevice,
    SetLinkUp,
    SetLinkDown,
    AssignAddress,
    RemoveAddress,
    AttachToBridge,
    DetachFromBridge,
    RenderConfig,
    StartService,
    StopService,
    ReloadService,
    ApplyFirewall,
    StartDhcpClient,
    StopDhcpClient,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ApplyAction {
    pub id: String,
    pub kind: ApplyActionKind,
    pub target: String,
    pub description: String,
    pub dependencies: Vec<String>,
    pub rollback: Option<Box<ApplyAction>>,
    pub dangerous: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ApplyPlan {
    pub id: String,
    pub created_at: String,
    pub actions: Vec<ApplyAction>,
    pub warnings: Vec<String>,
    pub requires_confirmation: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Ipv4InterfaceDesired {
    pub address: Option<Ipv4Addr>,
    pub prefix: Option<u8>,
    pub gateway: Option<Ipv4Addr>,
    pub dns_servers: Vec<IpAddr>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Ipv6InterfaceDesired {
    pub address: Option<Ipv6Addr>,
    pub prefix: Option<u8>,
    pub gateway: Option<Ipv6Addr>,
    pub dns_servers: Vec<IpAddr>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DhcpLeaseObserved {
    pub mac: String,
    pub hostname: Option<String>,
    pub lan: Option<String>,
    pub expires_in: Option<String>,
}

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
pub struct BridgeFdbEntry {
    pub mac: String,
    pub port: Option<String>,
    pub vlan_id: Option<u16>,
    pub local: bool,
    pub static_entry: bool,
    pub last_seen_at: Option<String>,
}

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
pub struct EthernetIdentity {
    pub ifname: String,
    pub ifindex: u32,
    pub mac: String,
    pub permanent_mac: Option<String>,
    pub driver: Option<String>,
    pub bus_path: Option<String>,
    pub pci_address: Option<String>,
    pub usb_path: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EthernetCapabilities {
    pub supports_carrier_detect: bool,
    pub supports_mtu_change: bool,
    pub supports_vlan: bool,
    pub supports_bridge_membership: bool,
    pub supports_wake_on_lan: bool,
    pub supported_speeds_mbps: Vec<u32>,
    pub supported_duplex: Vec<String>,
    pub max_mtu: Option<u32>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EthernetObserved {
    pub base: DeviceObservedBase,
    pub speed_mbps: Option<u32>,
    pub duplex: Option<String>,
    pub autoneg: Option<bool>,
    pub carrier_changes: Option<u64>,
    pub rx_bytes: Option<u64>,
    pub tx_bytes: Option<u64>,
    pub rx_packets: Option<u64>,
    pub tx_packets: Option<u64>,
    pub rx_errors: Option<u64>,
    pub tx_errors: Option<u64>,
    pub default_route: bool,
    pub dhcp_lease: Option<DhcpLeaseObserved>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EthernetRole {
    Wan,
    LanMember,
    Management,
    SwitchCpu,
    Unassigned,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EthernetDesired {
    pub role: EthernetRole,
    pub admin_state: AdminState,
    pub mtu: Option<u32>,
    pub ipv4: Option<Ipv4InterfaceDesired>,
    pub ipv6: Option<Ipv6InterfaceDesired>,
    pub bridge_member_of: Option<String>,
    pub vlan_parent: bool,
}

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
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WirelessPhyObserved {
    pub present: bool,
    pub country: Option<String>,
    pub regulatory_domain: Option<String>,
    pub active_channel: Option<u16>,
    pub active_frequency_mhz: Option<u32>,
    pub active_width_mhz: Option<u16>,
    pub active_interfaces: Vec<String>,
    pub rfkill_blocked: bool,
    pub tx_power_dbm: Option<f32>,
    pub noise_dbm: Option<f32>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ChannelPolicy {
    Fixed,
    Auto,
    PreferNonDfs,
    PreferDfs,
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WlanRuntimeMode {
    Ap,
    Station,
    Monitor,
    Unknown,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WlanInterfaceIdentity {
    pub ifname: String,
    pub ifindex: u32,
    pub mac: Option<String>,
    pub parent_phy: String,
    pub created_by_wland: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WlanInterfaceCapabilities {
    pub supported_modes: Vec<InterfaceMode>,
    pub can_bridge: bool,
    pub can_change_mac: bool,
    pub can_be_deleted: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WlanInterfaceObserved {
    pub base: DeviceObservedBase,
    pub mode: WlanRuntimeMode,
    pub parent_phy: String,
    pub bridge_member_of: Option<String>,
    pub hostapd_control_path: Option<String>,
    pub bss_id: Option<String>,
    pub ssid: Option<String>,
    pub clients: Vec<WifiClientObserved>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WlanInterfaceDesired {
    pub enabled: bool,
    pub ifname: String,
    pub parent_phy: String,
    pub mode: WlanRuntimeMode,
    pub bss_binding: String,
    pub lan_binding: String,
    pub bridge_member_of: String,
    pub generated: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BridgeIdentity {
    pub ifname: String,
    pub ifindex: u32,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BridgeCapabilities {
    pub supports_stp: bool,
    pub supports_vlan_filtering: bool,
    pub supports_multicast_snooping: bool,
    pub supports_hairpin_mode: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BridgeObserved {
    pub base: DeviceObservedBase,
    pub members: Vec<String>,
    pub stp_enabled: bool,
    pub vlan_filtering: bool,
    pub multicast_snooping: bool,
    pub fdb_entries: Vec<BridgeFdbEntry>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BridgeDesired {
    pub enabled: bool,
    pub ifname: String,
    pub stp_enabled: bool,
    pub vlan_filtering: bool,
    pub multicast_snooping: Option<bool>,
    pub ipv4: Option<Ipv4InterfaceDesired>,
    pub ipv6: Option<Ipv6InterfaceDesired>,
    pub members: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VlanInterfaceIdentity {
    pub ifname: String,
    pub ifindex: u32,
    pub parent: String,
    pub vlan_id: u16,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VlanInterfaceCapabilities {
    pub parent_supports_vlan: bool,
    pub supports_bridge_membership: bool,
    pub supports_mtu_change: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VlanInterfaceObserved {
    pub base: DeviceObservedBase,
    pub parent: String,
    pub vlan_id: u16,
    pub protocol: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VlanInterfaceDesired {
    pub enabled: bool,
    pub ifname: String,
    pub parent: String,
    pub vlan_id: u16,
    pub role: EthernetRole,
    pub bridge_member_of: Option<String>,
    pub ipv4: Option<Ipv4InterfaceDesired>,
    pub ipv6: Option<Ipv6InterfaceDesired>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SwitchChipIdentity {
    pub name: String,
    pub driver: Option<String>,
    pub devlink_name: Option<String>,
    pub bus_path: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SwitchChipCapabilities {
    pub port_count: u32,
    pub supports_vlan_filtering: bool,
    pub supports_port_isolation: bool,
    pub supports_stp_offload: bool,
    pub supports_lag: bool,
    pub supports_mirror: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SwitchChipObserved {
    pub present: bool,
    pub ports: Vec<String>,
    pub cpu_ports: Vec<String>,
    pub vlan_filtering: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SwitchChipDesired {
    pub enabled: bool,
    pub vlan_filtering: bool,
    pub ports: Vec<SwitchPortDesired>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SwitchPortMode {
    Access,
    Trunk,
    Routed,
    Unknown,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SwitchPortIdentity {
    pub ifname: String,
    pub ifindex: Option<u32>,
    pub switch_chip: String,
    pub port_index: u32,
    pub label: Option<String>,
    pub mac: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SwitchPortCapabilities {
    pub supports_access_mode: bool,
    pub supports_trunk_mode: bool,
    pub supports_isolation: bool,
    pub supports_stp_state: bool,
    pub supports_poe: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SwitchPortObserved {
    pub base: DeviceObservedBase,
    pub mode: SwitchPortMode,
    pub pvid: Option<u16>,
    pub tagged_vlans: Vec<u16>,
    pub untagged_vlans: Vec<u16>,
    pub isolated: bool,
    pub link_speed_mbps: Option<u32>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SwitchPortDesired {
    pub ifname: String,
    pub mode: SwitchPortMode,
    pub lan_binding: Option<String>,
    pub pvid: Option<u16>,
    pub tagged_vlans: Vec<u16>,
    pub untagged_vlans: Vec<u16>,
    pub isolated: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LoopbackIdentity {
    pub ifname: String,
    pub ifindex: u32,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LoopbackObserved {
    pub base: DeviceObservedBase,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LoopbackDesired {
    pub enabled: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WanAddressMethod {
    Dhcp,
    Static,
    Disabled,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WanDesired {
    pub name: String,
    pub interface: String,
    pub ipv4_method: WanAddressMethod,
    pub ipv6_method: WanAddressMethod,
    pub static_ipv4: Option<Ipv4InterfaceDesired>,
    pub static_ipv6: Option<Ipv6InterfaceDesired>,
    pub default_route: bool,
    pub nat_enabled: bool,
    pub masquerade: bool,
    pub firewall_profile: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WanObserved {
    pub interface: String,
    pub carrier: bool,
    pub has_default_route: bool,
    pub ipv4_addresses: Vec<String>,
    pub ipv6_addresses: Vec<String>,
    pub gateway_v4: Option<String>,
    pub gateway_v6: Option<String>,
    pub dns_servers: Vec<String>,
    pub dhcp_lease: Option<DhcpLeaseObserved>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LanDesired {
    pub name: String,
    pub bridge: String,
    pub description: Option<String>,
    pub ipv4: Option<Ipv4InterfaceDesired>,
    pub ipv6: Option<Ipv6InterfaceDesired>,
    pub dhcp: Option<DhcpServerDesired>,
    pub dns: Option<DnsForwarderDesired>,
    pub ethernet_members: Vec<String>,
    pub wlan_members: Vec<String>,
    pub vlan_id: Option<u16>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LanObserved {
    pub name: String,
    pub bridge: String,
    pub bridge_up: bool,
    pub addresses: Vec<String>,
    pub members: Vec<String>,
    pub dhcp_leases: Vec<DhcpLeaseObserved>,
    pub dns_running: bool,
    pub clients_seen: Vec<ClientObserved>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DhcpServerDesired {
    pub enabled: bool,
    pub lan_binding: String,
    pub interface: String,
    pub range_start: String,
    pub range_end: String,
    pub lease_time: String,
    pub gateway: String,
    pub dns_servers: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DnsForwarderDesired {
    pub enabled: bool,
    pub lan_binding: String,
    pub listen_interface: String,
    pub listen_address: String,
    pub upstream: Vec<String>,
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
