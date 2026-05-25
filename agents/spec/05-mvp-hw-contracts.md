# WLANd Hardware and Runtime Device Contracts

This document defines the internal contracts WLANd should use for hardware-backed and kernel-created network devices.

The goal is to keep discovery, validation, planning, rendering, apply, and observed-state collection cleanly separated. Each device contract answers five questions:

1. What is this device?
2. What can this device do?
3. What is it doing right now?
4. What does WLANd want it to do?
5. What operations may WLANd safely perform?

---

## 1. Contract Layers

WLANd should avoid treating Linux networking as a pile of commands. Each device should have a structured contract with these layers:

```text
Hardware identity
  -> capabilities
  -> observed state
  -> desired intent
  -> validation
  -> apply plan
  -> runtime events
```

Each contract should be explicit about whether the device is:

- physical hardware
- a virtual kernel device
- a logical WLANd object
- a backend service attached to a device

---

## 2. Common Device Contract

All device contracts should implement the common shape below.

```rust
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

pub enum DeviceOwnership {
    Unmanaged,
    ObservedOnly,
    WlandManaged,
    ExternallyManaged,
    Conflicted,
}

pub enum AdminState {
    Up,
    Down,
    Unknown,
}

pub enum OperState {
    Up,
    Down,
    Dormant,
    LowerLayerDown,
    NotPresent,
    Unknown,
}

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

pub struct DeviceMetadata {
    pub description: Option<String>,
    pub ownership: DeviceOwnership,
    pub discovered_at: String,
    pub last_seen_at: String,
    pub tags: Vec<String>,
}

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

pub struct DeviceContract<Caps, Desired, Observed> {
    pub identity: DeviceIdentity,
    pub metadata: DeviceMetadata,
    pub capabilities: Caps,
    pub desired: Option<Desired>,
    pub observed: Observed,
}
```

### Common Requirements

Every device contract must support:

- stable identity
- current observed state
- capabilities
- desired state, if WLANd owns the device
- ownership status
- validation errors
- apply-plan generation
- drift detection

### Common Validation Rules

A device is invalid if:

- its identity is ambiguous
- WLANd thinks it owns the device but another manager is actively controlling it
- desired state references a missing lower-level device
- desired state requests unsupported capabilities
- desired state would remove the current management path without explicit override

---

## 3. Ethernet Port Contract

An Ethernet port is a physical or virtual wired network interface exposed by Linux.

Examples:

- `enp2s0`
- `eth0`
- `eno1`
- `usb0`
- `lan1`

Ethernet ports are usually candidates for:

- WAN uplink
- LAN bridge member
- management interface
- switch CPU port
- VLAN parent interface

---

### 3.1 Identity

```rust
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
```

### 3.2 Capabilities

```rust
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
```

### 3.3 Observed State

```rust
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
```

### 3.4 Desired State

```rust
pub enum EthernetRole {
    Wan,
    LanMember,
    Management,
    SwitchCpu,
    Unassigned,
}

pub struct EthernetDesired {
    pub role: EthernetRole,
    pub admin_state: AdminState,
    pub mtu: Option<u32>,
    pub ipv4: Option<Ipv4InterfaceDesired>,
    pub ipv6: Option<Ipv6InterfaceDesired>,
    pub bridge_member_of: Option<String>,
    pub vlan_parent: bool,
}
```

### 3.5 Allowed Operations

WLANd may:

- bring the interface up or down
- assign or remove IP addresses
- start or stop DHCP client behavior
- attach or detach the interface from a bridge
- create VLAN subinterfaces
- set MTU if supported
- classify the interface as WAN, LAN, or unassigned

WLANd should not:

- rename a physical interface by default
- change permanent MAC address
- enslave a WAN interface into a LAN bridge
- remove the only active management path without explicit override

### 3.6 Validation Rules

A valid Ethernet desired state must satisfy:

- an interface cannot be both `Wan` and `LanMember`
- a WAN interface cannot be a bridge member unless explicitly configured for bridge-WAN mode
- DHCP client mode and static addressing are mutually exclusive for the same address family
- requested MTU must be less than or equal to max MTU
- bridge membership requires the bridge to exist in desired state
- default route behavior must be explicit for WAN candidates

---

## 4. Wireless PHY Contract

A wireless PHY is a physical radio exposed by Linux as `phy0`, `phy1`, etc.

