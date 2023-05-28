use std::{
    net::SocketAddr,
};
use tokio::net::{TcpListener, TcpStream};
use futures::{StreamExt, SinkExt};

use tokio::sync::mpsc;

use crate::shared::*;

pub async fn process_msg(trees: &Trees, msg: RequestMessage, sub_tx: Sender<SubscribeMessage>, sub_response_tx: Sender<ResponseMessage>) -> ResponseMessage {
    println!("{:?}", msg);
    match msg {
        RequestMessage::Status => {
            ResponseMessage::Status {
                last_head_block: match trees.root.get("last_head_block").unwrap() {
                    Some(value) => u32::from_be_bytes(vector_as_u8_4_array(&value)),
                    None => 0,
                },
                last_batch_block: match trees.root.get("last_batch_block").unwrap() {
                    Some(value) => u32::from_be_bytes(vector_as_u8_4_array(&value)),
                    None => 0,
                },
                batch_indexing_complete: match trees.root.get("batch_indexing_complete").unwrap() {
                    Some(value) => value.to_vec()[0] == 1,
                    None => false,
                },
            }
        },
        RequestMessage::GetEvents { key } => {
            match key {
                Key::AccountId(account_id) => {
                    let mut events = Vec::new();
        
                    for kv in trees.account_id.scan_prefix(account_id.clone()) {
                        let kv = kv.unwrap();
                        let key = AccountIdKey::unserialize(kv.0.to_vec());
        
                        events.push(Event {
                            block_number: key.block_number,
                            i: key.i,
                        });
                    }
        
                    ResponseMessage::Events {
                        key: Key::AccountId(account_id),
                        events
                    }
                },
                Key::AccountIndex(account_index) => {
                    let mut events = Vec::new();
        
                    for kv in trees.account_index.scan_prefix(account_index.to_be_bytes()) {
                        let kv = kv.unwrap();
                        let key = U32Key::unserialize(kv.0.to_vec());
        
                        events.push(Event {
                            block_number: key.block_number,
                            i: key.i,
                        });
                    }
        
                    ResponseMessage::Events {
                        key: Key::AccountIndex(account_index),
                        events
                    }
                },
                Key::AuctionIndex(auction_index) => {
                    let mut events = Vec::new();
        
                    for kv in trees.auction_index.scan_prefix(auction_index.to_be_bytes()) {
                        let kv = kv.unwrap();
                        let key = U32Key::unserialize(kv.0.to_vec());
        
                        events.push(Event {
                            block_number: key.block_number,
                            i: key.i,
                        });
                    }
        
                    ResponseMessage::Events {
                        key: Key::AuctionIndex(auction_index),
                        events
                    }
                },
                Key::BountyIndex(bounty_index) => {
                    let mut events = Vec::new();
        
                    for kv in trees.bounty_index.scan_prefix(bounty_index.to_be_bytes()) {
                        let kv = kv.unwrap();
                        let key = U32Key::unserialize(kv.0.to_vec());
        
                        events.push(Event {
                            block_number: key.block_number,
                            i: key.i,
                        });
                    }
        
                    ResponseMessage::Events {
                        key: Key::BountyIndex(bounty_index),
                        events
                    }
                },
                Key::CandidateHash(candidate_hash) => {
                    let mut events = Vec::new();
        
                    for kv in trees.candidate_hash.scan_prefix(&candidate_hash) {
                        let kv = kv.unwrap();
                        let key = CandidateHashKey::unserialize(kv.0.to_vec());
    
                        events.push(Event {
                            block_number: key.block_number,
                            i: key.i,
                        });
                    }

                    ResponseMessage::Events {
                        key: Key::CandidateHash(candidate_hash),
                        events
                    }
                },
                Key::EraIndex(era_index) => {
                    let mut events = Vec::new();
        
                    for kv in trees.era_index.scan_prefix(era_index.to_be_bytes()) {
                        let kv = kv.unwrap();
                        let key = U32Key::unserialize(kv.0.to_vec());
        
                        events.push(Event {
                            block_number: key.block_number,
                            i: key.i,
                        });
                    }
        
                    ResponseMessage::Events {
                        key: Key::EraIndex(era_index),
                        events
                    }
                },
                Key::MessageId(message_id) => {
                    let mut events = Vec::new();

                    for kv in trees.message_id.scan_prefix(&message_id) {
                        let kv = kv.unwrap();
                        let key = MessageIdKey::unserialize(kv.0.to_vec());

                        events.push(Event {
                            block_number: key.block_number,
                            i: key.i,
                        });
                    }

                    ResponseMessage::Events {
                        key: Key::MessageId(message_id),
                        events
                    }
                },
                Key::ParaId(para_id) => {
                    let mut events = Vec::new();
        
                    for kv in trees.para_id.scan_prefix(para_id.to_be_bytes()) {
                        let kv = kv.unwrap();
                        let key = U32Key::unserialize(kv.0.to_vec());
        
                        events.push(Event {
                            block_number: key.block_number,
                            i: key.i,
                        });
                    }
        
                    ResponseMessage::Events {
                        key: Key::ParaId(para_id),
                        events
                    }
                },
                Key::PoolId(pool_id) => {
                    let mut events = Vec::new();
        
                    for kv in trees.pool_id.scan_prefix(pool_id.to_be_bytes()) {
                        let kv = kv.unwrap();
                        let key = U32Key::unserialize(kv.0.to_vec());
        
                        events.push(Event {
                            block_number: key.block_number,
                            i: key.i,
                        });
                    }
        
                    ResponseMessage::Events {
                        key: Key::PoolId(pool_id),
                        events
                    }
                },
                Key::PreimageHash(preimage_hash) => {
                    let mut events = Vec::new();

                    for kv in trees.preimage_hash.scan_prefix(&preimage_hash) {
                        let kv = kv.unwrap();
                        let key = HashKey::unserialize(kv.0.to_vec());

                        events.push(Event {
                            block_number: key.block_number,
                            i: key.i,
                        });
                    }

                    ResponseMessage::Events {
                        key: Key::PreimageHash(preimage_hash),
                        events
                    }
                },
                Key::ProposalHash(proposal_hash) => {
                    let mut events = Vec::new();

                    for kv in trees.proposal_hash.scan_prefix(&proposal_hash) {
                        let kv = kv.unwrap();
                        let key = HashKey::unserialize(kv.0.to_vec());

                        events.push(Event {
                            block_number: key.block_number,
                            i: key.i,
                        });
                    }

                    ResponseMessage::Events {
                        key: Key::ProposalHash(proposal_hash),
                        events
                    }
                },
                Key::ProposalIndex(proposal_index) => {
                    let mut events = Vec::new();
        
                    for kv in trees.proposal_index.scan_prefix(proposal_index.to_be_bytes()) {
                        let kv = kv.unwrap();
                        let key = U32Key::unserialize(kv.0.to_vec());
        
                        events.push(Event {
                            block_number: key.block_number,
                            i: key.i,
                        });
                    }
        
                    ResponseMessage::Events {
                        key: Key::ProposalIndex(proposal_index),
                        events
                    }
                },
                Key::RefIndex(ref_index) => {
                    let mut events = Vec::new();
        
                    for kv in trees.ref_index.scan_prefix(ref_index.to_be_bytes()) {
                        let kv = kv.unwrap();
                        let key = U32Key::unserialize(kv.0.to_vec());
        
                        events.push(Event {
                            block_number: key.block_number,
                            i: key.i,
                        });
                    }
        
                    ResponseMessage::Events {
                        key: Key::RefIndex(ref_index),
                        events
                    }
                },
                Key::RegistrarIndex(registrar_index) => {
                    let mut events = Vec::new();
        
                    for kv in trees.registrar_index.scan_prefix(registrar_index.to_be_bytes()) {
                        let kv = kv.unwrap();
                        let key = U32Key::unserialize(kv.0.to_vec());
        
                        events.push(Event {
                            block_number: key.block_number,
                            i: key.i,
                        });
                    }
        
                    ResponseMessage::Events {
                        key: Key::RegistrarIndex(registrar_index),
                        events
                    }
                },
                Key::TipHash(tip_hash) => {
                    let mut events = Vec::new();

                    for kv in trees.tip_hash.scan_prefix(&tip_hash) {
                        let kv = kv.unwrap();
                        let key = TipHashKey::unserialize(kv.0.to_vec());

                        events.push(Event {
                            block_number: key.block_number,
                            i: key.i,
                        });
                    }

                    ResponseMessage::Events {
                        key: Key::TipHash(tip_hash),
                        events
                    }
                },
            }
        },
        RequestMessage::SubscribeEvents { key } => {
            let msg = SubscribeMessage {
                key,
                sub_response_tx,
            };
            sub_tx.send(msg).await;
            ResponseMessage::Subscribed
        },
    }
}

