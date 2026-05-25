pub mod schema;
pub mod cli;

use crate::rtnet::schema::{
    BridgeRequest, DeleteBridgeRequest, FetchRequest, InitRequest, MacAddress, NetdevChange,
    NetdevHandle, NetdevKind, OperState, ReloadRequest, RtnetDiff, RtnetInitResult,
    RtnetReloadResult, RtnetResult, RtnetState,
};
use anyhow::{Context, Result};
use futures_util::stream::TryStreamExt;
use rtnetlink::new_connection;
use std::collections::BTreeMap;
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Debug, Clone)]
pub struct RtnetlinkService {
    handle: rtnetlink::Handle,
    state: Arc<Mutex<RtnetState>>,
}

impl RtnetlinkService {
    pub async fn new() -> Result<Self> {
        let (connection, handle, _) = new_connection().context("failed to open rtnetlink connection")?;
        tokio::spawn(connection);
        Ok(Self {
            handle,
            state: Arc::new(Mutex::new(RtnetState { interfaces: Vec::new() })),
        })
    }

    pub async fn init(&self, _request: InitRequest) -> Result<RtnetInitResult> {
        let current = self.fetch(FetchRequest).await?.value;
        *self.state.lock().await = current.clone();
        Ok(RtnetInitResult { current })
    }

    pub async fn reload(&self, request: ReloadRequest) -> Result<RtnetReloadResult> {
        let current = self.fetch(FetchRequest).await?.value;
        let diff = diff_states(&request.previous, &current);
        *self.state.lock().await = current.clone();
        Ok(RtnetReloadResult { current, diff })
    }

    pub async fn fetch(&self, _request: FetchRequest) -> Result<RtnetResult<RtnetState>> {
        let mut links = self.handle.link().get().execute();
        let mut interfaces = Vec::new();

        while let Some(message) = links.try_next().await.context("failed to fetch links")? {
            interfaces.push(link_to_netdev(message));
        }

        Ok(RtnetResult {
            value: RtnetState { interfaces },
        })
    }

    pub async fn add_bridge(&self, request: BridgeRequest) -> Result<RtnetResult<NetdevHandle>> {
        self.handle
            .link()
            .add()
            .bridge(request.ifname.clone())
            .execute()
            .await
            .with_context(|| format!("failed to create bridge {}", request.ifname))?;

        let bridge = self
            .handle
            .link()
            .get()
            .match_name(request.ifname.clone())
            .execute()
            .try_next()
            .await
            .context("failed to resolve created bridge")?
            .context("created bridge not found")?;
        let mut handle = link_to_netdev(bridge);
        handle.admin_up = request.admin_up;
        Ok(RtnetResult { value: handle })
    }

    pub async fn delete_bridge(&self, request: DeleteBridgeRequest) -> Result<RtnetResult<()>> {
        let message = self
            .handle
            .link()
            .get()
            .match_name(request.ifname)
            .execute()
            .try_next()
            .await
            .context("failed to resolve bridge")?
            .context("bridge not found")?;
        self.handle
            .link()
            .del(message.header.index)
            .execute()
            .await
            .context("failed to delete bridge")?;
        Ok(RtnetResult { value: () })
    }
}

fn link_to_netdev(message: netlink_packet_route::link::LinkMessage) -> NetdevHandle {
    use netlink_packet_route::link::{LinkAttribute, LinkInfo, State};

    let ifindex = message.header.index as u32;
    let mut ifname = String::new();
    let mut kind = NetdevKind::Unknown;
    let admin_up = message.header.flags.contains(&netlink_packet_route::link::LinkFlag::Up);
    let mut oper_state = OperState::Unknown;
    let mut mtu = None;
    let mut mac = None;
    let mut master = None;

    for attribute in message.attributes {
        match attribute {
            LinkAttribute::IfName(name) => ifname = name,
            LinkAttribute::Mtu(value) => mtu = Some(value),
            LinkAttribute::Address(value) => mac = Some(MacAddress(format_mac(&value))),
            LinkAttribute::PermAddress(value) if mac.is_none() => {
                mac = Some(MacAddress(format_mac(&value)))
            }
            LinkAttribute::Controller(value) | LinkAttribute::Link(value) => master = Some(value),
            LinkAttribute::OperState(state) => {
                oper_state = match state {
                    State::Unknown => OperState::Unknown,
                    State::NotPresent => OperState::NotPresent,
                    State::Down => OperState::Down,
                    State::LowerLayerDown => OperState::LowerLayerDown,
                    State::Testing => OperState::Testing,
                    State::Dormant => OperState::Dormant,
                    State::Up => OperState::Up,
                    _ => OperState::Unknown,
                }
            }
            LinkAttribute::LinkInfo(items) => {
                for item in items {
                    if let LinkInfo::Kind(kind_value) = item {
                        kind = match kind_value.to_string().as_str() {
                            "bridge" => NetdevKind::Bridge,
                            "ether" | "ethernet" => NetdevKind::Ethernet,
                            _ => NetdevKind::Unknown,
                        };
                    }
                }
            }
            LinkAttribute::AfSpecBridge(_) => kind = NetdevKind::Bridge,
            LinkAttribute::Wireless(_) => kind = NetdevKind::Wlan,
            _ => {}
        }
    }

    NetdevHandle {
        ifindex,
        ifname,
        kind,
        admin_up,
        oper_state,
        mtu,
        mac,
        master,
    }
}

fn diff_states(previous: &RtnetState, current: &RtnetState) -> RtnetDiff {
    let previous_map: BTreeMap<_, _> = previous
        .interfaces
        .iter()
        .cloned()
        .map(|iface| (iface.ifindex, iface))
        .collect();
    let current_map: BTreeMap<_, _> = current
        .interfaces
        .iter()
        .cloned()
        .map(|iface| (iface.ifindex, iface))
        .collect();

    let added = current_map
        .values()
        .filter(|iface| !previous_map.contains_key(&iface.ifindex))
        .cloned()
        .collect();
    let removed = previous_map
        .values()
        .filter(|iface| !current_map.contains_key(&iface.ifindex))
        .cloned()
        .collect();
    let changed = current_map
        .iter()
        .filter_map(|(ifindex, after)| previous_map.get(ifindex).filter(|before| *before != after).map(|before| NetdevChange {
            before: before.clone(),
            after: after.clone(),
        }))
        .collect();

    RtnetDiff { added, removed, changed }
}

fn format_mac(bytes: &[u8]) -> String {
    bytes.iter().map(|byte| format!("{:02x}", byte)).collect::<Vec<_>>().join(":")
}