The PHY is hardware. WLANd should discover PHYs, not invent them.

A PHY can host one or more WLAN interfaces depending on driver and hardware capabilities.

---

### 4.1 Identity

```rust
pub struct WirelessPhyIdentity {
    pub phy_name: String,
    pub wiphy_index: u32,
    pub path: Option<String>,
    pub driver: Option<String>,
    pub mac: Option<String>,
    pub bus_path: Option<String>,
}
```

### 4.2 Capabilities

```rust
pub enum WifiBand {
    Ghz2,
    Ghz5,
    Ghz6,
}

pub enum InterfaceMode {
    Station,
    Ap,
    ApVlan,
    Monitor,
    MeshPoint,
    P2pClient,
    P2pGo,
}

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
```

### 4.3 Observed State

```rust
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
```

### 4.4 Desired State

```rust
pub enum ChannelPolicy {
    Fixed,
    Auto,
    PreferNonDfs,
    PreferDfs,
}

pub struct WirelessPhyDesired {
    pub enabled: bool,
    pub country: String,
    pub band: Option<WifiBand>,
    pub channel: Option<u16>,
    pub width_mhz: Option<u16>,
    pub tx_power_dbm: Option<f32>,
    pub channel_policy: ChannelPolicy,
}
```

### 4.5 Allowed Operations

WLANd may:

- read PHY capabilities through nl80211
- set regulatory country where allowed
- select channel/band/width policy
- create AP-mode WLAN interfaces
- remove WLAN interfaces created by WLANd
- validate BSS placement against PHY limits
- observe current channel and interface state

WLANd should not:

- invent PHYs in desired state
- assume multi-BSS support
- assume 6 GHz AP support just because the adapter supports 6 GHz client mode
- ignore DFS/radar constraints
- override global regulatory state without explicit policy

### 4.6 Validation Rules

A valid PHY desired state must satisfy:

- requested band must be supported
- requested channel must exist and not be disabled
- requested width must be supported on the selected channel
- AP mode must be supported if any BSS is bound to this PHY
- BSS count must not exceed `max_ap_interfaces`
- 6 GHz policy must enforce compatible security requirements
- DFS channels must either be allowed with CAC handling or rejected
- country code must exist before channel decisions are finalized

---

## 5. WLAN Interface Contract

A WLAN interface is a concrete Linux network interface created on top of a PHY.

Examples:

- `wlan0`
- `wlan0-ap0`
- `wlan0-ap1`
- `wlan0-guest`

A WLAN interface is not the same thing as a BSS. It is the runtime interface used to host or attach a BSS.

---

### 5.1 Identity

```rust
pub struct WlanInterfaceIdentity {
    pub ifname: String,
    pub ifindex: u32,
    pub mac: Option<String>,
    pub parent_phy: String,
    pub created_by_wland: bool,
}
```

### 5.2 Capabilities

```rust
pub struct WlanInterfaceCapabilities {
    pub supported_modes: Vec<InterfaceMode>,
    pub can_bridge: bool,
    pub can_change_mac: bool,
    pub can_be_deleted: bool,
}
```

### 5.3 Observed State

```rust
pub enum WlanRuntimeMode {
    Ap,
    Station,
    Monitor,
    Unknown,
}

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
```

### 5.4 Desired State

```rust
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
```

### 5.5 Allowed Operations

WLANd may:

- create the WLAN interface on a PHY
- delete WLAN interfaces it created
- bring the interface up or down
- attach the interface to a bridge
- detach the interface from a bridge
- pass the interface to hostapd
- collect associated-client state through hostapd

WLANd should not:

- delete pre-existing WLAN interfaces unless explicitly adopted
- move a WLAN interface between PHYs
- attach one WLAN interface to multiple LAN bridges
- start AP mode without a valid BSS binding
- start AP mode without a valid LAN binding

### 5.6 Validation Rules

A valid WLAN desired state must satisfy:

- parent PHY exists
- parent PHY supports AP mode
- BSS binding exists
- LAN binding exists
- bridge target exists
- interface name is unique
- interface count stays within PHY limits
- generated interfaces are marked as WLANd-owned

---

## 6. Bridge Contract

A bridge is a virtual Linux Layer 2 device used to implement a LAN segment.

Examples:

- `br-lan0`
- `br-guest0`
- `br-iot0`

