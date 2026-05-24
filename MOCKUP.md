# MOCKUP.md

# WLANd Mockup

WLANd is a controller-style configuration layer for Linux-based WLAN routers and access points.

The core idea is to model Wi-Fi, LAN, WAN, DHCP, NAT, and firewall policy as explicit objects, then render those objects into the required Linux networking components.

The user should not need to manually write `hostapd`, `dnsmasq`, `nftables`, `ip`, or bridge configuration unless they want to.

---

## Goals

- Provide a controller-like interface for bring-your-own Linux WLAN hardware.
- Treat wireless radios as first-class hardware resources.
- Treat SSIDs/BSSs as logical network objects.
- Make LAN, WAN, DHCP, NAT, firewall, and forwarding policy explicit.
- Support simple home-router setup first.
- Support advanced multi-LAN, multi-SSID, multi-radio setups later.
- Avoid shell-command orchestration as the primary control model.
- Prefer programmatic control through netlink, nl80211, DBus, hostapd control sockets, and direct config rendering.

---

## Non-Goals

- Replace the Linux kernel networking stack.
- Replace `hostapd` as the 802.11 AP state machine.
- Replace `nftables` as the firewall/NAT backend.
- Replace every enterprise WLAN feature immediately.
- Hide all advanced networking concepts from advanced users.

---

## Core Concepts

### WAN

A WAN is an upstream routed network.

Usually this is an Ethernet interface connected to an ISP modem, upstream router, or another routed network.

A WAN may be configured with:

- DHCP
- Static IPv4
- Static IPv6
- NAT enabled or disabled
- Firewall profile
- Default route behavior

---

### LAN

A LAN is a local Layer 2 and Layer 3 network segment.

A LAN usually maps to:

- A Linux bridge, for example `br-lan0`
- An IPv4 subnet, for example `192.168.1.0/24`
- Optional DHCP service
- Optional DNS forwarding
- One or more attached BSSs
- Optional Ethernet member interfaces
- Optional VLAN ID

A LAN is not just an IP subnet. It is the logical local network that clients join.

---

### PHY

A PHY is a physical wireless radio exposed by Linux as `phy0`, `phy1`, etc.

A PHY has hardware capabilities such as:

- Supported bands
- Supported channels
- Supported channel widths
- Supported interface modes
- Maximum TX power
- Supported Wi-Fi standards
- DFS/radar requirements
- Number of supported AP interfaces

A PHY is hardware. It should be detected, not invented.

---

### BSS

A BSS represents one advertised wireless network instance.

A BSS has:

- SSID
- Authentication/encryption settings
- Key or secret reference
- LAN assignment
- Client policy
- Optional VLAN behavior

Important: if the same SSID is advertised on both 2.4 GHz and 5 GHz, that is usually modeled as two BSS instances with the same SSID, each bound to a different PHY or radio configuration.

---

### WLAN Interface

A WLAN interface is the concrete virtual AP interface created on a PHY.

Examples:

- `wlan0`
- `wlan0-ap0`
- `wlan0-ap1`

A WLAN object binds together:

- One PHY
- One BSS
- One LAN
- One generated or existing Linux interface

The WLAN object is the bridge between logical config and actual Linux runtime state.

---

---

## Packet Filtering and Forwarding Model

WLANd should model packet filtering in a way that maps cleanly to `nftables`.

Rather than treating forwarding as a flat list of abstract source/destination rules, WLANd should represent firewall policy using nftables-like objects:

- tables
- chains
- sets
- maps
- rules
- NAT rules
- chain hooks
- chain priorities
- default policies

The controller may still present a simplified policy UI, but the internal model should compile directly into nftables semantics.

---

## nftables Concepts

### Table

A table groups related chains.

WLANd should primarily use an `inet` table so IPv4 and IPv6 filtering can share the same ruleset where possible.

Example:

```nft
table inet wland_filter {
  chain input {
    type filter hook input priority filter; policy drop;
  }

  chain forward {
    type filter hook forward priority filter; policy drop;
  }

  chain output {
    type filter hook output priority filter; policy accept;
  }
}
```

---

## Procedure: Simple Mode

Simple mode should produce a usable home-router configuration with the fewest possible decisions.

1. Device boots with default configuration.
2. WLANd detects physical interfaces.
3. WLANd selects a WAN candidate.
    - Defaults to the first active Ethernet interface with a DHCP lease.
    - If multiple candidates exist, the user is prompted.
4. WLANd creates a default LAN.
    - Name: `lan0`
    - Bridge: `br-lan0`
    - Subnet: `192.168.1.0/24`
    - Router address: `192.168.1.1`
    - DHCP range: `192.168.1.100-192.168.1.250`
5. User enters basic Wi-Fi settings.
    - SSID
    - Passphrase
    - Security mode defaults to WPA2/WPA3 mixed if supported.
6. WLANd selects the best available radio configuration.
    - Prefer 5 GHz or 6 GHz when supported.
    - Fall back to 2.4 GHz.
    - If multiple radios are available, create multiple BSSs with the same SSID.
