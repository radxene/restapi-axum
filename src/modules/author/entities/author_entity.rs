use serde::{Deserialize, Serialize};
use sqlx::{Error, FromRow, Pool, Postgres};
use validator::Validate;

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
    pub async fn get_author(db: Pool<Postgres>, id: i64) -> Result<AuthorEntity, Error> {
        let query = sqlx::query_as::<_, AuthorEntity>(
            r#"SELECT id, first_name, last_name FROM authors WHERE id = $1"#,
        );
        let author = query.bind(id).fetch_one(&db).await?;

        Ok(author)
    }

    pub async fn create_author(
        db: Pool<Postgres>,
        data: &AuthorEntity,
    ) -> Result<AuthorEntity, Error> {
        let query = sqlx::query_as::<_, AuthorEntity>(
            r#"INSERT INTO authors (first_name, last_name) VALUES ($1, $2) RETURNING id, first_name, last_name"#,
        );
        let author = query
            .bind(data.first_name.clone())
            .bind(data.last_name.clone())
            .fetch_one(&db)
            .await?;

        Ok(author)
    }
}