A bridge is not physical hardware, but it is a critical runtime device and should have a contract.

---

### 6.1 Identity

```rust
pub struct BridgeIdentity {
    pub ifname: String,
    pub ifindex: u32,
}
```

### 6.2 Capabilities

```rust
pub struct BridgeCapabilities {
    pub supports_stp: bool,
    pub supports_vlan_filtering: bool,
    pub supports_multicast_snooping: bool,
    pub supports_hairpin_mode: bool,
}
```

### 6.3 Observed State

```rust
pub struct BridgeObserved {
    pub base: DeviceObservedBase,
    pub members: Vec<String>,
    pub stp_enabled: bool,
    pub vlan_filtering: bool,
    pub multicast_snooping: bool,
    pub fdb_entries: Vec<BridgeFdbEntry>,
}
```

### 6.4 Desired State

```rust
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
```

### 6.5 Allowed Operations

WLANd may:

- create bridges
- delete WLANd-owned bridges
- assign router IP addresses to bridges
- attach or detach member interfaces
- enable or disable STP
- enable or disable VLAN filtering
- observe FDB entries

WLANd should not:

- attach WAN interfaces to LAN bridges unless explicitly configured
- delete externally managed bridges
- assign duplicate subnets
- bridge two LANs together unless the desired-state model says they are the same LAN

### 6.6 Validation Rules

A valid bridge desired state must satisfy:

- bridge name is unique
- all members exist or are planned for creation
- members are not already attached to another bridge
- bridge IP subnet does not overlap another LAN
- DHCP range, if present, fits inside the bridge subnet
- router address is inside the bridge subnet
- bridge does not contain the active WAN unless explicitly allowed

---

## 7. VLAN Interface Contract

A VLAN interface is a tagged logical interface on top of a parent Ethernet port, bridge, or switch port.

Examples:

- `enp2s0.10`
- `br-lan0.20`
- `lan1.30`

---

### 7.1 Identity

```rust
pub struct VlanInterfaceIdentity {
    pub ifname: String,
    pub ifindex: u32,
    pub parent: String,
    pub vlan_id: u16,
}
```

### 7.2 Capabilities

```rust
pub struct VlanInterfaceCapabilities {
    pub parent_supports_vlan: bool,
    pub supports_bridge_membership: bool,
    pub supports_mtu_change: bool,
}
```

### 7.3 Observed State

```rust
pub struct VlanInterfaceObserved {
    pub base: DeviceObservedBase,
    pub parent: String,
    pub vlan_id: u16,
    pub protocol: String,
}
```

### 7.4 Desired State

```rust
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
```

### 7.5 Allowed Operations

WLANd may:

- create VLAN interfaces
- delete WLANd-owned VLAN interfaces
- assign IP addresses
- attach VLAN interfaces to bridges
- use VLAN interfaces as WAN or LAN members

WLANd should not:

- create duplicate VLAN IDs on the same parent
- create VLANs on parents without VLAN support
- use VLAN ID 0 or invalid IDs
- silently change parent interface ownership

### 7.6 Validation Rules

A valid VLAN desired state must satisfy:

- parent exists
- parent supports VLAN interfaces
- VLAN ID is between 1 and 4094
- parent plus VLAN ID is unique
- role does not conflict with parent role
- bridge membership target exists if configured

---

## 8. Switch Chip Contract

A switch chip is a hardware switching fabric exposed through Linux, commonly through DSA.

Examples:

- embedded router switch ASIC
- multi-port Ethernet switch exposed as `lan1`, `lan2`, `lan3`, `wan`
- CPU-facing switch port

This can be deferred for early MVP if the first target platform uses normal independent NICs.

---

### 8.1 Identity

```rust
pub struct SwitchChipIdentity {
    pub name: String,
    pub driver: Option<String>,
    pub devlink_name: Option<String>,
    pub bus_path: Option<String>,
}
```

### 8.2 Capabilities

```rust
pub struct SwitchChipCapabilities {
    pub port_count: u32,
    pub supports_vlan_filtering: bool,
    pub supports_port_isolation: bool,
    pub supports_stp_offload: bool,
    pub supports_lag: bool,
    pub supports_mirror: bool,
}
```

### 8.3 Observed State

```rust
pub struct SwitchChipObserved {
    pub present: bool,
    pub ports: Vec<String>,
    pub cpu_ports: Vec<String>,
    pub vlan_filtering: bool,
}
```

