# Structure

wland should be the aggregate/root module. It should define the top-level desired state, observed state, runtime state, validation result, and apply plan. It should not contain all schemas forever. It should compose the domain modules.

For example:

src/wland
├── mod.rs
├── desired.rs
├── observed.rs
├── runtime.rs
├── plan.rs
├── validate.rs
├── http.rs
└── apply.rs

Then each domain module owns its own local schema and behavior.

For example:

src/wan
├── mod.rs
├── schema.rs
├── engine.rs
├── observe.rs
├── http.rs
└── apply.rs

Specific engines and integration must be kept within their appropriate domain. 

firewall/render.rs       // internally uses nftables
lan/bridge.rs            // internally uses rtnetlink
phy/observe.rs           // internally uses nl80211
wlan/interface.rs        // internally uses nl80211/rtnetlink
bss/hostapd.rs           // internally uses hostapd config/control socket

# Responsabilities
- wan: Upstream interface config, DHCP/static WAN behavior, default route observation.
- lan: Local bridge/subnet model, router address, DHCP intent, DNS intent.
- phy: **Physical** radio identity, country, channel, width, tx power, hardware capability observation.
- bss: SSID/security/client policy model. Logical advertised network.
- wlan: Concrete AP interface binding: phy + bss + lan + Linux interface.
- firewall: Opinionated zone policy, router-local service exposure, NAT intent, nftables rendering.
- dhcp: DHCP service config, rendered backend config, leases as runtime data.
- clients: Associated stations, DHCP leases, neighbor mappings, hostnames, counters.
- logs: Apply logs, validation logs, service logs, last failure.
- dbus: IPC surface. Should expose WLANd operations, not contain domain logic.
- wland: Aggregate controller: desired state, observed state, runtime state, validation, planning, apply.