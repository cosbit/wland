use crate::common::errcodes::ErrorCode;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WlandException {
    pub code: ErrorCode,
    pub message: String,
    pub scope: Option<String>,
    pub details: Option<String>,
}

impl WlandException {
    pub fn new(code: ErrorCode, message: impl Into<String>) -> Self {
        Self {
            code,
            message: message.into(),
            scope: None,
            details: None,
        }
    }
}
