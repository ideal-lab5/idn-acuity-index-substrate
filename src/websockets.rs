use std::{
    net::SocketAddr,
};
use serde::{Serialize, Deserialize};
use tokio::net::{TcpListener, TcpStream};
use futures::{StreamExt, SinkExt};
use subxt::utils::AccountId32;

use crate::shared::*;

#[derive(Deserialize, Debug, Clone)]
#[serde(tag = "type")]
pub enum RequestMessage {
    GetEventsAccountId {
        account_id: AccountId32,
    },
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct EventFull {
    block_number: u32,
    event: Event,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct ResponseMessage {
    events: Vec<EventFull>,
}

async fn process_msg(trees: &Trees, msg: RequestMessage) -> String {
    println!("msg: {:?}", msg);

    match msg {
        RequestMessage::GetEventsAccountId { account_id } => {
            println!("getEventsAccountId: {}", account_id);

            let mut events = Vec::new();

            for kv in trees.account_id.scan_prefix(account_id) {
                let kv = kv.unwrap();
                let key = AccountIdKey::unserialize(kv.0.to_vec());
                println!("value: {:?}", kv.1);
                let event: Event = bincode::decode_from_slice(&kv.1.to_vec(), bincode::config::standard()).unwrap().0;

                events.push(EventFull {
                    block_number: key.block_number,
                    event: event,
                });
            }

            let response_message = ResponseMessage {events: events};

            serde_json::to_string(&response_message).unwrap()
        },
    }
}

async fn handle_connection(raw_stream: TcpStream, addr: SocketAddr, trees: Trees) {
    println!("Incoming TCP connection from: {}", addr);

    let ws_stream = tokio_tungstenite::accept_async(raw_stream)
        .await
        .expect("Error during the websocket handshake occurred");
    println!("WebSocket connection established: {}", addr);

    let (mut ws_sender, mut ws_receiver) = ws_stream.split();

    loop {
        tokio::select! {
            Some(msg) = ws_receiver.next() => {
                let msg = msg.unwrap();
                println!("Message: {}", msg.to_text().unwrap());

                if msg.is_text() || msg.is_binary() {
                    let json = process_msg(&trees, serde_json::from_str(msg.to_text().unwrap()).unwrap()).await;
                    ws_sender.send(tokio_tungstenite::tungstenite::Message::Text(json)).await.unwrap();
                }
            }
        }
    }
}


pub async fn websockets_listen(trees: Trees) {
    let addr = "127.0.0.1:8080".to_string();

    // Create the event loop and TCP listener we'll accept connections on.
    let try_socket = TcpListener::bind(&addr).await;
    let listener = try_socket.expect("Failed to bind");
    println!("Listening on: {}", addr);

    // Let's spawn the handling of each connection in a separate task.
    while let Ok((stream, addr)) = listener.accept().await {
        tokio::spawn(handle_connection(stream, addr, trees.clone()));
    }
}
