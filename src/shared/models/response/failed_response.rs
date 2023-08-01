use axum::http::StatusCode;
use serde::Serialize;

#[derive(Clone, Serialize, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub struct FailedResponse {
    code: u16,
    details: String,
}

impl FailedResponse {
    pub fn new(status: StatusCode, details: &str) -> Self {
        Self {
            code: status.as_u16(),
            details: details.to_owned(),
        }
    }
}
