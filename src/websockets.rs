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
        candidate_hash: [u8; 32],
    },
    GetEventsByMessageId {
        message_id: [u8; 32],
    },
    GetEventsByParaId {
        para_id: u32,
    },
    GetEventsByPoolId {
        pool_id: u32,
    },
    GetEventsByProposalHash {
        proposal_hash: [u8; 32],
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
        tip_hash: [u8; 32],
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

    let events = match msg {
        RequestMessage::GetEventsByAccountId { account_id } => {
            println!("GetEventsByAccountId: {}", account_id);

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

            events
        },
        RequestMessage::GetEventsByAccountIndex { account_index } => {
            println!("GetEventsByAccountIndex: {}", account_index);

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

            events
        },
        RequestMessage::GetEventsByAuctionIndex { auction_index } => {
            println!("GetEventsByAuctionIndex: {}", auction_index);

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

            events
        },
        RequestMessage::GetEventsByBountyIndex { bounty_index } => {
            println!("GetEventsByBountyIndex: {}", bounty_index);

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

            events
        },
        RequestMessage::GetEventsByCandidateHash { candidate_hash } => {
//            println!("GetEventsByCandidateHash: {}", candidate_hash);

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

            events
        },
        RequestMessage::GetEventsByMessageId { message_id } => {
//            println!("GetEventsByMessageId: {}", message_id);

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

            events
        },
        RequestMessage::GetEventsByParaId { para_id } => {
            println!("GetEventsByParaId: {}", para_id);

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

            events
        },
        RequestMessage::GetEventsByPoolId { pool_id } => {
            println!("GetEventsByPoolId: {}", pool_id);

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

            events
        },
        RequestMessage::GetEventsByProposalHash { proposal_hash } => {
//            println!("GetEventsByProposalHash: {}", proposal_hash);

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

            events
        },
        RequestMessage::GetEventsByProposalIndex { proposal_index } => {
            println!("GetEventsByProposalIndex: {}", proposal_index);

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

            events
        },
        RequestMessage::GetEventsByRefIndex { ref_index } => {
            println!("GetEventsByRefIndex: {}", ref_index);

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

            events
        },
        RequestMessage::GetEventsByRegistrarIndex { registrar_index } => {
            println!("GetEventsByRegistrarIndex: {}", registrar_index);

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

            events
        },
        RequestMessage::GetEventsByTipHash { tip_hash } => {
//            println!("GetEventsByTipHash: {}", tip_hash);

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

            events
        },
    };

    let response_message = ResponseMessage {events: events};
    serde_json::to_string(&response_message).unwrap()
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
