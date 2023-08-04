use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
pub struct RejectValidatorResponse {
    code: u16,
    message: String,
    errors: Option<Value>,
}

impl RejectValidatorResponse {
    pub fn report(status: StatusCode, message: &str, errors: Value) -> Response {
        let value = Self {
            code: status.as_u16(),
            message: message.to_owned(),
            errors: Some(errors),
        };

        (status, Json(value)).into_response()
    }
}
