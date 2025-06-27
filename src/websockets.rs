use crate::shared::*;

use futures::{SinkExt, StreamExt};
use sled::Tree;
use std::net::SocketAddr;
use subxt::backend::legacy::LegacyRpcMethods;
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::{
    mpsc::{unbounded_channel, UnboundedSender},
    watch::Receiver,
};
use tokio_tungstenite::tungstenite;
use tracing::{error, info};
use zerocopy::FromBytes;

pub fn process_msg_status<R: RuntimeIndexer>(span_db: &Tree) -> ResponseMessage<R::ChainKey> {
    let mut spans = vec![];
    for (key, value) in span_db.into_iter().flatten() {
        let span_value = SpanDbValue::read_from(&value).unwrap();
        let start: u32 = span_value.start.into();
        let end: u32 = u32::from_be_bytes(key.as_ref().try_into().unwrap());
        let span = Span { start, end };
        spans.push(span);
    }
    ResponseMessage::Status(spans)
}

pub fn process_msg_subscribe_status<R: RuntimeIndexer>(
    sub_tx: &UnboundedSender<SubscriptionMessage<R::ChainKey>>,
    sub_response_tx: &UnboundedSender<ResponseMessage<R::ChainKey>>,
) -> ResponseMessage<R::ChainKey> {
    let msg = SubscriptionMessage::SubscribeStatus {
        sub_response_tx: sub_response_tx.clone(),
    };
    sub_tx.send(msg).unwrap();
    ResponseMessage::Subscribed
}

pub fn process_msg_unsubscribe_status<R: RuntimeIndexer>(
    sub_tx: &UnboundedSender<SubscriptionMessage<R::ChainKey>>,
    sub_response_tx: &UnboundedSender<ResponseMessage<R::ChainKey>>,
) -> ResponseMessage<R::ChainKey> {
    let msg = SubscriptionMessage::UnsubscribeStatus {
        sub_response_tx: sub_response_tx.clone(),
    };
    sub_tx.send(msg).unwrap();
    ResponseMessage::Unsubscribed
}

pub async fn process_msg_variants<R: RuntimeIndexer>(
    rpc: &LegacyRpcMethods<R::RuntimeConfig>,
) -> Result<ResponseMessage<R::ChainKey>, IndexError> {
    let metadata = rpc.state_get_metadata(None).await?;
    let mut pallets = Vec::new();

    for pallet in metadata.pallets() {
        let mut pallet_meta = PalletMeta {
            index: pallet.index(),
            name: pallet.name().to_owned(),
            events: Vec::new(),
        };

        if let Some(variants) = pallet.event_variants() {
            for variant in variants {
                pallet_meta.events.push(EventMeta {
                    index: variant.index,
                    name: variant.name.clone(),
                })
            }
            pallets.push(pallet_meta);
        }
    }
    Ok(ResponseMessage::Variants(pallets))
}

pub fn get_events_variant(tree: &Tree, pallet_id: u8, variant_id: u8) -> Vec<Event> {
    get_events_variant_with_limit(tree, pallet_id, variant_id, Some(100)).0
}

/// Get events by variant with optional limit (None = unlimited)
/// Returns (events, has_more) tuple
pub fn get_events_variant_with_limit(tree: &Tree, pallet_id: u8, variant_id: u8, limit: Option<usize>) -> (Vec<Event>, bool) {
    let mut events = Vec::new();
    let mut iter = tree.scan_prefix([pallet_id, variant_id]).keys();
    let mut has_more = false;

    while let Some(Ok(key)) = iter.next_back() {
        let key = VariantKey::read_from(&key).unwrap();

        // Check if we've reached the limit before adding the event
        if let Some(limit) = limit {
            if events.len() == limit {
                has_more = true; // There's at least one more event
                break;
            }
        }

        events.push(Event {
            block_number: key.block_number.into(),
            event_index: key.event_index.into(),
        });
    }
    (events, has_more)
}

pub fn get_events_bytes32(tree: &Tree, key: &Bytes32) -> Vec<Event> {
    get_events_bytes32_with_limit(tree, key, Some(100)).0
}