### 8.4 Desired State

```rust
pub struct SwitchChipDesired {
    pub enabled: bool,
    pub vlan_filtering: bool,
    pub ports: Vec<SwitchPortDesired>,
}
```

### 8.5 Allowed Operations

WLANd may:

- observe switch topology
- map switch ports to Linux interfaces
- configure VLAN filtering if supported
- configure port isolation if supported
- assign ports to LANs

WLANd should not:

- assume all Ethernet ports are independent NICs
- assume switch ports can route independently
- configure switch ASIC features before ownership is explicit
- break CPU-port connectivity

### 8.6 Validation Rules

A valid switch desired state must satisfy:

- CPU port remains connected
- port VLAN membership does not isolate management unintentionally
- each access port has one untagged VLAN unless configured otherwise
- trunk ports have explicit allowed VLANs
- switch chip supports requested offload behavior

---

## 9. Switch Port Contract

A switch port is a physical external port controlled by a switch chip.

Examples:

- `lan1`
- `lan2`
- `lan3`
- `wan`

A switch port may appear as a Linux interface, but behavior depends on the DSA/switch model.

---

### 9.1 Identity

```rust
pub struct SwitchPortIdentity {
    pub ifname: String,
    pub ifindex: Option<u32>,
    pub switch_chip: String,
    pub port_index: u32,
    pub label: Option<String>,
    pub mac: Option<String>,
}
```

### 9.2 Capabilities

```rust
pub struct SwitchPortCapabilities {
    pub supports_access_mode: bool,
    pub supports_trunk_mode: bool,
    pub supports_isolation: bool,
    pub supports_stp_state: bool,
    pub supports_poe: bool,
}
```

### 9.3 Observed State

```rust
pub enum SwitchPortMode {
    Access,
    Trunk,
    Routed,
    Unknown,
}

pub struct SwitchPortObserved {
    pub base: DeviceObservedBase,
    pub mode: SwitchPortMode,
    pub pvid: Option<u16>,
    pub tagged_vlans: Vec<u16>,
    pub untagged_vlans: Vec<u16>,
    pub isolated: bool,
    pub link_speed_mbps: Option<u32>,
}
```

### 9.4 Desired State

```rust
pub struct SwitchPortDesired {
    pub ifname: String,
    pub mode: SwitchPortMode,
    pub lan_binding: Option<String>,
    pub pvid: Option<u16>,
    pub tagged_vlans: Vec<u16>,
    pub untagged_vlans: Vec<u16>,
    pub isolated: bool,
}
```

### 9.5 Allowed Operations

WLANd may:

- assign a switch port to a LAN
- configure access/trunk behavior
- configure VLAN membership
- configure isolation if supported
- observe link state and speed

WLANd should not:

- configure port VLANs unless switch ownership is explicit
- isolate the management port without confirmation
- use the same untagged VLAN ambiguously
- treat a switch port exactly like a standalone NIC without checking topology

### 9.6 Validation Rules

A valid switch port desired state must satisfy:

- referenced switch chip exists
- VLAN IDs are valid
- access mode has exactly one untagged VLAN
- trunk mode has explicit allowed VLANs
- routed mode is not also a bridge member
- management connectivity is preserved

---

## 10. Loopback Contract

Loopback is a kernel interface, not external hardware. WLANd should observe it but generally not manage it.

---

### 10.1 Identity

```rust
pub struct LoopbackIdentity {
    pub ifname: String,
    pub ifindex: u32,
}
```

### 10.2 Observed State

```rust
pub struct LoopbackObserved {
    pub base: DeviceObservedBase,
}
```

### 10.3 Desired State

```rust
pub struct LoopbackDesired {
    pub enabled: bool,
}
```

### 10.4 Rules

WLANd should:

- expect `lo` to exist
- allow loopback in generated firewall policy
- avoid taking ownership of loopback configuration

WLANd should not:

- delete loopback
- rename loopback
- attach loopback to bridges
- expose loopback as a normal LAN/WAN candidate

---

## 11. WAN Uplink Contract

A WAN is a logical role assigned to an Ethernet port, VLAN interface, modem interface, or other routed upstream device.

Although WAN is not itself hardware, WLANd needs a clear contract for upstream connectivity.

---

### 11.1 Desired State

