use std::time::Duration;

use dotenvy_macro::dotenv;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

pub struct PostgresConf;

impl PostgresConf {
    pub async fn connect() -> Pool<Postgres> {
        let max_pool = dotenv!("POSTGRES_MAX_POOL").parse::<u32>().unwrap_or(1);
        let ac_timeout = dotenv!("POSTGRES_AC_TIMEOUT").parse::<u64>().unwrap_or(1);
        let database_url = dotenv!("POSTGRES_SCHEMA_URL");

        let pool = PgPoolOptions::new()
            .max_connections(max_pool)
            .acquire_timeout(Duration::from_secs(ac_timeout))
            .connect(&database_url)
            .await
            .expect("Can't connect to database");

        sqlx::migrate!("./migrations").run(&pool).await.unwrap();

        pool
    }
}
