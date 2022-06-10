use serde::{Deserialize, Serialize};
use std::fmt;
use thiserror::Error;

/// response of a failed request
#[derive(Serialize, Deserialize, Debug, Error)]
pub struct ErrorMessage {
    pub code: i32,
    pub message: String,
}

impl fmt::Display for ErrorMessage {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "error code: {} msg: {}", self.code, self.message)
    }
}