```rust
pub enum WanAddressMethod {
    Dhcp,
    Static,
    Disabled,
}

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
```

### 11.2 Observed State

```rust
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
```

### 11.3 Validation Rules

A valid WAN desired state must satisfy:

- interface exists
- interface is not a LAN bridge member
- default route behavior is explicit
- NAT source LANs are explicit
- firewall profile exists
- static gateway is reachable through the WAN subnet
- DHCP and static addressing are not both enabled for the same address family

---

## 12. LAN Segment Contract

A LAN is a logical local network segment. It usually maps to a Linux bridge plus IP addressing, DHCP, DNS, and firewall policy.

LAN is not raw hardware, but it binds physical and virtual devices together.

---

### 12.1 Desired State

```rust
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
```

### 12.2 Observed State

```rust
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
```

### 12.3 Validation Rules

A valid LAN desired state must satisfy:

- bridge exists or is planned for creation
- router address is inside the LAN subnet
- DHCP range is inside the LAN subnet
- DHCP range does not include router address
- LAN subnet does not overlap another LAN
- members exist or are planned for creation
- members are not WAN interfaces
- VLAN ID, if present, is valid

---

## 13. Client Device Contract

A client device is not WLANd-owned hardware, but client visibility is part of the controller experience.

Clients can be observed through:

- hostapd association tables
- DHCP leases
- ARP/neighbor tables
- bridge FDB entries

---

### 13.1 Identity

```rust
pub struct ClientIdentity {
    pub mac: String,
    pub hostname: Option<String>,
    pub vendor_oui: Option<String>,
}
```

### 13.2 Observed State

```rust
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
```

### 13.3 Desired Policy

```rust
pub struct ClientPolicyDesired {
    pub mac: String,
    pub name: Option<String>,
    pub blocked: bool,
    pub reserved_ip: Option<String>,
    pub allowed_lans: Vec<String>,
}
```

### 13.4 Validation Rules

A valid client policy must satisfy:

- MAC address is valid
- reserved IP is inside the assigned LAN subnet
- reserved IP does not collide with DHCP pool unless explicitly reserved
- blocked clients should compile to hostapd or firewall policy depending on enforcement backend

---

## 14. Backend Service Attachment Contracts

These are not hardware devices, but each service is attached to devices and should have explicit contracts.

---

### 14.1 DHCP Server Attachment

```rust
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
```

Validation:

- LAN exists
- interface is LAN bridge
- range is inside subnet
- gateway is router address
- DNS servers are valid IPs

---

### 14.2 DNS Forwarder Attachment

```rust
pub struct DnsForwarderDesired {
    pub enabled: bool,
    pub lan_binding: String,
    pub listen_interface: String,
    pub listen_address: String,
    pub upstream: Vec<String>,
}
```

Validation:

- LAN exists
- listen address belongs to LAN bridge
- upstream resolvers are valid
- firewall allows LAN clients to reach DNS service

---

### 14.3 Firewall Attachment

```rust
pub struct FirewallDesired {
    pub backend: String,
    pub tables: Vec<NftTableDesired>,
    pub profiles: Vec<FirewallProfileDesired>,
}
```

Validation:

- referenced LANs and WANs exist
- generated chains have deterministic names
- WLANd-owned tables do not clobber external tables
- management access rule exists if management UI/API is enabled

---

### 14.4 NAT Attachment

```rust
pub struct NatDesired {
    pub enabled: bool,
    pub source_lan: String,
    pub outbound_wan: String,
    pub masquerade: bool,
}
```

Validation:

- source LAN exists
- outbound WAN exists
- forwarding policy allows LAN to WAN
- masquerade is valid for the selected backend

---

## 15. Discovery Sources by Contract

| Contract | Primary Discovery Source | Secondary Source |
|---|---|---|
| EthernetPort | netlink | sysfs, ethtool |
| WirelessPhy | nl80211 | sysfs, iw-compatible data model |
| WlanInterface | netlink, nl80211 | hostapd state |
| Bridge | netlink | `/sys/class/net` |
| VLAN Interface | netlink | `/proc/net/vlan`, sysfs |
| SwitchChip | devlink, DSA sysfs | bridge VLAN state |
| SwitchPort | netlink, DSA sysfs | ethtool |
| Loopback | netlink | sysfs |
| WAN | derived from Ethernet/VLAN/modem state | routing table, DHCP lease |
| LAN | desired state plus bridge state | DHCP leases, ARP, FDB |
| Client | hostapd | DHCP leases, ARP, neighbor table, FDB |

