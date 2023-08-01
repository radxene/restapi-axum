use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{get, post},
    Json, Router,
};

use crate::{
    config::AppState, modules::author::entities::AuthorEntity,
    shared::models::response::FailedResponse,
};

async fn get_author(State(state): State<AppState>, Path(id): Path<i64>) -> Response {
    let failed_response = FailedResponse::new(StatusCode::NOT_FOUND, "Author not found");
    match AuthorEntity::get_author(state.db, id).await {
        Ok(author) => (StatusCode::OK, Json(author)).into_response(),
        Err(_) => (StatusCode::NOT_FOUND, Json(failed_response)).into_response(),
    }
}

async fn create_author(
    State(state): State<AppState>,
    Json(payload): Json<AuthorEntity>,
) -> Response {
    let failed_response = FailedResponse::new(
        StatusCode::UNPROCESSABLE_ENTITY,
        "The author was not created",
    );
    match AuthorEntity::create_author(state.db, &payload).await {
        Ok(author) => (StatusCode::CREATED, Json(author)).into_response(),
        Err(_) => (StatusCode::UNPROCESSABLE_ENTITY, Json(failed_response)).into_response(),
    }
}

pub fn author_routes() -> Router<AppState> {
    Router::new()
        .route("/author", post(create_author))
        .route("/author/:id", get(get_author))
}
