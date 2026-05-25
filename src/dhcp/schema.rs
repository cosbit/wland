use serde::{Deserialize, Serialize};

#[allow(unused_imports)]
pub use crate::common::{DhcpLeaseObserved, DhcpServerDesired};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DhcpRuntime {
    pub state: String,
    pub leases: u32,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct DhcpLease {
    pub mac: String,
    pub hostname: Option<String>,
    pub lan: Option<String>,
    pub expires_in: Option<String>,
}
