use axum::{http::StatusCode, response::Response};

use super::RejectJsonResponse;

#[derive(Debug, Clone)]
pub struct UnprocessableResponse;

impl UnprocessableResponse {
    pub fn report(message: &str) -> Response {
        RejectJsonResponse::report(StatusCode::UNPROCESSABLE_ENTITY, message)
    }
}
