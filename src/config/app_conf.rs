use dotenvy_macro::dotenv;

use std::net::{SocketAddr, ToSocketAddrs};

pub struct AppConf;

impl AppConf {
    pub fn get_socket_addr() -> SocketAddr {
        let server = dotenv!("APP_HOST_NAME")
            .to_socket_addrs()
            .expect("Unable to resolve domain")
            .collect::<Vec<_>>();

        *server.first().unwrap()
    }
}
