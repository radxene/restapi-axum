use std::sync::Arc;

use axum::{
    extract::{Path, State},
    response::Response,
    routing::{get, post},
    Router,
};

use crate::{config::AppState, extractor::ValidJson, modules::author::entities::AuthorEntity};

#[derive(Debug, PartialEq, strum::Display, strum::EnumString, strum::IntoStaticStr)]
pub enum AuthorPath {
    #[strum(serialize = "/author")]
    CreateAuthor,
    #[strum(serialize = "/author/:id")]
    GetAuthor,
}

#[utoipa::path(
    get,
    path = "/author/:id",
    responses(
        (status = StatusCode::OK, description = "Get the author by id"),
        (status = StatusCode::NOT_FOUND, description = "Not found author by id", body = RejectJsonResponse),
    ),
    params(
        ("id" = i64, Path, description = "Author database id to get Author for"),
    ),
)]
pub async fn get_author(State(state): State<Arc<AppState>>, Path(id): Path<i64>) -> Response {
    AuthorEntity::get_author(&state.db, id).await
}

#[utoipa::path(
    post,
    path = "/author",
    responses(
        (status = StatusCode::CREATED, description = "Create the author")
    )
)]
pub async fn create_author(
    State(state): State<Arc<AppState>>,
    ValidJson(payload): ValidJson<AuthorEntity>,
) -> Response {
    AuthorEntity::create_author(&state.db, &payload).await
}

pub fn author_routes() -> Router<Arc<AppState>> {
    Router::new()
        .route(AuthorPath::CreateAuthor.into(), post(create_author))
        .route(AuthorPath::GetAuthor.into(), get(get_author))
}