/// Get events by bytes32 key with optional limit (None = unlimited)
/// Returns (events, has_more) tuple
pub fn get_events_bytes32_with_limit(tree: &Tree, key: &Bytes32, limit: Option<usize>) -> (Vec<Event>, bool) {
    let mut events = Vec::new();
    let mut iter = tree.scan_prefix(&key.0).keys();
    let mut has_more = false;

    while let Some(Ok(key)) = iter.next_back() {
        let key = Bytes32Key::read_from(&key).unwrap();

        // Check if we've reached the limit before adding the event
        if let Some(limit) = limit {
            if events.len() == limit {
                has_more = true; // There's at least one more event
                break;
            }
        }

        events.push(Event {
            block_number: key.block_number.into(),
            event_index: key.event_index.into(),
        });
    }
    (events, has_more)
}

pub fn get_events_u32(tree: &Tree, key: u32) -> Vec<Event> {
    get_events_u32_with_limit(tree, key, Some(100)).0
}

/// Get events by u32 key with optional limit (None = unlimited)
/// Returns (events, has_more) tuple
pub fn get_events_u32_with_limit(tree: &Tree, key: u32, limit: Option<usize>) -> (Vec<Event>, bool) {
    let mut events = Vec::new();
    let mut iter = tree.scan_prefix(key.to_be_bytes()).keys();
    let mut has_more = false;

    while let Some(Ok(key)) = iter.next_back() {
        let key = U32Key::read_from(&key).unwrap();

        // Check if we've reached the limit before adding the event
        if let Some(limit) = limit {
            if events.len() == limit {
                has_more = true; // There's at least one more event
                break;
            }
        }

        events.push(Event {
            block_number: key.block_number.into(),
            event_index: key.event_index.into(),
        });
    }
    (events, has_more)
}

pub fn process_msg_get_events_substrate<R: RuntimeIndexer>(
    trees: &Trees<<R::ChainKey as IndexKey>::ChainTrees>,
    key: &SubstrateKey,
) -> Vec<Event> {
    process_msg_get_events_substrate_with_limit::<R>(trees, key, Some(100)).0
}

/// Get substrate events with optional limit (None = unlimited)
/// Returns (events, has_more) tuple
pub fn process_msg_get_events_substrate_with_limit<R: RuntimeIndexer>(
    trees: &Trees<<R::ChainKey as IndexKey>::ChainTrees>,
    key: &SubstrateKey,
    limit: Option<usize>,
) -> (Vec<Event>, bool) {
    match key {
        SubstrateKey::AccountId(account_id) => {
            get_events_bytes32_with_limit(&trees.substrate.account_id, account_id, limit)
        }
        SubstrateKey::AccountIndex(account_index) => {
            get_events_u32_with_limit(&trees.substrate.account_index, *account_index, limit)
        }
        SubstrateKey::BountyIndex(bounty_index) => {
            get_events_u32_with_limit(&trees.substrate.bounty_index, *bounty_index, limit)
        }
        SubstrateKey::EraIndex(era_index) => get_events_u32_with_limit(&trees.substrate.era_index, *era_index, limit),
        SubstrateKey::MessageId(message_id) => {
            get_events_bytes32_with_limit(&trees.substrate.message_id, message_id, limit)
        }
        SubstrateKey::PoolId(pool_id) => get_events_u32_with_limit(&trees.substrate.pool_id, *pool_id, limit),
        SubstrateKey::PreimageHash(preimage_hash) => {
            get_events_bytes32_with_limit(&trees.substrate.preimage_hash, preimage_hash, limit)
        }
        SubstrateKey::ProposalHash(proposal_hash) => {
            get_events_bytes32_with_limit(&trees.substrate.proposal_hash, proposal_hash, limit)
        }
        SubstrateKey::ProposalIndex(proposal_index) => {
            get_events_u32_with_limit(&trees.substrate.proposal_index, *proposal_index, limit)
        }
        SubstrateKey::RefIndex(ref_index) => get_events_u32_with_limit(&trees.substrate.ref_index, *ref_index, limit),
        SubstrateKey::RegistrarIndex(registrar_index) => {
            get_events_u32_with_limit(&trees.substrate.registrar_index, *registrar_index, limit)
        }
        SubstrateKey::SessionIndex(session_index) => {
            get_events_u32_with_limit(&trees.substrate.session_index, *session_index, limit)
        }
        SubstrateKey::TipHash(tip_hash) => get_events_bytes32_with_limit(&trees.substrate.tip_hash, tip_hash, limit),
        // Handle Ideal Network specific keys
        SubstrateKey::SubscriptionId(subscription_id) => {
            get_events_bytes32_with_limit(&trees.substrate.subscription_id, &subscription_id.0, limit)
        }
    }
}

