# WLANd Project Milestones

WLANd is a controller-style configuration layer for Linux-based WLAN routers and access points. The project should be built backend-first: object model, validation, rendering, safe apply, observed state, and reconciliation before a full UI.

The core product is a system that turns explicit desired-state objects for WAN, LAN, PHY, BSS, WLAN bindings, DHCP, NAT, firewall, and management policy into working Linux runtime state.

---

## Milestone Overview

| Milestone | Goal | Deliverable | Acceptance Criteria |
|---|---|---|---|
| 0. Project foundation | Define the daemon shape and internal contracts. | `wland` daemon skeleton, config schema module, error model, logging, and test harness. | Daemon starts, loads config, validates top-level schema, emits structured logs, and exposes a basic health endpoint or CLI command. |
| 1. Desired-state model | Implement the object model before touching the system. | Rust structs/schemas for WAN, LAN, PHY policy, BSS, WLAN binding, DHCP/DNS, NAT, firewall, and management. | Example desired-state JSON can be parsed, validated, serialized, and round-tripped without losing data. |
| 2. Hardware/runtime discovery | Detect what Linux already exposes. | Discovery layer for Ethernet interfaces, wireless PHYs, WLAN interfaces, bridges, addresses, routes, and basic driver/capability info. | `wland inspect` or API equivalent shows WAN candidates, PHYs, supported bands/modes, existing bridges, IPs, and default route state. |
| 3. Validation engine | Prevent impossible configs before applying anything. | Constraint checker for subnet conflicts, missing PHYs, unsupported AP mode, invalid BSS/LAN bindings, bad DHCP ranges, and invalid channel/band combinations. | Invalid configs fail with useful messages. Valid simple-router configs pass without needing backend apply code. |
| 4. Backend rendering | Generate Linux backend configuration from the model. | Renderers for hostapd config, DHCP/DNS config, nftables ruleset, and bridge/IP apply plan. | Given a desired-state object, WLANd produces deterministic backend artifacts and a readable apply plan. |
| 5. Safe apply engine | Apply network state without unnecessarily bricking the host. | Apply planner with ordered operations, dry-run mode, checkpointing, rollback hooks, and management-access guardrails. | `wland apply --dry-run` shows planned changes. `wland apply` can create a bridge, assign LAN IP, configure DHCP, apply nftables, and start services on a lab box. |
| 6. Simple Mode MVP | Produce the first usable home-router flow. | Simple-mode config generator: pick WAN, create `lan0`, create `br-lan0`, configure DHCP, DNS forwarding, NAT, firewall, and one SSID. | Fresh install can become a working NAT router/AP with minimal inputs: WAN interface, SSID, passphrase, and country code. Client joins Wi-Fi, gets DHCP, resolves DNS, and reaches the internet. |
| 7. hostapd control integration | Stop treating Wi-Fi as only static config. | hostapd control-socket client for status, connected clients, reload/reconfigure where safe, and later deauth/kick controls. | WLANd reports BSS state and associated clients without parsing random CLI output. |
| 8. Observed state and drift detection | Separate “what should exist” from “what exists.” | Observed-state collector for interfaces, bridges, PHYs, BSS state, clients, DHCP leases, DNS state, firewall state, and NAT state. | WLANd reports drift such as missing bridge, failed DHCP service, hostapd down, firewall table modified, WAN lease lost, or BSS down. |
| 9. Reconciliation loop | Move from one-shot config tool to controller. | Daemon loop that periodically compares desired state to observed state and remediates safe drift. | If hostapd, DHCP, or nftables state disappears, WLANd detects and restores it according to policy. Dangerous topology changes require explicit apply. |
| 10. API layer | Make backend capabilities accessible cleanly. | HTTP API or DBus API for config CRUD, apply, inspect, observed state, clients, logs, and validation. | UI and CLI use the same API. No direct UI coupling to hostapd, nftables, DHCP, or DNS backend details. |
| 11. CLI | Provide the admin/debug interface first. | `wland inspect`, `wland validate`, `wland plan`, `wland apply`, `wland status`, `wland clients`, and `wland logs`. | The whole product can be run and debugged headlessly over SSH before building a full frontend. |
| 12. Minimal UI | Build the controller surface after backend behavior works. | Web UI for Simple Mode setup, current clients, WAN/LAN/WLAN status, logs, and apply-plan review. | A normal user can configure the router without editing config files. Advanced users can inspect generated backend state. |
| 13. Advanced Mode v1 | Expose the real object model. | UI/API support for multiple LANs, guest networks, multiple BSSs with the same SSID, multi-radio bindings, VLAN IDs, NAT toggles, and firewall profiles. | Can configure primary LAN plus guest LAN, same SSID across two radios, guest isolation, and LAN-to-WAN NAT. |
| 14. Firewall model v1 | Treat nftables as a first-class backend. | Internal tables/chains/sets/maps/rules model that compiles to nftables, plus simplified presets. | Default simple-mode firewall allows LAN → WAN, denies unsolicited WAN → LAN, allows LAN DHCP/DNS/UI, allows established/related, and drops invalid traffic. |
| 15. Packaging and appliance mode | Make it installable and recoverable. | NixOS module first, then systemd unit files, config directories, state directories, backup/restore, and reset-to-default. | Can install on a router host declaratively. A bad config can be rolled back from local console or recovery command. |
| 16. Hardening and test lab | Make the system boring enough to trust. | Integration tests with network namespaces/veth, mocked PHY data, nftables validation, config migration tests, and failure-mode tests. | CI proves generated nftables parses, DHCP ranges are valid, apply order is stable, and common bad configs are rejected. |

