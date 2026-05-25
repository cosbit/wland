use crate::bss::schema::{BssConfig, BssObserved};
use crate::clients::schema::ClientsRuntime;
use crate::common::errcodes::ErrorCode;
use crate::common::exceptions::WlandException;
use crate::dhcp::schema::DhcpRuntime;
use crate::firewall::schema::{FirewallConfig, FirewallObserved};
use crate::lan::schema::LanConfig;
use crate::logs::schema::LogsState;
use crate::phy::schema::{PhyConfig, PhyObserved};
use crate::wan::schema::WanConfig;
use crate::wlan::schema::{WlanConfig, WlanObserved};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WlandMetadata {
    pub name: String,
    pub description: Option<String>,
    pub version: Option<String>,
    pub labels: BTreeMap<String, String>,
    pub annotations: BTreeMap<String, String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WlandSecretRef {
    pub uri: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WlandApplyPolicy {
    pub rollback: bool,
    pub lockout_protection: bool,
    pub validation_required: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WlandDesiredState {
    pub version: u32,
    pub metadata: WlandMetadata,
    pub wan: BTreeMap<String, WanConfig>,
    pub lan: BTreeMap<String, LanConfig>,
    pub phy: BTreeMap<String, PhyConfig>,
    pub bss: BTreeMap<String, BssConfig>,
    pub wlan: BTreeMap<String, WlanConfig>,
    pub firewall: FirewallConfig,
    pub management: WlandManagement,
    pub secrets: BTreeMap<String, WlandSecretRef>,
    pub apply_policy: WlandApplyPolicy,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WlandInterfaceState {
    pub operstate: String,
    pub carrier: bool,
    pub mac: Option<String>,
    pub ipv4: Vec<String>,
    pub default_route: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WlandRouteState {
    pub destination: Option<String>,
    pub gateway: Option<String>,
    pub interface: Option<String>,
    pub default_route: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WlandServicesState {
    pub dhcp: Option<String>,
    pub dns: Option<String>,
    pub firewall: Option<String>,
    pub nat: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WlandObservedState {
    pub timestamp: Option<String>,
    pub interfaces: BTreeMap<String, WlandInterfaceState>,
    pub routes: BTreeMap<String, WlandRouteState>,
    pub phy: BTreeMap<String, PhyObserved>,
    pub wlan: BTreeMap<String, WlanObserved>,
    pub bss: BTreeMap<String, BssObserved>,
    pub services: WlandServicesState,
    pub firewall: FirewallObserved,
    pub logs: LogsState,
    pub drift: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WlandRuntimeState {
    pub timestamp: Option<String>,
    pub clients: ClientsRuntime,
    pub dhcp: DhcpRuntime,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WlandValidationIssue {
    pub code: ErrorCode,
    pub message: String,
    pub scope: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WlandValidationResult {
    pub valid: bool,
    pub timestamp: Option<String>,
    pub issues: Vec<WlandValidationIssue>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WlandApplyStep {
    pub name: String,
    pub status: String,
    pub message: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WlandApplyPlan {
    pub timestamp: Option<String>,
    pub status: String,
    pub steps: Vec<WlandApplyStep>,
    pub last_error: Option<WlandException>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WlandManagementEndpoint {
    pub enabled: bool,
    pub listen: String,
    pub port: u16,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WlandManagementSsh {
    pub enabled: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WlandManagement {
    pub ui: WlandManagementEndpoint,
    pub api: WlandManagementEndpoint,
    pub ssh: WlandManagementSsh,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WlandState {
    pub desired: WlandDesiredState,
    pub observed: WlandObservedState,
    pub runtime: WlandRuntimeState,
    pub validation: Option<WlandValidationResult>,
    pub apply: Option<WlandApplyPlan>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bss::schema::{BssConfig, BssPolicy, BssSecurity};
    use crate::clients::schema::{ClientLease, ClientStation, ClientsRuntime};
    use crate::common::errcodes::ErrorCode;
    use crate::dhcp::schema::DhcpRuntime;
    use crate::firewall::schema::{
        FirewallApplyRecord, FirewallConfig, FirewallCustomRules, FirewallObserved,
        FirewallPortForward, FirewallZone,
    };
    use crate::lan::schema::{LanBridge, LanConfig, LanDhcp, LanDns, LanIpv4};
    use crate::logs::schema::{LogEntry, LogsState};
    use crate::phy::schema::{PhyConfig, PhyObserved};
    use crate::wan::schema::{WanConfig, WanInterface, WanIpConfig, WanNat};
    use crate::wlan::schema::{WlanConfig, WlanInterface, WlanObserved};
    use serde_json::Value;
    use std::collections::BTreeMap;
    use std::net::{IpAddr, Ipv4Addr};

    fn sample_state() -> WlandState {
        let mut wan = BTreeMap::new();
        wan.insert(
            "wan0".to_string(),
            WanConfig {
                description: Some("Primary upstream WAN".to_string()),
                interface: WanInterface {
                    name: "enp2s0".to_string(),
                    interface_type: "ethernet".to_string(),
                    mac: Some("aa:bb:cc:dd:ee:ff".to_string()),
                },
                ipv4: WanIpConfig {
                    method: "dhcp".to_string(),
                    address: None,
                },
                ipv6: WanIpConfig {
                    method: "disabled".to_string(),
                    address: None,
                },
                nat: WanNat {
                    enabled: true,
                    masquerade: true,
                },
                firewall_profile: Some("wan-default".to_string()),
            },
        );

        let mut lan = BTreeMap::new();
        lan.insert(
            "lan0".to_string(),
            LanConfig {
                name: "devices".to_string(),
                description: Some("Primary home devices network".to_string()),
                bridge: LanBridge {
                    name: "br-lan0".to_string(),
                    stp: false,
                },
                ipv4: LanIpv4 {
                    address: Ipv4Addr::new(192, 168, 1, 1),
                    prefix: 24,
                    network: Ipv4Addr::new(192, 168, 1, 0),
                },
                dhcp: LanDhcp {
                    enabled: true,
                    range_start: Ipv4Addr::new(192, 168, 1, 100),
                    range_end: Ipv4Addr::new(192, 168, 1, 250),
                    lease_time: "12h".to_string(),
                    dns_servers: vec![IpAddr::V4(Ipv4Addr::new(192, 168, 1, 1))],
                    gateway: Ipv4Addr::new(192, 168, 1, 1),
                },
                dns: LanDns {
                    enabled: true,
                    upstream: vec![
                        IpAddr::V4(Ipv4Addr::new(1, 1, 1, 1)),
                        IpAddr::V4(Ipv4Addr::new(8, 8, 8, 8)),
                    ],
                },
            },
        );

        let mut phy = BTreeMap::new();
        phy.insert(
            "phy0".to_string(),
            PhyConfig {
                description: Some("Primary 5 GHz radio".to_string()),
                country: "US".to_string(),
                band: "5GHz".to_string(),
                channel: 36,
                width: "80MHz".to_string(),
                txpower_dbm: 20,
                hw_mode: "802.11ax".to_string(),
                channel_policy: "fixed".to_string(),
            },
        );

        let mut bss = BTreeMap::new();
        bss.insert(
            "bss0".to_string(),
            BssConfig {
                ssid: "HomeNetwork".to_string(),
                description: Some("Primary home SSID".to_string()),
                security: BssSecurity {
                    mode: "wpa2-wpa3-personal".to_string(),
                    secret_ref: "secret://wifi/home".to_string(),
                },
                lan_assignment: "lan0".to_string(),
                policy: BssPolicy {
                    client_isolation: false,
                    mac_filter_mode: "none".to_string(),
                },
            },
        );

        let mut wlan = BTreeMap::new();
        wlan.insert(
            "wlan0".to_string(),
            WlanConfig {
                description: Some("AP interface".to_string()),
                phy_binding: "phy0".to_string(),
                bss_binding: "bss0".to_string(),
                lan_binding: "lan0".to_string(),
                interface: WlanInterface {
                    name: "wlan0".to_string(),
                    mode: "ap".to_string(),
                },
            },
        );

        let mut zones = BTreeMap::new();
        zones.insert(
            "wan".to_string(),
            FirewallZone {
                zone_type: "wan".to_string(),
                interfaces: vec!["wan0".to_string()],
                input_policy: "drop".to_string(),
                forward_policy: "drop".to_string(),
                masquerade_to: vec![],
                router_services: vec!["dhcp".to_string(), "dns".to_string()],
                deny_zones: vec![],
                allow_ping: Some(false),
                allow_dhcp_client: Some(true),
            },
        );

        let firewall = FirewallConfig {
            backend: "nftables".to_string(),
            profile: "router-default".to_string(),
            zones,
            port_forwards: vec![FirewallPortForward {
                name: "ssh".to_string(),
            }],
            custom_rules: FirewallCustomRules { enabled: false },
        };

        let desired = WlandDesiredState {
            version: 1,
            metadata: WlandMetadata {
                name: "default-router".to_string(),
                description: Some("Default WLANd router configuration".to_string()),
                version: Some("1".to_string()),
                labels: BTreeMap::new(),
                annotations: BTreeMap::new(),
            },
            wan,
            lan,
            phy: phy.clone(),
            bss: bss.clone(),
            wlan,
            firewall,
            management: WlandManagement {
                ui: WlandManagementEndpoint {
                    enabled: true,
                    listen: "lan".to_string(),
                    port: 8443,
                },
                api: WlandManagementEndpoint {
                    enabled: true,
                    listen: "lan".to_string(),
                    port: 9443,
                },
                ssh: WlandManagementSsh { enabled: false },
            },
            secrets: BTreeMap::from([(
                "wifi_home".to_string(),
                WlandSecretRef {
                    uri: "secret://wifi/home".to_string(),
                },
            )]),
            apply_policy: WlandApplyPolicy {
                rollback: true,
                lockout_protection: true,
                validation_required: true,
            },
        };

        let mut interfaces = BTreeMap::new();
        interfaces.insert(
            "enp2s0".to_string(),
            WlandInterfaceState {
                operstate: "up".to_string(),
                carrier: true,
                mac: Some("aa:bb:cc:dd:ee:ff".to_string()),
                ipv4: vec!["203.0.113.10/24".to_string()],
                default_route: true,
            },
        );

        let mut routes = BTreeMap::new();
        routes.insert(
            "default".to_string(),
            WlandRouteState {
                destination: None,
                gateway: Some("203.0.113.1".to_string()),
                interface: Some("enp2s0".to_string()),
                default_route: true,
            },
        );

        let mut observed_phy = BTreeMap::new();
        observed_phy.insert(
            "phy0".to_string(),
            PhyObserved {
                present: true,
                driver: Some("ath12k".to_string()),
                bands: vec!["2.4GHz".to_string(), "5GHz".to_string()],
                supports_ap: true,
                supports_multi_bss: true,
            },
        );

        let mut observed_wlan = BTreeMap::new();
        observed_wlan.insert(
            "wlan0".to_string(),
            WlanObserved {
                present: true,
                up: true,
                phy_binding: Some("phy0".to_string()),
                bss_binding: Some("bss0".to_string()),
                bridge: Some("br-lan0".to_string()),
            },
        );

        let mut observed_bss = BTreeMap::new();
        observed_bss.insert(
            "bss0".to_string(),
            BssObserved {
                state: "running".to_string(),
                ssid: "HomeNetwork".to_string(),
                clients: 8,
            },
        );

        let observed = WlandObservedState {
            timestamp: Some("2026-05-24T00:00:00Z".to_string()),
            interfaces,
            routes,
            phy: observed_phy,
            wlan: observed_wlan,
            bss: observed_bss,
            services: WlandServicesState {
                dhcp: Some("running".to_string()),
                dns: Some("running".to_string()),
                firewall: Some("applied".to_string()),
                nat: Some("enabled".to_string()),
            },
            firewall: FirewallObserved {
                backend: "nftables".to_string(),
                state: "applied".to_string(),
                mode: "managed".to_string(),
                active_tables: vec!["inet wland_filter".to_string()],
                ruleset_checksum: Some("sha256:8c0f4a9f3e6d1b2c".to_string()),
                desired_checksum: Some("sha256:8c0f4a9f3e6d1b2c".to_string()),
                drift: false,
                last_apply: Some(FirewallApplyRecord {
                    timestamp: "2026-05-24T00:00:00Z".to_string(),
                    result: "success".to_string(),
                }),
            },
            logs: LogsState {
                recent: vec![LogEntry {
                    timestamp: "2026-05-24T00:00:00Z".to_string(),
                    level: "info".to_string(),
                    message: "apply completed".to_string(),
                    source: Some("wland".to_string()),
                }],
                last_apply: Some("success".to_string()),
                last_validation_failure: None,
            },
            drift: false,
        };

        let runtime = WlandRuntimeState {
            timestamp: Some("2026-05-24T00:00:00Z".to_string()),
            clients: ClientsRuntime {
                timestamp: Some("2026-05-24T00:00:00Z".to_string()),
                stations: BTreeMap::from([(
                    "de:ad:be:ef:00:01".to_string(),
                    ClientStation {
                        mac: "de:ad:be:ef:00:01".to_string(),
                        ip: Some("192.168.1.120".to_string()),
                        hostname: Some("laptop".to_string()),
                        bss: Some("bss0".to_string()),
                        wlan: Some("wlan0".to_string()),
                        lan: Some("lan0".to_string()),
                        signal_dbm: Some(-48),
                        rx_bytes: Some(123_456_789),
                        tx_bytes: Some(987_654_321),
                        associated_for: Some("2h13m".to_string()),
                        state: "associated".to_string(),
                    },
                )]),
                leases: BTreeMap::from([(
                    "192.168.1.120".to_string(),
                    ClientLease {
                        mac: "de:ad:be:ef:00:01".to_string(),
                        hostname: Some("laptop".to_string()),
                        lan: Some("lan0".to_string()),
                        expires_in: Some("8h22m".to_string()),
                    },
                )]),
            },
            dhcp: DhcpRuntime {
                state: "running".to_string(),
                leases: 13,
            },
        };

        let validation = WlandValidationResult {
            valid: true,
            timestamp: Some("2026-05-24T00:00:00Z".to_string()),
            issues: vec![WlandValidationIssue {
                code: ErrorCode::ValidationFailed,
                message: "example".to_string(),
                scope: Some("wan0".to_string()),
            }],
        };

        let apply = WlandApplyPlan {
            timestamp: Some("2026-05-24T00:00:00Z".to_string()),
            status: "succeeded".to_string(),
            steps: vec![WlandApplyStep {
                name: "render firewall".to_string(),
                status: "done".to_string(),
                message: None,
            }],
            last_error: None,
        };

        WlandState {
            desired,
            observed,
            runtime,
            validation: Some(validation),
            apply: Some(apply),
        }
    }

    #[test]
    fn top_level_model_serializes_with_all_containers() {
        let state = sample_state();
        let json = serde_json::to_value(state).expect("state should serialize");

        let Value::Object(root) = json else {
            panic!("expected object");
        };

        for key in ["desired", "observed", "runtime", "validation", "apply"] {
            assert!(root.contains_key(key), "missing key {key}");
        }
    }

    #[test]
    fn validation_and_apply_types_are_constructible() {
        let validation = WlandValidationResult {
            valid: false,
            timestamp: None,
            issues: vec![WlandValidationIssue {
                code: ErrorCode::Conflict,
                message: "conflicting policy".to_string(),
                scope: Some("firewall".to_string()),
            }],
        };

        let apply = WlandApplyPlan {
            timestamp: None,
            status: "pending".to_string(),
            steps: vec![WlandApplyStep {
                name: "stage config".to_string(),
                status: "pending".to_string(),
                message: None,
            }],
            last_error: Some(crate::common::exceptions::WlandException::new(
                ErrorCode::ApplyFailed,
                "backend rejected config",
            )),
        };

        assert!(!validation.valid);
        assert_eq!(apply.status, "pending");
    }
}
