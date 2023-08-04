use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq, ToSchema)]
pub struct RejectJsonResponse {
    code: u16,
    message: String,
}

impl RejectJsonResponse {
    pub fn report(status: StatusCode, message: &str) -> Response {
        let value = Self {
            code: status.as_u16(),
            message: message.to_owned(),
        };

        (status, Json(value)).into_response()
    }
}
