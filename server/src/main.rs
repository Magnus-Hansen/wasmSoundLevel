use tokio::net::{TcpListener, UdpSocket};
use tokio_tungstenite::tungstenite::protocol::Message;
use tokio_tungstenite::accept_async;
use futures_util::{StreamExt, SinkExt};
use std::sync::{Arc, Mutex};
use tokio_tungstenite::tungstenite::{util};
// Server side
// This is a simple UDP server that listens for incoming messages on port 9000

type Client = futures_util::stream::SplitSink<tokio_tungstenite::WebSocketStream<tokio::net::TcpStream>, Message>;

#[tokio::main]
async fn main() {
    let clients = Arc::new(Mutex::new(Vec::<Client>::new()));

    // Start the WebSocket server
    let clients_ws = Arc::clone(&clients);
    tokio::spawn(async move {
        let listener = TcpListener::bind("127.0.0.1:9000").await.unwrap();
        println!("Websocket server listening on ws://localhost:9000");

        loop {
            let (stream, _) = listener.accept().await.unwrap();
            let ws_stream = accept_async(stream).await.unwrap();
            println!("New websocket client connected");

            let (write, _) = ws_stream.split();
            clients_ws.lock().unwrap().push(write);
        }
    });

    // UDP Receiver
    let udp_socket = UdpSocket::bind("127.0.0.1:8080").await.unwrap();
    println!("UDP socket listening on udp://127.0.0.1:8080");

    let mut buf = [0u8; 1024];
    loop {
        println!("[UDP] Waiting for data...");
        println!("[UDP] Got UDP packet of size {}", buf.len());
        let (len, _) = udp_socket.recv_from(&mut buf).await.unwrap();
        if len < 4 { continue; }

        let number = i32::from_be_bytes(buf[0..4].try_into().unwrap());
        println!("Received from Microbit: {}", number);

        let message = Message::Text(number.to_string());

        // Broadcast to all WebSocket clients
        let mut clients_guard = clients.lock().unwrap();
        clients_guard.retain_mut(|client| {
            futures::executor::block_on(client.send(message.clone())).is_ok()
        });
    }
}