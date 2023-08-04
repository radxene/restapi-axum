use axum::{http::StatusCode, response::Response};

use super::RejectJsonResponse;

#[derive(Debug, Clone)]
pub struct NotFoundResponse;

impl NotFoundResponse {
    pub fn report(message: &str) -> Response {
        RejectJsonResponse::report(StatusCode::NOT_FOUND, message)
    }
}
