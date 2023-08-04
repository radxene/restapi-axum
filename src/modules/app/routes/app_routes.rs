use std::sync::Arc;

use axum::{routing::get, Router};
use tower_http::catch_panic::CatchPanicLayer;

use crate::modules::author::routes::author_routes;
use crate::modules::swagger_ui::routes::swagger_ui_routes;
use crate::{config::AppState, middleware::catch_panic::custom_handle_panic};

pub async fn app_routes(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/panic", get(|| async { panic!() }))
        .merge(author_routes())
        .merge(swagger_ui_routes())
        .layer(CatchPanicLayer::custom(custom_handle_panic))
        .with_state(state)
}
