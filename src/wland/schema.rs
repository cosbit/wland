use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

use crate::common::schemas::{BssId, LanId, Metadata, PhyId, WanId, WlanId};
use crate::firewall::schema::FirewallDesired;
use crate::lan::schema::LanDesired;
use crate::wland::management::schema::ManagementDesired;
use crate::phy::schema::WirelessPhyDesired as PhyDesired;
use crate::wan::schema::WanDesired;
use crate::wlan::schema::WlanDesired;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WlandDesiredState {
    pub version: u32,
    pub metadata: Metadata,

    #[serde(default)]
    pub wan: BTreeMap<WanId, WanDesired>,

    #[serde(default)]
    pub lan: BTreeMap<LanId, LanDesired>,

    #[serde(default)]
    pub phy: BTreeMap<PhyId, PhyDesired>,

    #[serde(default)]
    pub bss: BTreeMap<BssId, crate::bss::schema::BssConfig>,

    #[serde(default)]
    pub wlan: BTreeMap<WlanId, WlanDesired>,

    pub firewall: FirewallDesired,
    pub management: ManagementDesired,
}
