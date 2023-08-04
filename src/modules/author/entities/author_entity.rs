use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Pool, Postgres};
use validator::Validate;

use crate::shared::models::dto::response::{NotFoundResponse, UnprocessableResponse};

#[derive(Debug, Default, FromRow, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct AuthorEntity {
    #[serde(skip_deserializing)]
    pub id: i64,
    #[validate(length(min = 3, max = 255))]
    pub first_name: String,
    #[validate(length(min = 3, max = 255))]
    pub last_name: String,
}

impl AuthorEntity {
    pub async fn get_author(db: &Pool<Postgres>, id: i64) -> Response {
        let query = sqlx::query_as::<_, AuthorEntity>(
            r#"SELECT id, first_name, last_name FROM authors WHERE id = $1"#,
        );

        let f_fetch = query.bind(id).fetch_one(db);

        match f_fetch.await {
            Ok(author) => (StatusCode::OK, Json(author)).into_response(),
            Err(_) => NotFoundResponse::report("The author is not found"),
        }
    }

    pub async fn create_author(db: &Pool<Postgres>, data: &AuthorEntity) -> Response {
        let query = sqlx::query_as::<_, AuthorEntity>(
            r#"INSERT INTO authors (first_name, last_name) VALUES ($1, $2) RETURNING id, first_name, last_name"#,
        );

        let f_fetch = query
            .bind(data.first_name.clone())
            .bind(data.last_name.clone())
            .fetch_one(db);

        match f_fetch.await {
            Ok(author) => (StatusCode::CREATED, Json(author)).into_response(),
            Err(_) => UnprocessableResponse::report("The author was not created").into_response(),
        }
    }
}
