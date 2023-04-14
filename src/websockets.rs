use std::{
    net::SocketAddr,
};
use parity_scale_codec::Decode;
use serde::{Serialize, Deserialize};
use tokio::net::{TcpListener, TcpStream};
use futures::{StreamExt, SinkExt};
use subxt::utils::AccountId32;

use crate::shared::*;

#[derive(Deserialize, Debug, Clone)]
#[serde(tag = "type")]
pub enum RequestMessage {
    GetStatus,
    GetEventsByAccountId {
        account_id: AccountId32,
    },
    GetEventsByAccountIndex {
        account_index: u32,
    },
    GetEventsByAuctionIndex {
        auction_index: u32,
    },
    GetEventsByBountyIndex {
        bounty_index: u32,
    },
    GetEventsByCandidateHash {
        candidate_hash: String,
    },
    GetEventsByMessageId {
        message_id: String,
    },
    GetEventsByParaId {
        para_id: u32,
    },
    GetEventsByPoolId {
        pool_id: u32,
    },
    GetEventsByProposalHash {
        proposal_hash: String,
    },
    GetEventsByProposalIndex {
        proposal_index: u32,
    },
    GetEventsByRefIndex {
        ref_index: u32,
    },
    GetEventsByRegistrarIndex {
        registrar_index: u32,
    },
    GetEventsByTipHash {
        tip_hash: String,
    },
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct EventFull {
    block_number: u32,
    event: Event,
}

#[derive(Serialize)]
#[serde(tag = "type", content = "data")]
#[serde(rename_all = "camelCase")]
enum ResponseMessage {
    Status {
        last_block: u32,
    },
    Events {
        events: Vec<EventFull>,
    },
    Error,
}

async fn process_msg(trees: &Trees, msg: RequestMessage) -> ResponseMessage {
    println!("msg: {:?}", msg);

    match msg {
        RequestMessage::GetStatus => {
            ResponseMessage::Status {
                last_block: match trees.root.get("last_block").unwrap() {
                    Some(value) => u32::from_be_bytes(vector_as_u8_4_array(&value.to_vec())),
                    None => 0,
                }
            }
        },
        RequestMessage::GetEventsByAccountId { account_id } => {
            let mut events = Vec::new();

            for kv in trees.account_id.scan_prefix(account_id) {
                let kv = kv.unwrap();
                let key = AccountIdKey::unserialize(kv.0.to_vec());
                let event = Event::decode(&mut kv.1.as_ref()).unwrap();

                events.push(EventFull {
                    block_number: key.block_number,
                    event: event,
                });
            }

            ResponseMessage::Events { events: events }
        },
        RequestMessage::GetEventsByAccountIndex { account_index } => {
            let mut events = Vec::new();

            for kv in trees.account_index.scan_prefix(account_index.to_be_bytes().to_vec()) {
                let kv = kv.unwrap();
                let key = AccountIndexKey::unserialize(kv.0.to_vec());
                let event = Event::decode(&mut kv.1.as_ref()).unwrap();

                events.push(EventFull {
                    block_number: key.block_number,
                    event: event,
                });
            }

            ResponseMessage::Events { events: events }
        },
        RequestMessage::GetEventsByAuctionIndex { auction_index } => {
            let mut events = Vec::new();

            for kv in trees.auction_index.scan_prefix(auction_index.to_be_bytes().to_vec()) {
                let kv = kv.unwrap();
                let key = AuctionIndexKey::unserialize(kv.0.to_vec());
                let event = Event::decode(&mut kv.1.as_ref()).unwrap();

                events.push(EventFull {
                    block_number: key.block_number,
                    event: event,
                });
            }

            ResponseMessage::Events { events: events }
        },
        RequestMessage::GetEventsByBountyIndex { bounty_index } => {
            let mut events = Vec::new();

            for kv in trees.bounty_index.scan_prefix(bounty_index.to_be_bytes().to_vec()) {
                let kv = kv.unwrap();
                let key = BountyIndexKey::unserialize(kv.0.to_vec());
                let event = Event::decode(&mut kv.1.as_ref()).unwrap();

                events.push(EventFull {
                    block_number: key.block_number,
                    event: event,
                });
            }

            ResponseMessage::Events { events: events }
        },
        RequestMessage::GetEventsByCandidateHash { candidate_hash } => {
            match candidate_hash.get(2..66) {
                Some(candidate_hash) => match hex::decode(candidate_hash) {
                    Ok(candidate_hash) => {
                        let mut events = Vec::new();

                        for kv in trees.candidate_hash.scan_prefix(candidate_hash.to_vec()) {
                            let kv = kv.unwrap();
                            let key = CandidateHashKey::unserialize(kv.0.to_vec());
                            let event = Event::decode(&mut kv.1.as_ref()).unwrap();

                            events.push(EventFull {
                                block_number: key.block_number,
                                event: event,
                            });
                        }
                        ResponseMessage::Events { events: events }
                    },
                    Err(_) => ResponseMessage::Error,
                },
                None => ResponseMessage::Error,
            }
        },
        RequestMessage::GetEventsByMessageId { message_id } => {
            match message_id.get(2..66) {
                Some(message_id) => match hex::decode(message_id) {
                    Ok(message_id) => {
                        let mut events = Vec::new();

                        for kv in trees.message_id.scan_prefix(message_id.to_vec()) {
                            let kv = kv.unwrap();
                            let key = MessageIdKey::unserialize(kv.0.to_vec());
                            let event = Event::decode(&mut kv.1.as_ref()).unwrap();

                            events.push(EventFull {
                                block_number: key.block_number,
                                event: event,
                            });
                        }
                        ResponseMessage::Events { events: events }
                    },
                    Err(_) => ResponseMessage::Error,
                },
                None => ResponseMessage::Error,
            }
        },
        RequestMessage::GetEventsByParaId { para_id } => {
            let mut events = Vec::new();

            for kv in trees.para_id.scan_prefix(para_id.to_be_bytes().to_vec()) {
                let kv = kv.unwrap();
                let key = ParaIdKey::unserialize(kv.0.to_vec());
                let event = Event::decode(&mut kv.1.as_ref()).unwrap();

                events.push(EventFull {
                    block_number: key.block_number,
                    event: event,
                });
            }

            ResponseMessage::Events { events: events }
        },
        RequestMessage::GetEventsByPoolId { pool_id } => {
            let mut events = Vec::new();

            for kv in trees.pool_id.scan_prefix(pool_id.to_be_bytes().to_vec()) {
                let kv = kv.unwrap();
                let key = PoolIdKey::unserialize(kv.0.to_vec());
                let event = Event::decode(&mut kv.1.as_ref()).unwrap();

                events.push(EventFull {
                    block_number: key.block_number,
                    event: event,
                });
            }

            ResponseMessage::Events { events: events }
        },
        RequestMessage::GetEventsByProposalHash { proposal_hash } => {
            match proposal_hash.get(2..66) {
                Some(proposal_hash) => match hex::decode(proposal_hash) {
                    Ok(proposal_hash) => {
                        let mut events = Vec::new();

                        for kv in trees.proposal_hash.scan_prefix(proposal_hash.to_vec()) {
                            let kv = kv.unwrap();
                            let key = ProposalHashKey::unserialize(kv.0.to_vec());
                            let event = Event::decode(&mut kv.1.as_ref()).unwrap();

                            events.push(EventFull {
                                block_number: key.block_number,
                                event: event,
                            });
                        }
                        ResponseMessage::Events { events: events }
                    },
                    Err(_) => ResponseMessage::Error,
                },
                None => ResponseMessage::Error,
            }
        },
        RequestMessage::GetEventsByProposalIndex { proposal_index } => {
            let mut events = Vec::new();

            for kv in trees.proposal_index.scan_prefix(proposal_index.to_be_bytes().to_vec()) {
                let kv = kv.unwrap();
                let key = ProposalIndexKey::unserialize(kv.0.to_vec());
                let event = Event::decode(&mut kv.1.as_ref()).unwrap();

                events.push(EventFull {
                    block_number: key.block_number,
                    event: event,
                });
            }

            ResponseMessage::Events { events: events }
        },
        RequestMessage::GetEventsByRefIndex { ref_index } => {
            let mut events = Vec::new();

            for kv in trees.ref_index.scan_prefix(ref_index.to_be_bytes().to_vec()) {
                let kv = kv.unwrap();
                let key = RefIndexKey::unserialize(kv.0.to_vec());
                let event = Event::decode(&mut kv.1.as_ref()).unwrap();

                events.push(EventFull {
                    block_number: key.block_number,
                    event: event,
                });
            }

            ResponseMessage::Events { events: events }
        },
        RequestMessage::GetEventsByRegistrarIndex { registrar_index } => {
            let mut events = Vec::new();

            for kv in trees.registrar_index.scan_prefix(registrar_index.to_be_bytes().to_vec()) {
                let kv = kv.unwrap();
                let key = RegistrarIndexKey::unserialize(kv.0.to_vec());
                let event = Event::decode(&mut kv.1.as_ref()).unwrap();

                events.push(EventFull {
                    block_number: key.block_number,
                    event: event,
                });
            }

            ResponseMessage::Events { events: events }
        },
        RequestMessage::GetEventsByTipHash { tip_hash } => {
            match tip_hash.get(2..66) {
                Some(tip_hash) => match hex::decode(tip_hash) {
                    Ok(tip_hash) => {
                        let mut events = Vec::new();

                        for kv in trees.tip_hash.scan_prefix(tip_hash.to_vec()) {
                            let kv = kv.unwrap();
                            let key = TipHashKey::unserialize(kv.0.to_vec());
                            let event = Event::decode(&mut kv.1.as_ref()).unwrap();

                            events.push(EventFull {
                                block_number: key.block_number,
                                event: event,
                            });
                        }
                        ResponseMessage::Events { events: events }
                    },
                    Err(_) => ResponseMessage::Error,
                },
                None => ResponseMessage::Error,
            }
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
                    match serde_json::from_str(msg.to_text().unwrap()) {
                        Ok(request_json) => {
                            let response_msg = process_msg(&trees, request_json).await;
                            let response_json = serde_json::to_string(&response_msg).unwrap();
                            ws_sender.send(tokio_tungstenite::tungstenite::Message::Text(response_json)).await.unwrap();
                        },
                        Err(_) => {},
                    }
                }
            }
        }
    }
}


pub async fn websockets_listen(trees: Trees) {
    let addr = "0.0.0.0:8080".to_string();

    // Create the event loop and TCP listener we'll accept connections on.
    let try_socket = TcpListener::bind(&addr).await;
    let listener = try_socket.expect("Failed to bind");
    println!("Listening on: {}", addr);

    // Let's spawn the handling of each connection in a separate task.
    while let Ok((stream, addr)) = listener.accept().await {
        tokio::spawn(handle_connection(stream, addr, trees.clone()));
    }
}
