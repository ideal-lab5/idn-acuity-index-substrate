use std::{
    net::SocketAddr,
};
use serde::{Serialize, Deserialize};
use tokio::net::{TcpListener, TcpStream};
use futures::{StreamExt, SinkExt};
use subxt::utils::AccountId32;
use serde_json::{Result, Value};

use crate::shared::*;

#[derive(Deserialize, Debug, Clone)]
#[serde(tag = "type")]
pub enum RequestMessage {
    GetEventsAccountId {
        account_id: AccountId32,
    },
}

#[derive(Serialize, Debug)]
#[serde(tag = "type")]
#[serde(rename_all = "camelCase")]
enum JsonResponseMessage {
    #[serde(rename_all = "camelCase")]
    Transfers {
    },
}

async fn process_msg(db: &sled::Db, msg: RequestMessage) -> String {
    println!("msg: {:?}", msg);

    match msg {
        RequestMessage::GetEventsAccountId { account_id } => {
            println!("getEventsAccountId: {}", account_id);

            for kv in db.scan_prefix(account_id) {
                let kv = kv.unwrap();
                let key = AccountIdKey::unserialize(kv.0.to_vec());
                let value = TransferEventValue::unserialize(kv.1.to_vec());
                println!("From: {:}", value.from);
                println!("To: {:}", value.to);
                println!("Amount: {:}", value.value);
            }

            let response = JsonResponseMessage::Transfers {
            };
            serde_json::to_string(&response).unwrap()
        },
    }
}

async fn handle_connection(raw_stream: TcpStream, addr: SocketAddr, db: sled::Db) {
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
                    let json = process_msg(&db, serde_json::from_str(msg.to_text().unwrap()).unwrap()).await;
                //    ws_sender.send(tokio_tungstenite::tungstenite::Message::Text(json)).await.unwrap();
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
