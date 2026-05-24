Backend Rendering

WLANd should compile the desired state into backend-specific configuration.

Linux Interfaces

Responsible for:

Creating bridges
Assigning IP addresses
Creating AP-mode virtual interfaces
Attaching WLAN interfaces to LAN bridges
Applying MTU where needed
Bringing interfaces up/down

Preferred control path:

Netlink
nl80211
systemd-networkd backend, optional
NetworkManager backend, optional
hostapd

Responsible for:

AP beaconing
Authentication
WPA/WPA2/WPA3 handling
Client association
Per-BSS wireless behavior
Runtime station information

WLANd should not replace hostapd.

WLANd should generate hostapd configuration and use the hostapd control socket for runtime operations.

Examples of runtime operations:

Show connected clients
Disconnect client
Reload BSS
Enable/disable BSS
Read station statistics
DHCP/DNS

Possible backends:

dnsmasq
Kea DHCP
systemd-networkd DHCP server

Initial MVP backend should probably be dnsmasq because it can provide DHCP and simple DNS forwarding with minimal complexity.

Firewall/NAT

Preferred backend:

nftables

Generated policy should include:

Input chain
Forward chain
NAT postrouting chain
LAN management allows
WAN default-deny rules
LAN-to-WAN forwarding
Optional inter-LAN forwarding
Validation Rules

Before applying configuration, WLANd should validate that:

Every WAN references an existing Ethernet interface.
Every LAN has a valid subnet.
No LAN subnets overlap.
DHCP ranges are inside the LAN subnet.
DHCP ranges do not include the router address.
Every BSS references an existing LAN.
Every WLAN references an existing PHY.
Every WLAN references an existing BSS.
PHY supports requested AP mode.
PHY supports requested band.
PHY supports requested channel.
PHY supports requested channel width.
Multiple BSSs on one PHY are allowed by hardware and driver.
NAT rules reference valid LAN/WAN objects.
Guest isolation does not conflict with explicit forwarding rules.
Secrets referenced by BSS objects exist.
Country code is set before enabling AP mode.
Transaction Model

Applying config should be transactional where possible.

Suggested flow:

Load desired state.
Validate schema.
Validate against detected hardware.
Build dependency graph.
Generate backend configs.
Stage backend configs.
Apply low-risk interface changes.
Start or reload services.
Verify runtime state.
Commit config as active.
Roll back if critical services fail.

Rollback should preserve local access whenever possible.

For example, WLANd should avoid applying a firewall rule that locks the user out of the management interface.

Secret Handling

BSS objects should not store raw passphrases directly in the main config.

Instead, they should reference secrets.

Example:
```json
{
  "security": {
    "mode": "wpa2-wpa3-personal",
    "secret_ref": "secret://wifi/home"
  }
}
```
Possible secret backends:

Local encrypted file
Linux keyring
TPM-backed store
Plain local file for development only

For MVP, a local root-only secret file is acceptable.

## Client Model

WLANd should expose connected clients as first-class runtime objects.

Example:
```json
{
  "clients": {
    "client0": {
      "mac": "de:ad:be:ef:00:01",
      "hostname": "laptop",
      "ip": "192.168.1.120",
      "bss": "bss0",
      "wlan": "wlan0",
      "lan": "lan0",
      "signal_dbm": -48,
      "rx_bytes": 123456789,
      "tx_bytes": 987654321,
      "connected_for": "2h13m"
    }
  }
}
```

Client data may come from:

hostapd station table
DHCP lease table
ARP/neighbor table
Optional mDNS/hostname discovery
## Example CLI
```bash
wland status

wland interfaces list
wland phy list
wland phy show phy0

wland lan create lan0 \
  --name devices \
  --subnet 192.168.1.0/24 \
  --dhcp true

wland wan set wan0 \
  --interface enp2s0 \
  --method dhcp \
  --nat true

wland bss create bss0 \
  --ssid HomeNetwork \
  --security wpa2-wpa3-personal \
  --secret-ref secret://wifi/home \
  --lan lan0

wland wlan create wlan0 \
  --phy phy0 \
  --bss bss0

wland forwarding allow lan0 wan0 --nat true

wland apply

```

## Example API Shape
```http
GET /api/v1/status
GET /api/v1/interfaces
GET /api/v1/phy
GET /api/v1/clients

GET /api/v1/config
PUT /api/v1/config

POST /api/v1/lan
POST /api/v1/wan
POST /api/v1/bss
POST /api/v1/wlan
POST /api/v1/forwarding

POST /api/v1/apply
POST /api/v1/rollback
```
## MVP Scope

The first useful version should support:

One WAN
One LAN
One DHCP scope
One or more PHYs
One SSID across one or more radios
WPA2/WPA3 personal
NAT from LAN to WAN
Default deny inbound firewall
Connected client list
Basic web UI
Basic API
Config validation
Apply/rollback safety
Later Features

Potential future features:

Multiple LANs
Guest networks
VLAN tagging
WPA Enterprise
802.11r fast roaming
Band steering
Client steering
Mesh/backhaul support
Captive portal
Per-client bandwidth limits
Per-SSID firewall policy
Scheduled SSID availability
Multi-WAN
IPv6 prefix delegation
Dynamic DNS
VPN integration
Prometheus metrics
OpenTelemetry traces
Controller-agent split for multiple APs
Design Principle

