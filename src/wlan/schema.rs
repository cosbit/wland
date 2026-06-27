use serde::{Deserialize, Serialize};

use crate::common::schemas::{BssId, LanId, PhyId, WlanInterfaceRef};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WlanDesired {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    pub phy_binding: PhyId,
    pub bss_binding: BssId,
    pub lan_binding: LanId,
    pub interface: WlanInterfaceRef,
}