7. WLANd binds BSSs to the default LAN.
8. WLANd enables NAT from `lan0` to `wan0`.
9. WLANd applies default firewall behavior.
    - Allow established/related traffic.
    - Allow LAN to WAN.
    - Deny unsolicited WAN to LAN.
    - Allow DHCP/DNS from LAN to router.
    - Allow controller management from LAN.
10. WLANd renders and applies runtime configuration.

---

## Procedure: Advanced Mode

Advanced mode exposes the object model directly.

1. Device boots with default configuration.
2. Interfaces are detected.
    - Ethernet interfaces
    - Wireless PHYs
    - Existing WLAN interfaces
    - Bridges
    - VLANs
3. If an interface receives DHCP, runtime state is updated.
4. User reviews detected interfaces.
5. User assigns WAN roles.
    - Select Ethernet interface.
    - Configure DHCP or static addressing.
    - Configure NAT behavior.
    - Configure WAN firewall profile.
6. User reviews physical radios.
    - Supported bands
    - Supported channels
    - Supported widths
    - DFS requirements
    - AP mode support
    - Multi-BSS support
7. User configures PHY policy.
    - Band selection
    - Channel
    - Channel width
    - TX power
    - Country code
    - Steering preference
8. User creates LANs.
    - Name
    - Description
    - Bridge name
    - Subnet
    - Router address
    - DHCP range
    - DNS behavior
    - Optional VLAN ID
9. User reviews forwarding table.
    - LAN to WAN
    - LAN to LAN
    - NAT behavior
    - Guest isolation
    - Inter-VLAN routing
10. User creates BSS objects.
    - SSID
    - Security mode
    - Secret reference
    - LAN assignment
    - Client isolation
    - MAC filtering
11. User creates WLAN bindings.
    - Bind PHY to BSS.
    - Bind BSS to LAN.
    - Generate or select Linux WLAN interface.
12. WLANd validates the full config.
13. WLANd renders backend configuration.
14. WLANd applies changes transactionally.
15. WLANd updates observed runtime state.

---

## Default Firewall Policy

Default simple-mode firewall behavior:

| Direction | Action | Notes |
|---|---:|---|
| LAN -> WAN | Allow | NAT enabled by default |
| WAN -> LAN | Deny | No unsolicited inbound traffic |
| WAN -> Router | Deny | Except explicit management rules |
| LAN -> Router DHCP | Allow | UDP 67/68 |
| LAN -> Router DNS | Allow | TCP/UDP 53 |
| LAN -> Router UI/API | Allow | Controller management |
| Guest LAN -> Main LAN | Deny | If guest LAN exists |
| Established/Related | Allow | Stateful return traffic |
| Invalid | Drop | Basic hygiene |

---

## Desired State vs Observed State

WLANd should separate desired configuration from observed runtime state.

Desired state answers:

> What should the network look like?

Observed state answers:

> What is actually happening right now?

This allows the controller to detect drift.

Examples of drift:

- WAN interface lost DHCP lease.
- WLAN interface failed to start.
- PHY does not support requested channel width.
- DHCP server failed.
- Firewall table was modified externally.
- Client is associated but did not receive DHCP.

---

## WLANd Desired State Object