pub fn process_msg_get_events<R: RuntimeIndexer>(
    trees: &Trees<<R::ChainKey as IndexKey>::ChainTrees>,
    key: Key<R::ChainKey>,
) -> ResponseMessage<R::ChainKey> {
    let events = match key {
        Key::Variant(pallet_id, variant_id) => {
            get_events_variant(&trees.variant, pallet_id, variant_id)
        }
        Key::Substrate(ref key) => process_msg_get_events_substrate::<R>(trees, key),
        Key::Chain(ref key) => key.get_key_events(&trees.chain),
    };
    ResponseMessage::Events { key, events }
}

/// Process GetEventsWithLimit requests with pagination support
pub fn process_msg_get_events_with_limit<R: RuntimeIndexer>(
    trees: &Trees<<R::ChainKey as IndexKey>::ChainTrees>,
    key: Key<R::ChainKey>,
    limit: Option<usize>,
) -> ResponseMessage<R::ChainKey> {
    let (events, has_more) = match key {
        Key::Variant(pallet_id, variant_id) => {
            get_events_variant_with_limit(&trees.variant, pallet_id, variant_id, limit)
        }
        Key::Substrate(ref substrate_key) => process_msg_get_events_substrate_with_limit::<R>(trees, substrate_key, limit),
        Key::Chain(ref chain_key) => {
            // For chain-specific keys, we'd need to add similar limit support
            // For now, fall back to existing implementation without limit tracking
            let events = chain_key.get_key_events(&trees.chain);
            (events, false) // Assume no more events for now
        }
    };
    
    let total_returned = events.len();
    ResponseMessage::EventsWithLimit { 
        key, 
        events, 
        has_more, 
        total_returned 
    }
}

pub fn process_msg_subscribe_events<R: RuntimeIndexer>(
    key: Key<R::ChainKey>,
    sub_tx: &UnboundedSender<SubscriptionMessage<R::ChainKey>>,
    sub_response_tx: &UnboundedSender<ResponseMessage<R::ChainKey>>,
) -> ResponseMessage<R::ChainKey> {
    let msg = SubscriptionMessage::SubscribeEvents {
        key,
        sub_response_tx: sub_response_tx.clone(),
    };
    sub_tx.send(msg).unwrap();
    ResponseMessage::Subscribed
}

pub fn process_msg_unsubscribe_events<R: RuntimeIndexer>(
    key: Key<R::ChainKey>,
    sub_tx: &UnboundedSender<SubscriptionMessage<R::ChainKey>>,
    sub_response_tx: &UnboundedSender<ResponseMessage<R::ChainKey>>,
) -> ResponseMessage<R::ChainKey> {
    let msg = SubscriptionMessage::UnsubscribeEvents {
        key,
        sub_response_tx: sub_response_tx.clone(),
    };
    sub_tx.send(msg).unwrap();
    ResponseMessage::Unsubscribed
}

