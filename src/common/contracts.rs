#![allow(deprecated)]

#[deprecated(note = "Move these contracts into the owning domain schemas.")]
pub use crate::clients::schema::{ClientIdentity, ClientObserved, ClientPolicyDesired, WifiClientObserved};
#[deprecated(note = "Move these contracts into the owning domain schemas.")]
pub use crate::dhcp::schema::{DhcpLeaseObserved, DhcpServerDesired};
#[deprecated(note = "Move these contracts into the owning domain schemas.")]
pub use crate::firewall::schema::{FirewallDesired, FirewallProfileDesired, NatDesired, NftTableDesired};
#[deprecated(note = "Move these contracts into the owning domain schemas.")]
pub use crate::lan::schema::{BridgeCapabilities, BridgeDesired, BridgeFdbEntry, BridgeIdentity, BridgeObserved, DhcpServerDesired as LanDhcpServerDesired, DnsForwarderDesired, Ipv4InterfaceDesired as LanIpv4InterfaceDesired, Ipv6InterfaceDesired as LanIpv6InterfaceDesired, LanDesired};
#[deprecated(note = "Move these contracts into the owning domain schemas.")]
pub use crate::phy::schema::{ChannelCapability, ChannelPolicy, InterfaceMode, WifiBand, WirelessPhyCapabilities, WirelessPhyDesired, WirelessPhyIdentity, WirelessPhyObserved};
#[deprecated(note = "Move these contracts into the owning domain schemas.")]
pub use crate::wan::schema::{Ipv4InterfaceDesired as WanIpv4InterfaceDesired, Ipv6InterfaceDesired as WanIpv6InterfaceDesired, NatDesired as WanNatDesired, WanAddressMethod, WanDesired};
#[deprecated(note = "Move these contracts into the owning domain schemas.")]
pub use crate::wlan::schema::{DeviceObservedBase, WlanInterfaceCapabilities, WlanInterfaceDesired, WlanInterfaceIdentity, WlanInterfaceObserved, WlanRuntimeMode};
