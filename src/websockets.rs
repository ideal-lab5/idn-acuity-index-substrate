use std::{
    net::SocketAddr,
};

use tokio::net::{TcpListener, TcpStream};
use futures::{StreamExt, SinkExt};


async fn handle_connection(raw_stream: TcpStream, addr: SocketAddr, db: sled::Db) {
    println!("Incoming TCP connection from: {}", addr);

    let ws_stream = tokio_tungstenite::accept_async(raw_stream)
        .await
        .expect("Error during the websocket handshake occurred");
    println!("WebSocket connection established: {}", addr);

    let (mut ws_sender, mut ws_receiver) = ws_stream.split();


/*
    let mut iterator = db.iterator_cf(&db.cf_handle("order_value").unwrap(), IteratorMode::Start);
    let orders = iterator.collect::<Vec<_>>();
    println!("Orders: {:?}", orders);
    let mut iterator = db.iterator_cf(&db.cf_handle("order_static").unwrap(), IteratorMode::Start);
    let orders = iterator.collect::<Vec<_>>();
    println!("Orders: {:?}", orders);
*/
    loop {
        tokio::select! {
            Some(msg) = ws_receiver.next() => {
                let msg = msg.unwrap();
                if msg.is_text() || msg.is_binary() {
/*
                    let json = process_msg(&db, serde_json::from_str(msg.to_text().unwrap()).unwrap()).await;
                    ws_sender.send(tokio_tungstenite::tungstenite::Message::Text(json)).await.unwrap();
*/
                }
            }
        }
    }
}


pub async fn websockets_listen(db: sled::Db) {
    let addr = "127.0.0.1:8080".to_string();

    // Create the event loop and TCP listener we'll accept connections on.
    let try_socket = TcpListener::bind(&addr).await;
    let listener = try_socket.expect("Failed to bind");
    println!("Listening on: {}", addr);

    // Let's spawn the handling of each connection in a separate task.
    while let Ok((stream, addr)) = listener.accept().await {
        tokio::spawn(handle_connection(stream, addr, db.clone()));
    }
}
