use std::{
    collections::HashMap,
    net::SocketAddr,
    sync::{Arc, Mutex},
};

use futures::pin_mut;
use futures_channel::mpsc::{unbounded, UnboundedSender};
use futures_util::{StreamExt, TryStreamExt};
use log::{error, info};
use tokio::net::TcpStream;
use tokio_tungstenite::{
    accept_async,
    tungstenite::{Error, Message, Result},
};

pub type PeerMap = Arc<Mutex<HashMap<SocketAddr, UnboundedSender<Message>>>>;

pub async fn accept_connection(peer_address: SocketAddr, stream: TcpStream, peers: PeerMap) {
    if let Err(e) = handle_connection(peer_address, stream, peers).await {
        match e {
            Error::ConnectionClosed | Error::Protocol(_) | Error::Utf8 => (),
            err => error!("Error processing connection: {}", err),
        }
    }
}

async fn handle_connection(
    peer_address: SocketAddr,
    stream: TcpStream,
    peer_map: PeerMap,
) -> Result<()> {
    info!("Handling connection from {}", &peer_address);
    let ws_stream = match accept_async(stream).await {
        Ok(ws) => ws,
        Err(e) => return Err(e),
    };

    let (sender, receiver) = unbounded();
    peer_map.lock().unwrap().insert(peer_address, sender);

    let (outgoing, incoming) = ws_stream.split();
    let broadcast_callback = incoming.try_for_each(|message| {
        let peers = peer_map.lock().unwrap();
        for (_, peer) in peers.iter() {
            peer.unbounded_send(message.clone()).unwrap();
        }
        futures::future::ok(())
    });
    let receive_mpsc = receiver.map(Ok).forward(outgoing);

    pin_mut!(broadcast_callback, receive_mpsc);
    futures::future::select(broadcast_callback, receive_mpsc).await;

    info!("{} disconected", &peer_address);
    peer_map.lock().unwrap().remove(&peer_address);
    Ok(())
}
