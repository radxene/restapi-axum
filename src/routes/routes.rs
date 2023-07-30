use axum::{routing::get, Router};
use sqlx::{Pool, Postgres};

pub fn app_routes(pool: Pool<Postgres>) -> Router {
    Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .with_state(pool)
}