---

## Critical Path

```text
schema
  -> discovery
  -> validation
  -> render plan
  -> dry-run
  -> apply
  -> simple router
  -> observed state
  -> reconciliation
  -> API/UI
```

The first real product boundary is **Milestone 6: Simple Mode MVP**. At that point, WLANd can turn a Linux box into a working home router/AP using the desired-state model.

---

## Release Plan

### Version 0.1: CLI-only Simple Mode

Scope:

- One WAN
- One LAN
- One bridge
- One SSID
- DHCP
- DNS forwarding
- NAT
- Default firewall
- CLI commands for validate, plan, apply, inspect, and status

Goal:

> A fresh Linux host can become a working router/AP without manually writing `hostapd`, `dnsmasq`, `nftables`, `ip`, or bridge configuration.

---

### Version 0.2: Observed State and Client Visibility

Scope:

- WAN state
- LAN bridge state
- DHCP lease visibility
- Associated Wi-Fi clients
- BSS status
- Firewall/NAT status
- Basic drift detection

Goal:

> WLANd can show what is actually happening, not just what was configured.

---

### Version 0.3: API and Minimal UI

Scope:

- Simple Mode setup wizard
- Status dashboard
- Clients page
- Logs page
- Apply-plan review
- API shared by CLI and UI

Goal:

> A user can configure and inspect the router through a controller-like interface.

---

### Version 0.4: Guest Network

Scope:

- Second LAN
- Second BSS
- Guest DHCP
- Guest NAT
- Guest-to-main isolation
- Firewall presets for guest behavior

Goal:

> WLANd supports the most important multi-network home-router use case.

---

### Version 0.5: Advanced Object Editor

Scope:

- Multi-radio configuration
- Multi-BSS configuration
- Explicit LAN/WLAN bindings
- VLAN IDs
- Configurable firewall profiles
- Advanced NAT/forwarding controls

Goal:

> Advanced users can directly model more complex WLAN/router topologies.

---

## Major Technical Risks

### Safe apply and rollback

Bad network configuration can cut off SSH or the management UI. WLANd needs dry-run, apply plans, checkpoints, rollback hooks, and management-access guardrails early.

### hostapd lifecycle management

WLANd should not replace hostapd, but it needs to own enough of hostapd configuration and runtime state to feel like a controller.

### nftables ownership

WLANd should avoid clobbering unrelated firewall rules. It should own its own tables/chains and clearly separate generated policy from user-managed policy.

### NetworkManager and systemd-networkd coexistence

The project needs a clear stance on whether it owns interfaces directly or coexists with other network managers. Appliance mode should eventually prefer explicit ownership.

### Driver and PHY variability

Different wireless drivers expose different AP capabilities, multi-BSS behavior, DFS restrictions, channel support, and 6 GHz constraints. The validation layer must account for this.

### State drift

External tools may mutate bridges, interfaces, routes, hostapd state, DHCP state, or firewall rules. Observed state and reconciliation should be built as core features, not later polish.

---

## Recommended Build Order

1. Build the schema and object model.
2. Build validation against static desired state.
3. Build runtime discovery.
4. Build deterministic renderers.
5. Build dry-run apply plans.
6. Build a safe apply engine.
7. Ship CLI-only Simple Mode.
8. Add observed state.
9. Add drift detection.
10. Add reconciliation.
11. Add API.
12. Add minimal UI.
13. Add guest network support.
14. Add advanced object editing.
15. Add NixOS packaging and appliance-mode hardening.

---

## Deferred Features

These are valid, but should not block the first MVP:

- Band steering
- Fast roaming
- WPA Enterprise
- Captive portal
- Mesh support
- 802.11k/v/r
- Per-client rate limits
- Deep traffic analytics
- Multi-WAN failover
- Policy-based routing
- Full VLAN trunk management
- Cloud controller mode
- Mobile app
- Automatic channel optimization

---

## Product Principle

WLANd should not be a pile of shell commands behind a web UI.

It should be a controller with:

- explicit desired state
- observed runtime state
- validation
- deterministic rendering
- safe apply
- drift detection
- reconciliation
- usable Simple Mode
- direct Advanced Mode
