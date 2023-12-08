use std::net::SocketAddr;

use futures_util::{SinkExt, StreamExt};
use log::LevelFilter;
use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::tungstenite::Message;

#[tokio::main]
async fn main() {
    env_logger::builder()
        .filter_level(LevelFilter::Info)
        .filter(Some("simple_websocket"), LevelFilter::Debug)
        .init();

    let addr = "0.0.0.0:1234";
    let listener = TcpListener::bind(addr).await.expect("Failed to bind");
    log::info!("Listening on: {}", addr);

    while let Ok((stream, addr)) = listener.accept().await {
        tokio::spawn(accept_connection(stream, addr));
    }
}

async fn accept_connection(stream: TcpStream, addr: SocketAddr) {
    log::info!("Incoming TCP connection from: {}", addr);

    let mut ws_stream = tokio_tungstenite::accept_async(stream)
        .await
        .expect("Error during the websocket handshake occurred");
    log::info!("WebSocket connection established: {}", addr);

    while let Some(Ok(msg)) = ws_stream.next().await {
        if msg.is_close() {
            break;
        }
        if msg.is_text() {
            log::debug!("Received a message from {}: {}", addr, msg.to_text().unwrap());
            if msg.to_text().unwrap() == "PING" {
                ws_stream.send(Message::Text("PONG".to_string())).await.unwrap();
            } else {
                ws_stream.send(msg).await.unwrap();
            }
        } else {
            log::debug!("Received a non-text message from {}", addr);
            ws_stream.send(msg).await.unwrap();
        }
    }

    log::info!("{} disconnected", &addr);
}