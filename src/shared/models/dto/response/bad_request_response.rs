use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde_json::Value;

use super::RejectValidatorResponse;

#[derive(Debug, Clone)]
pub struct BadRequestResponse;

impl BadRequestResponse {
    pub fn report(message: &str, errors: Value) -> Response {
        RejectValidatorResponse::report(StatusCode::BAD_REQUEST, message, errors).into_response()
    }
}