---

## 16. Ownership Model

Every device must have an ownership state.

```rust
pub enum DeviceOwnership {
    Unmanaged,
    ObservedOnly,
    WlandManaged,
    ExternallyManaged,
    Conflicted,
}
```

### Unmanaged

WLANd ignores the device except for optional display.

### ObservedOnly

WLANd reports the device but does not mutate it.

### WlandManaged

WLANd owns configuration and may apply changes.

### ExternallyManaged

Another manager owns the device, such as NetworkManager or systemd-networkd.

### Conflicted

WLANd desired state references the device, but observed state suggests another manager is also mutating it.

---

## 17. Drift Model

Each device should support drift detection.

```rust
pub enum DriftSeverity {
    None,
    Info,
    Warning,
    Critical,
}

pub struct DriftEvent {
    pub device_id: String,
    pub device_kind: DeviceKind,
    pub severity: DriftSeverity,
    pub field: String,
    pub desired: Option<String>,
    pub observed: Option<String>,
    pub message: String,
}
```

Examples:

- desired bridge exists, observed bridge missing
- desired WLAN interface exists, observed interface missing
- desired PHY channel is 36, observed channel is 149
- desired DHCP service is enabled, observed service is down
- desired firewall table exists, observed table missing
- desired WAN has DHCP lease, observed lease missing
- desired BSS is running, observed hostapd reports down

---

## 18. Apply Plan Contract

No device contract should directly mutate the system during validation. Validation should produce either errors or an apply plan.

```rust
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

pub struct ApplyAction {
    pub id: String,
    pub kind: ApplyActionKind,
    pub target: String,
    pub description: String,
    pub dependencies: Vec<String>,
    pub rollback: Option<Box<ApplyAction>>,
    pub dangerous: bool,
}

pub struct ApplyPlan {
    pub id: String,
    pub created_at: String,
    pub actions: Vec<ApplyAction>,
    pub warnings: Vec<String>,
    pub requires_confirmation: bool,
}
```

### Apply Plan Rules

- validation happens before mutation
- dry-run uses the same plan as apply
- dangerous actions are marked explicitly
- rollback hooks are attached where practical
- management path preservation is checked before apply
- backend render artifacts are inspectable before apply

---

## 19. Minimum MVP Contract Set

For the first usable WLANd MVP, implement only these contracts:

1. `EthernetPort`
2. `WirelessPhy`
3. `WlanInterface`
4. `Bridge`
5. `WAN`
6. `LAN`
7. `DHCP Server Attachment`
8. `DNS Forwarder Attachment`
9. `Firewall Attachment`
10. `NAT Attachment`
11. `ClientObserved`

Defer these until after Simple Mode works:

- `SwitchChip`
- `SwitchPort`
- advanced VLAN trunking
- client policy enforcement
- multi-WAN
- modem/cellular WAN
- advanced roaming features

---

## 20. Recommended Rust Module Layout

```text
src/
  device/
    mod.rs
    identity.rs
    ownership.rs
    drift.rs
    apply.rs

  ethernet/
    contract.rs
    discover.rs
    validate.rs
    plan.rs

  phy/
    contract.rs
    discover.rs
    validate.rs
    plan.rs

  wlan/
    contract.rs
    discover.rs
    validate.rs
    plan.rs

  bridge/
    contract.rs
    discover.rs
    validate.rs
    plan.rs

  vlan/
    contract.rs
    discover.rs
    validate.rs
    plan.rs

  switch/
    contract.rs
    discover.rs
    validate.rs
    plan.rs

  lan/
    contract.rs
    validate.rs
    plan.rs

  wan/
    contract.rs
    validate.rs
    plan.rs

  services/
    dhcp.rs
    dns.rs
    firewall.rs
    nat.rs
```

---

## 21. Contract Design Principle

Hardware contracts should be conservative.

WLANd should only apply a configuration when all of these are true:

1. the device exists or WLANd can safely create it
2. the device is either WLANd-managed or explicitly adopted
3. requested capabilities are supported
4. desired state is internally consistent
5. rendered backend configuration is deterministic
6. management access is preserved
7. rollback is possible or the risk is explicitly accepted

