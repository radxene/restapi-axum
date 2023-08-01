mod config;
mod middleware;
mod modules;
mod shared;
mod utils;

use std::error::Error;

use config::{AppConf, PostgresConf};
use modules::app::routes::app_routes;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    AppConf::init()?;

    let pool = PostgresConf::connect().await;
    let addr = AppConf::get_app_addr();
    let state = AppConf::get_app_state(&pool);
    let app = app_routes(&state).await;

    AppConf::listening(&addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}