pub async fn process_msg<R: RuntimeIndexer>(
    rpc: &LegacyRpcMethods<R::RuntimeConfig>,
    trees: &Trees<<R::ChainKey as IndexKey>::ChainTrees>,
    msg: RequestMessage<R::ChainKey>,
    sub_tx: &UnboundedSender<SubscriptionMessage<R::ChainKey>>,
    sub_response_tx: &UnboundedSender<ResponseMessage<R::ChainKey>>,
) -> Result<ResponseMessage<R::ChainKey>, IndexError> {
    Ok(match msg {
        RequestMessage::Status => process_msg_status::<R>(&trees.span),
        RequestMessage::SubscribeStatus => {
            process_msg_subscribe_status::<R>(sub_tx, sub_response_tx)
        }
        RequestMessage::UnsubscribeStatus => {
            process_msg_unsubscribe_status::<R>(sub_tx, sub_response_tx)
        }
        RequestMessage::Variants => process_msg_variants::<R>(rpc).await?,
        RequestMessage::GetEvents { key } => {
            // Note: Subscription ID hex parsing may not be needed if the client already sends proper format
            // If you need hex string parsing, ensure the client sends the subscription ID in the correct format
            process_msg_get_events::<R>(trees, key)
        },
        RequestMessage::GetEventsWithLimit { key, limit } => {
            process_msg_get_events_with_limit::<R>(trees, key, limit)
        },
        RequestMessage::SubscribeEvents { key } => {
            process_msg_subscribe_events::<R>(key, sub_tx, sub_response_tx)
        }
        RequestMessage::UnsubscribeEvents { key } => {
            process_msg_unsubscribe_events::<R>(key, sub_tx, sub_response_tx)
        }
        RequestMessage::SizeOnDisk => ResponseMessage::SizeOnDisk(trees.root.size_on_disk()?),
    })
}

async fn handle_connection<R: RuntimeIndexer>(
    rpc: LegacyRpcMethods<R::RuntimeConfig>,
    raw_stream: TcpStream,
    addr: SocketAddr,
    trees: Trees<<R::ChainKey as IndexKey>::ChainTrees>,
    sub_tx: UnboundedSender<SubscriptionMessage<R::ChainKey>>,
) -> Result<(), IndexError> {
    info!("Incoming TCP connection from: {}", addr);
    let ws_stream = tokio_tungstenite::accept_async(raw_stream).await?;
    info!("WebSocket connection established: {}", addr);

    let (mut ws_sender, mut ws_receiver) = ws_stream.split();
    // Create the channel for the substrate thread to send event messages to this thread.
    let (sub_events_tx, mut sub_events_rx) = unbounded_channel();

    loop {
        tokio::select! {
            Some(Ok(msg)) = ws_receiver.next() => {
                if msg.is_text() || msg.is_binary() {
                    match serde_json::from_str(msg.to_text()?) {
                        Ok(request_json) => {
                            let response_msg = process_msg::<R>(&rpc, &trees, request_json, &sub_tx, &sub_events_tx).await?;
                            let response_json = serde_json::to_string(&response_msg).unwrap();
                            ws_sender.send(tungstenite::Message::Text(response_json)).await?;
                        },
                        Err(error) => error!("{}", error),
                    }
                }
            },
            Some(msg) = sub_events_rx.recv() => {
                let response_json = serde_json::to_string(&msg).unwrap();
                ws_sender.send(tungstenite::Message::Text(response_json)).await?;
            },
        }
    }
}

pub async fn websockets_listen<R: RuntimeIndexer + 'static>(
    trees: Trees<<R::ChainKey as IndexKey>::ChainTrees>,
    rpc: LegacyRpcMethods<R::RuntimeConfig>,
    port: u16,
    mut exit_rx: Receiver<bool>,
    sub_tx: UnboundedSender<SubscriptionMessage<R::ChainKey>>,
) {
    let mut addr = "0.0.0.0:".to_string();
    addr.push_str(&port.to_string());

    // Create the event loop and TCP listener we'll accept connections on.
    let try_socket = TcpListener::bind(&addr).await;
    let listener = try_socket.expect("Failed to bind");
    info!("Listening on: {}", addr);

    // Let's spawn the handling of each connection in a separate task.
    loop {
        tokio::select! {
            biased;

            _ = exit_rx.changed() => {
                break;
            }
            Ok((stream, addr)) = listener.accept() => {
                tokio::spawn(handle_connection::<R>(
                    rpc.clone(),
                    stream,
                    addr,
                    trees.clone(),
                    sub_tx.clone(),
                ));
            }
        }
    }
}
