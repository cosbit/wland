use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ErrorCode {
    InvalidConfiguration,
    ValidationFailed,
    ApplyFailed,
    NotFound,
    Conflict,
    LockoutPrevented,
    InternalError,
}