async fn handle_connection(raw_stream: TcpStream, addr: SocketAddr, trees: Trees, sub_tx: Sender<SubscribeMessage>) {
    println!("Incoming TCP connection from: {}", addr);

    let ws_stream = tokio_tungstenite::accept_async(raw_stream)
        .await
        .expect("Error during the websocket handshake occurred");
    println!("WebSocket connection established: {}", addr);

    let (mut ws_sender, mut ws_receiver) = ws_stream.split();

    let (sub_events_tx, sub_events_rx) = mpsc::channel(100);

    loop {
        tokio::select! {
            Some(msg) = ws_receiver.next() => {
                let msg = msg.unwrap();
                if msg.is_text() || msg.is_binary() {
                    match serde_json::from_str(msg.to_text().unwrap()) {
                        Ok(request_json) => {
                            let response_msg = process_msg(&trees, request_json, sub_tx.clone(), sub_events_tx.clone()).await;
                            let response_json = serde_json::to_string(&response_msg).unwrap();
                            ws_sender.send(tokio_tungstenite::tungstenite::Message::Text(response_json)).await.unwrap();
                        },
                        Err(error) => println!("{}", error),
                    }
                }
            }
        }
    }
}

use tokio::sync::mpsc::Sender;

pub async fn websockets_listen(trees: Trees, tx: Sender<SubscribeMessage>) {
    let addr = "0.0.0.0:8172".to_string();

    // Create the event loop and TCP listener we'll accept connections on.
    let try_socket = TcpListener::bind(&addr).await;
    let listener = try_socket.expect("Failed to bind");
    println!("Listening on: {}", addr);

    // Let's spawn the handling of each connection in a separate task.
    while let Ok((stream, addr)) = listener.accept().await {
        tokio::spawn(handle_connection(stream, addr, trees.clone(), tx.clone()));
    }
}
