use std::{collections::HashMap, io::Error as IoError, sync::Mutex};

use dotenv::dotenv;
use log::{error, info};
use tokio::net::TcpListener;

use crate::handler::{accept_connection, PeerMap};

mod handler;

#[tokio::main]
async fn main() -> Result<(), IoError> {
    dotenv().ok();
    env_logger::init();
    info!("Starting server");

    let host = std::env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_owned());
    let port = std::env::var("PORT").unwrap_or_else(|_| "8000".to_owned());
    info!("Binding server to {}:{}", &host, &port);
    let try_socket = TcpListener::bind(format!("{}:{}", &host, &port)).await;
    let listener = match try_socket {
        Ok(l) => l,
        Err(err) => {
            error!("{}", &err);
            panic!();
        }
    };
    info!("Successfully listening to {}:{}", &host, &port);

    let peers = PeerMap::new(Mutex::new(HashMap::new()));
    while let Ok((stream, _)) = listener.accept().await {
        let peer = match stream.peer_addr() {
            Ok(p) => p,
            Err(err) => {
                error!("Stream without peer address: {}", &err);
                panic!()
            }
        };
        tokio::spawn(accept_connection(peer, stream, peers.clone()));
    }
    Ok(())
}
