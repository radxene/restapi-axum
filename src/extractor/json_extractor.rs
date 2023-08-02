use axum::{
    extract::{rejection::JsonRejection, FromRequest},
    http::StatusCode,
    response::IntoResponse,
    response::Response,
    Json,
};
use serde_json::json;

#[derive(FromRequest)]
#[from_request(via(Json), rejection(ApiError))]
pub struct JsonExtractor<T>(pub T);

#[derive(Debug)]
pub struct ApiError {
    status: StatusCode,
    message: String,
}

impl From<JsonRejection> for ApiError {
    fn from(rejection: JsonRejection) -> Self {
        Self {
            // rejection.status() default is 422 (Unprocessable Entity)
            status: StatusCode::BAD_REQUEST,
            message: rejection.body_text(),
        }
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let payload = json!({
            "code": self.status.as_u16(),
            "message": self.message,
            "origin": "derive_from_request"
        });

        (self.status, axum::Json(payload)).into_response()
    }
}
