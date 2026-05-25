use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NetdevKind {
    Bridge,
    Ethernet,
    Wlan,
    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OperState {
    Unknown,
    NotPresent,
    Down,
    LowerLayerDown,
    Testing,
    Dormant,
    Up,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MacAddress(pub String);

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Ipv4Address {
    pub address: String,
    pub prefix: u8,
    pub network: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NetdevHandle {
    pub ifindex: u32,
    pub ifname: String,
    pub kind: NetdevKind,
    pub admin_up: bool,
    pub oper_state: OperState,
    pub mtu: Option<u32>,
    pub mac: Option<MacAddress>,
    pub ipv4: Option<Ipv4Address>,
    pub master: Option<u32>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RtnetState {
    pub interfaces: Vec<NetdevHandle>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RtnetDiff {
    pub added: Vec<NetdevHandle>,
    pub removed: Vec<NetdevHandle>,
    pub changed: Vec<NetdevChange>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NetdevChange {
    pub before: NetdevHandle,
    pub after: NetdevHandle,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RtnetResult<T> {
    pub value: T,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BridgeRequest {
    pub ifname: String,
    pub admin_up: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DeleteBridgeRequest {
    pub ifname: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FetchRequest;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InitRequest;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReloadRequest {
    pub previous: RtnetState,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RtnetInitResult {
    pub current: RtnetState,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RtnetReloadResult {
    pub current: RtnetState,
    pub diff: RtnetDiff,
}
