use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{get, post},
    Json, Router,
};

use crate::{
    config::AppState, extractor::JsonExtractor, modules::author::entities::AuthorEntity,
    shared::models::response::FailedSqlxResponse,
};

async fn get_author(State(state): State<AppState>, Path(id): Path<i64>) -> Response {
    match AuthorEntity::get_author(state.db, id).await {
        Ok(author) => (StatusCode::OK, Json(author)).into_response(),
        Err(_) => {
            FailedSqlxResponse::to_pair(StatusCode::NOT_FOUND, "Author not found").into_response()
        }
    }
}

async fn create_author(
    State(state): State<AppState>,
    JsonExtractor(payload): JsonExtractor<AuthorEntity>,
) -> Response {
    match AuthorEntity::create_author(state.db, &payload).await {
        Ok(author) => (StatusCode::CREATED, Json(author)).into_response(),
        Err(_) => FailedSqlxResponse::to_pair(
            StatusCode::UNPROCESSABLE_ENTITY,
            "The author was not created",
        )
        .into_response(),
    }
}

pub fn author_routes() -> Router<AppState> {
    Router::new()
        .route("/author", post(create_author))
        .route("/author/:id", get(get_author))
}
