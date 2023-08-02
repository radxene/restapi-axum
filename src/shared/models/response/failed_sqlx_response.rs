use axum::{http::StatusCode, Json};
use serde::Serialize;

#[derive(Clone, Serialize, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub struct FailedSqlxResponse {
    code: u16,
    details: String,
}

impl FailedSqlxResponse {
    pub fn to_pair(status: StatusCode, details: &str) -> (StatusCode, Json<Self>) {
        (
            status,
            Json(Self {
                code: status.as_u16(),
                details: details.to_owned(),
            }),
        )
    }
}
