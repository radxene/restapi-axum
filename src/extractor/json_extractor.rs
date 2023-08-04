use axum::{
    async_trait,
    body::HttpBody,
    extract::{rejection::JsonRejection, FromRequest, Json as ExtJson},
    http::Request,
    response::IntoResponse,
    response::Response,
    BoxError,
};
use serde::de::DeserializeOwned;
use thiserror::Error;
use validator::{Validate, ValidationErrors};

use crate::shared::models::dto::response::{BadRequestResponse, UnprocessableResponse};

#[derive(Debug, Clone, Copy, Default)]
pub struct ValidJson<T>(pub T);

#[async_trait]
impl<S, B, T> FromRequest<S, B> for ValidJson<T>
where
    S: Send + Sync,
    B: HttpBody + Send + 'static,
    B::Data: Send,
    B::Error: Into<BoxError>,
    T: DeserializeOwned + Validate,
{
    type Rejection = ServerError;

    async fn from_request(req: Request<B>, state: &S) -> Result<Self, Self::Rejection> {
        let ExtJson(value) = ExtJson::<T>::from_request(req, state).await?;
        value.validate()?;
        Ok(Self(value))
    }
}

#[derive(Debug, Error)]
pub enum ServerError {
    #[error(transparent)]
    ValidationError(#[from] ValidationErrors),

    #[error(transparent)]
    AxumJsonError(#[from] JsonRejection),
}

impl IntoResponse for ServerError {
    fn into_response(self) -> Response {
        match self {
            Self::ValidationError(err) => BadRequestResponse::report(
                "Validation failed.",
                serde_json::to_value(&err).unwrap(),
            ),
            Self::AxumJsonError(err) => UnprocessableResponse::report(err.to_string().as_str()),
        }
    }
}
