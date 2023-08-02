use std::{
    error::Error,
    net::{SocketAddr, ToSocketAddrs},
};

use axum::extract::FromRef;
use dotenvy_macro::dotenv;
use sqlx::{Pool, Postgres};
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

#[derive(FromRef, Clone)]
pub struct AppState {
    pub db: Pool<Postgres>,
}

pub struct AppConf;

impl AppConf {
    pub fn init() -> Result<(), Box<dyn Error>> {
        dotenvy::dotenv()?;
        tracing_subscriber::registry()
            .with(EnvFilter::try_from_env("LOGGER_TRACING_LEVEL").unwrap_or_default())
            .with(fmt::layer())
            .init();
        Ok(())
    }

    pub fn listening(addr: &SocketAddr) {
        tracing::info!("->> Listening on {}", &addr);
    }

    pub fn get_app_addr() -> SocketAddr {
        let server = dotenv!("APP_HOST_NAME")
            .to_socket_addrs()
            .expect("Unable to resolve domain")
            .collect::<Vec<_>>();

        *server.first().unwrap()
    }

    pub fn get_app_state(pool: &Pool<Postgres>) -> AppState {
        AppState { db: pool.clone() }
    }
}