```json
{
  "version": 1,
  "metadata": {
    "name": "default-router",
    "description": "Default WLANd router configuration"
  },
  "wan": {
    "wan0": {
      "description": "Primary upstream WAN",
      "interface": {
        "name": "enp2s0",
        "type": "ethernet",
        "mac": "aa:bb:cc:dd:ee:ff"
      },
      "ipv4": {
        "method": "dhcp"
      },
      "ipv6": {
        "method": "disabled"
      },
      "nat": {
        "enabled": true,
        "masquerade": true
      },
      "firewall_profile": "wan-default"
    }
  },
  "lan": {
    "lan0": {
      "name": "devices",
      "description": "Primary home devices network",
      "bridge": {
        "name": "br-lan0",
        "stp": false
      },
      "ipv4": {
        "address": "192.168.1.1",
        "prefix": 24,
        "network": "192.168.1.0"
      },
      "dhcp": {
        "enabled": true,
        "range_start": "192.168.1.100",
        "range_end": "192.168.1.250",
        "lease_time": "12h",
        "dns_servers": ["192.168.1.1"],
        "gateway": "192.168.1.1"
      },
      "dns": {
        "enabled": true,
        "upstream": ["1.1.1.1", "8.8.8.8"]
      }
    },
    "lan1": {
      "name": "guest",
      "description": "Guest Wi-Fi network",
      "bridge": {
        "name": "br-guest0",
        "stp": false
      },
      "ipv4": {
        "address": "192.168.20.1",
        "prefix": 24,
        "network": "192.168.20.0"
      },
      "dhcp": {
        "enabled": true,
        "range_start": "192.168.20.100",
        "range_end": "192.168.20.250",
        "lease_time": "4h",
        "dns_servers": ["192.168.20.1"],
        "gateway": "192.168.20.1"
      },
      "dns": {
        "enabled": true,
        "upstream": ["1.1.1.1", "8.8.8.8"]
      }
    }
  },
  "phy": {
    "phy0": {
      "description": "Primary 5 GHz radio",
      "country": "US",
      "band": "5GHz",
      "channel": 36,
      "width": "80MHz",
      "txpower_dbm": 20,
      "hw_mode": "802.11ax",
      "channel_policy": "fixed"
    },
    "phy1": {
      "description": "Fallback 2.4 GHz radio",
      "country": "US",
      "band": "2.4GHz",
      "channel": 6,
      "width": "20MHz",
      "txpower_dbm": 20,
      "hw_mode": "802.11n",
      "channel_policy": "fixed"
    }
  },
  "bss": {
    "bss0": {
      "ssid": "HomeNetwork",
      "description": "Primary 5 GHz home SSID",
      "security": {
        "mode": "wpa2-wpa3-personal",
        "secret_ref": "secret://wifi/home"
      },
      "lan_assignment": "lan0",
      "policy": {
        "client_isolation": false,
        "mac_filter_mode": "none"
      }
    },
    "bss1": {
      "ssid": "HomeNetwork",
      "description": "Primary 2.4 GHz home SSID",
      "security": {
        "mode": "wpa2-wpa3-personal",
        "secret_ref": "secret://wifi/home"
      },
      "lan_assignment": "lan0",
      "policy": {
        "client_isolation": false,
        "mac_filter_mode": "none"
      }
    },
    "bss2": {
      "ssid": "HomeNetwork_Guest",
      "description": "Guest Wi-Fi SSID",
      "security": {
        "mode": "wpa2-personal",
        "secret_ref": "secret://wifi/guest"
      },
      "lan_assignment": "lan1",
      "policy": {
        "client_isolation": true,
        "mac_filter_mode": "none"
      }
    }
  },
  "wlan": {
    "wlan0": {
      "description": "5 GHz primary AP interface",
      "phy_binding": "phy0",
      "bss_binding": "bss0",
      "lan_binding": "lan0",
      "interface": {
        "name": "wlan0",
        "mode": "ap"
      }
    },
    "wlan1": {
      "description": "2.4 GHz primary AP interface",
      "phy_binding": "phy1",
      "bss_binding": "bss1",
      "lan_binding": "lan0",
      "interface": {
        "name": "wlan1",
        "mode": "ap"
      }
    },
    "wlan2": {
      "description": "Guest AP interface on 5 GHz radio",
      "phy_binding": "phy0",
      "bss_binding": "bss2",
      "lan_binding": "lan1",
      "interface": {
        "name": "wlan0-guest",
        "mode": "ap"
      }
    }
  },
  "forwarding": [
    {
      "rule_id": 1,
      "description": "Allow primary LAN to reach WAN with NAT",
      "source": "lan0",
      "destination": "wan0",
      "nat": true,
      "action": "accept"
    },
    {
      "rule_id": 2,
      "description": "Allow guest LAN to reach WAN with NAT",
      "source": "lan1",
      "destination": "wan0",
      "nat": true,
      "action": "accept"
    },
    {
      "rule_id": 3,
      "description": "Block guest LAN from primary LAN",
      "source": "lan1",
      "destination": "lan0",
      "nat": false,
      "action": "drop"
    }
  ],
  "management": {
    "ui": {
      "enabled": true,
      "listen": "lan",
      "port": 8443
    },
    "api": {
      "enabled": true,
      "listen": "lan",
      "port": 9443
    },
    "ssh": {
      "enabled": false
    }
  }
}

{
  "timestamp": "2026-05-24T00:00:00Z",
  "interfaces": {
    "enp2s0": {
      "operstate": "up",
      "carrier": true,
      "mac": "aa:bb:cc:dd:ee:ff",
      "ipv4": ["203.0.113.10/24"],
      "default_route": true
    },
    "br-lan0": {
      "operstate": "up",
      "ipv4": ["192.168.1.1/24"],
      "members": ["wlan0", "wlan1"]
    },
    "br-guest0": {
      "operstate": "up",
      "ipv4": ["192.168.20.1/24"],
      "members": ["wlan0-guest"]
    }
  },
  "phy": {
    "phy0": {
      "present": true,
      "driver": "ath12k",
      "bands": ["2.4GHz", "5GHz", "6GHz"],
      "supports_ap": true,
      "supports_multi_bss": true
    },
    "phy1": {
      "present": true,
      "driver": "iwlwifi",
      "bands": ["2.4GHz", "5GHz"],
      "supports_ap": true,
      "supports_multi_bss": false
    }
  },
  "bss": {
    "bss0": {
      "state": "running",
      "ssid": "HomeNetwork",
      "clients": 8
    },
    "bss1": {
      "state": "running",
      "ssid": "HomeNetwork",
      "clients": 3
    },
    "bss2": {
      "state": "running",
      "ssid": "HomeNetwork_Guest",
      "clients": 2
    }
  },
  "services": {
    "dhcp": {
      "state": "running",
      "leases": 13
    },
    "dns": {
      "state": "running"
    },
    "firewall": {
      "state": "applied",
      "backend": "nftables"
    },
    "nat": {
      "state": "enabled"
    }
  }
}