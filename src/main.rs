mod config;
mod middleware;
mod models;
mod routes;
mod utils;

use std::error::Error;

use config::{AppConf, PostgresConf};
use routes::app_routes;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    tracing_subscriber::fmt::init();
    dotenvy::dotenv()?;

    let addr = AppConf::get_socket_addr();
    let pool = PostgresConf::connect().await;

    let app = app_routes(pool);

    tracing::info!("->> Listening on {}", &addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}
