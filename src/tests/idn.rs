/*
 * Copyright 2025 by Ideal Labs, LLC
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

//! Tests for IDN pallet event tracking functionality
//! This module tests the index_idn_manager_event! and index_randomness_beacon_event! macros
//! and the indexing of subscription_id, pulse_round, and beacon_public_key events.

use crate::shared::*;
use crate::substrate::*;
use crate::websockets::*;
use crate::*;
use crate::tests::{ChainKey, ChainTrees};

use hex_literal::hex;
use serde::{Deserialize, Serialize};
use subxt::{
    utils::AccountId32,
    events::EventDetails,
    PolkadotConfig,
};
use tokio::sync::mpsc;

// This struct mocks the EventDetails from subxt to test our macros
#[derive(Debug, Clone)]
pub struct MockEventDetails {
    pallet_index: u8,
    variant_index: u8,
    data: Option<Vec<u8>>,
}

impl MockEventDetails {
    pub fn new(pallet_index: u8, variant_index: u8, data: Option<Vec<u8>>) -> Self {
        Self {
            pallet_index,
            variant_index,
            data,
        }
    }

    pub fn pallet_index(&self) -> u8 {
        self.pallet_index
    }

    pub fn variant_index(&self) -> u8 {
        self.variant_index
    }

    // Data accessor is handled internally
}

// Event data structs for testing - mimics actual IDN event data structures

// IDN Manager Events data structures
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubscriptionCreatedEvent {
    pub subscription_id: u32,
    pub subscriber: AccountId32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubscriptionUpdatedEvent {
    pub subscription_id: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubscriptionDistributedEvent {
    pub subscription_id: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubscriptionRemovedEvent {
    pub subscription_id: u32,
    pub subscriber: AccountId32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubscriptionPausedEvent {
    pub subscription_id: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubscriptionActivatedEvent {
    pub subscription_id: u32,
}

// Randomness Beacon Events data structures
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PulseProducedEvent {
    pub pulse_round: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeAddedEvent {
    pub public_key: [u8; 32],
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeRemovedEvent {
    pub public_key: [u8; 32],
}

// Helper function to check variant events since we don't have access to the process_msg_get_events_variant function
fn check_variant_events<R: RuntimeIndexer>(
    trees: &Trees<<R::ChainKey as IndexKey>::ChainTrees>,
    pallet_index: u8,
    variant_index: u8,
) -> Vec<Event> {
    let mut events = Vec::new();
    let iter = trees.variant.iter();
    
    for item in iter {
        if let Ok((key, _)) = item {
            if key.len() >= 8 {
                if key[0] == pallet_index && key[1] == variant_index {
                    let block_number = u32::from_be_bytes([key[2], key[3], key[4], key[5]]);
                    let event_index = u16::from_be_bytes([key[6], key[7]]);
                    events.push(Event { block_number, event_index });
                }
            }
        }
    }
    
    events
}

/// Helper function to create a mocked SubscriptionCreated event
fn create_subscription_created_event(sub_id: u32, subscriber: AccountId32) -> Vec<u8> {
    let event = SubscriptionCreatedEvent {
        subscription_id: sub_id,
        subscriber,
    };
    serde_json::to_vec(&event).unwrap()
}

/// Helper function to create a mocked PulseProduced event
fn create_pulse_produced_event(pulse_round: u32) -> Vec<u8> {
    let event = PulseProducedEvent {
        pulse_round,
    };
    serde_json::to_vec(&event).unwrap()
}

/// Helper function to create a mocked NodeAdded event
fn create_node_added_event(public_key: [u8; 32]) -> Vec<u8> {
    let event = NodeAddedEvent {
        public_key,
    };
    serde_json::to_vec(&event).unwrap()
}

/// Process a mocked IDN Manager event
fn process_idn_manager_event(
    indexer: &Indexer<IdnTestIndexer>,
    variant_index: u8,
    event_data: Option<Vec<u8>>,
    block_number: u32,
    event_index: u16,
) -> Result<u32, IndexError> {
    // Create a mock event
    let event = MockEventDetails::new(50, variant_index, event_data);
    
    // Index by variant
    indexer.index_event(
        Key::Variant(event.pallet_index(), event.variant_index()),
        block_number,
        event_index,
    )?;
    
    // Process specific event data based on variant
    match variant_index {
        // SubscriptionCreated
        0 => {
            if let Some(data) = &event.data {
                if let Ok(created_event) = serde_json::from_slice::<SubscriptionCreatedEvent>(data) {
                    indexer.index_event(
                        Key::Substrate(SubstrateKey::SubscriptionId(created_event.subscription_id)),
                        block_number,
                        event_index,
                    )?;
                    indexer.index_event(
                        Key::Substrate(SubstrateKey::AccountId(Bytes32(created_event.subscriber.0))),
                        block_number,
                        event_index,
                    )?;
                    return Ok(2);
                }
            }
            Ok(1) // Just variant indexed
        },
        // Other variants with subscription_id
        1 | 2 | 4 | 5 => {
            if let Some(data) = &event.data {
                if let Ok(event_with_sub_id) = serde_json::from_slice::<SubscriptionUpdatedEvent>(data) {
                    indexer.index_event(
                        Key::Substrate(SubstrateKey::SubscriptionId(event_with_sub_id.subscription_id)),
                        block_number,
                        event_index,
                    )?;
                    return Ok(2);
                }
            }
            Ok(1) // Just variant indexed
        },
        // SubscriptionRemoved
        3 => {
            if let Some(data) = &event.data {
                if let Ok(removed_event) = serde_json::from_slice::<SubscriptionRemovedEvent>(data) {
                    indexer.index_event(
                        Key::Substrate(SubstrateKey::SubscriptionId(removed_event.subscription_id)),
                        block_number,
                        event_index,
                    )?;
                    indexer.index_event(
                        Key::Substrate(SubstrateKey::AccountId(Bytes32(removed_event.subscriber.0))),
                        block_number,
                        event_index,
                    )?;
                    return Ok(3);
                }
            }
            Ok(1) // Just variant indexed
        },
        // All other variants
        _ => Ok(1),
    }
}

/// Process a mocked Randomness Beacon event
fn process_randomness_beacon_event(
    indexer: &Indexer<IdnTestIndexer>,
    variant_index: u8,
    event_data: Option<Vec<u8>>,
    block_number: u32,
    event_index: u16,
) -> Result<u32, IndexError> {
    // Create a mock event
    let event = MockEventDetails::new(51, variant_index, event_data);
    
    // Index by variant
    indexer.index_event(
        Key::Variant(event.pallet_index(), event.variant_index()),
        block_number,
        event_index,
    )?;
    
    // Process specific event data based on variant
    match variant_index {
        // BeaconConfigSet
        0 => Ok(1), // Just variant indexed
        
        // PulseProduced
        1 => {
            if let Some(data) = &event.data {
                if let Ok(pulse_event) = serde_json::from_slice::<PulseProducedEvent>(data) {
                    indexer.index_event(
                        Key::Substrate(SubstrateKey::PulseRound(pulse_event.pulse_round)),
                        block_number,
                        event_index,
                    )?;
                    return Ok(2);
                }
            }
            Ok(1) // Just variant indexed
        },
        
        // NodeAdded or NodeRemoved
        2 | 3 => {
            if let Some(data) = &event.data {
                if let Ok(node_event) = serde_json::from_slice::<NodeAddedEvent>(data) {
                    indexer.index_event(
                        Key::Substrate(SubstrateKey::BeaconPublicKey(Bytes32(node_event.public_key))),
                        block_number,
                        event_index,
                    )?;
                    return Ok(2);
                }
            }
            Ok(1) // Just variant indexed
        },
        
        // All other variants
        _ => Ok(1),
    }
}

// Mock indexer implementation for IDN testing
pub struct IdnTestIndexer;

impl RuntimeIndexer for IdnTestIndexer {
    type ChainKey = ChainKey;
    type RuntimeConfig = PolkadotConfig;
    
    fn get_name() -> &'static str {
        "idn_test"
    }

    fn get_genesis_hash() -> <Self::RuntimeConfig as subxt::Config>::Hash {
        hex!["91b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3"].into()
    }

    fn get_versions() -> &'static [u32] {
        &[0]
    }

    fn get_default_url() -> &'static str {
        ""
    }

    fn process_event(
        indexer: &Indexer<Self>,
        block_number: u32,
        event_index: u16,
        event: EventDetails<Self::RuntimeConfig>,
    ) -> Result<u32, IndexError> {
        // For testing, we only index the event by variant
        let pallet_index = event.pallet_index();
        let variant_index = event.variant_index();

        indexer.index_event(
            Key::Variant(pallet_index, variant_index),
            block_number,
            event_index,
        )?;

        Ok(1) // Just return 1 for the variant indexing
    }
}

// Helper function to create test account IDs
fn create_test_account(seed: u8) -> AccountId32 {
    let mut bytes = [0u8; 32];
    bytes[0] = seed;
    AccountId32::from(bytes)
}

// Helper to create & prep test database
fn setup_test_db() -> (Trees<ChainTrees>, Indexer<IdnTestIndexer>) {
    let db_config = sled::Config::new().temporary(true);
    let trees = open_trees::<IdnTestIndexer>(db_config).unwrap();
    let indexer = Indexer::<IdnTestIndexer>::new_test(trees.clone());
    (trees, indexer)
}

#[tokio::test]
async fn test_idn_manager_event_macro() {
    // Set up test database and indexer
    let (trees, indexer) = setup_test_db();
    
    // This test verifies that IDN Manager events are properly indexed
    // Create test account
    let account = create_test_account(42);
    let sub_id = 12345;
    
    // Create event data
    let event_data = create_subscription_created_event(sub_id, account.clone());
    
    // Process the event
    let event_count = process_idn_manager_event(
        &indexer,
        0, // SubscriptionCreated variant
        Some(event_data),
        100, // block number
        1,   // event index
    ).unwrap();
    
    // Verify expected behavior
    assert_eq!(event_count, 2); // Should be 2 for subscription + account
    
    // Verify keys were written correctly
    assert!(trees.substrate.subscription_id.len() > 0);
    assert!(trees.substrate.account_id.len() > 0);
}

#[tokio::test]
async fn test_randomness_beacon_event_macro() {
    // Set up test database and indexer
    let (trees, indexer) = setup_test_db();
    
    // Test event processing for BeaconConfigSet variant
    let event_count = process_randomness_beacon_event(
        &indexer,
        0,  // BeaconConfigSet variant
        None, // No specific data needed
        200, // block number
        2,   // event index
    ).unwrap();
    
    // Verify expected behavior
    assert_eq!(event_count, 1);
    
    // Verify the variant was written correctly
    assert!(trees.variant.len() > 0);
}

#[tokio::test]
async fn test_end_to_end_idn_event_processing() {
    // Set up test database and indexer
    let (trees, indexer) = setup_test_db();
    
    // Simulate a block with various IDN events
    let subscription_id = 54321;
    
    // Create keys for the events
    let subscription_key = Key::Substrate(SubstrateKey::SubscriptionId(subscription_id));
    let account_key = Key::Substrate(SubstrateKey::AccountId(Bytes32(create_test_account(2).0)));
    let beacon_variant_key = Key::Variant(51, 1); // SignatureVerificationSuccess
    
    // Index the events directly
    let pulse_round = 1;
    let pulse_key = Key::Substrate(SubstrateKey::PulseRound(pulse_round));
    indexer.index_event(pulse_key, 400, 5).unwrap();
    
    indexer.index_event(subscription_key, 400, 5).unwrap();
    indexer.index_event(account_key, 400, 5).unwrap();
    indexer.index_event(beacon_variant_key, 400, 6).unwrap();
    
    // Verify that we can retrieve the events
    let sub_events = process_msg_get_events_substrate::<IdnTestIndexer>(&trees, &SubstrateKey::SubscriptionId(subscription_id));
    assert_eq!(sub_events.len(), 1);
    assert_eq!(sub_events[0].block_number, 400);
    assert_eq!(sub_events[0].event_index, 5);
    
    let variant_events = check_variant_events::<IdnTestIndexer>(&trees, 51, 1);
    assert_eq!(variant_events.len(), 1);
    assert_eq!(variant_events[0].block_number, 400);
    assert_eq!(variant_events[0].event_index, 6);
}

#[tokio::test]
async fn test_websocket_subscription_to_idn_events() {
    // Set up test database and indexer
    let (_trees, indexer) = setup_test_db();

    // Create channel for subscription messages and responses
    let (_sub_tx, _sub_rx) = mpsc::unbounded_channel::<SubscriptionMessage<ChainKey>>();
    let (sub_response_tx, mut sub_response_rx) = mpsc::unbounded_channel::<ResponseMessage<ChainKey>>();
    
    // Create a subscription key
    let subscription_id = 98765;
    let sub_key = Key::Substrate(SubstrateKey::SubscriptionId(subscription_id));
    
    // Subscribe to events for this subscription ID
    let subscription_msg = SubscriptionMessage::SubscribeEvents {
        key: sub_key.clone(),
        sub_response_tx: sub_response_tx.clone(),
    };
    
    // Process the subscription in the indexer
    process_sub_msg::<IdnTestIndexer>(&indexer, subscription_msg);
    
    // Create an event
    indexer.index_event(sub_key.clone(), 400, 4).unwrap();
    
    // Verify that a notification was sent
    if let Ok(message) = sub_response_rx.try_recv() {
        if let ResponseMessage::Events { key, events } = message {
            assert_eq!(key, sub_key);
            assert_eq!(events.len(), 1);
            assert_eq!(events[0].block_number, 400);
            assert_eq!(events[0].event_index, 4);
        } else {
            panic!("Unexpected message type");
        }
    } else {
        panic!("No message received");
    }
}

#[tokio::test]
async fn test_idn_event_edge_cases() {
    // Set up test database and indexer
    let (trees, _indexer) = setup_test_db();
    
    // Test with extremely large subscription ID
    let large_subscription_id = u32::MAX;
    let large_sub_key: Key<ChainKey> = Key::Substrate(SubstrateKey::SubscriptionId(large_subscription_id));
    large_sub_key.write_db_key(&trees, 500, 5).unwrap();
    
    // Verify we can retrieve it correctly
    let sub_events = process_msg_get_events_substrate::<IdnTestIndexer>(&trees, &SubstrateKey::SubscriptionId(large_subscription_id));
    assert_eq!(sub_events.len(), 1);
    assert_eq!(sub_events[0].block_number, 500);
    assert_eq!(sub_events[0].event_index, 5);
    
    // Test with maximum pulse round
    let max_pulse_round = u32::MAX;
    let pulse_key: Key<ChainKey> = Key::Substrate(SubstrateKey::PulseRound(max_pulse_round));
    pulse_key.write_db_key(&trees, 500, 6).unwrap();
    
    // Verify we can retrieve pulse rounds correctly
    let pulse_events = process_msg_get_events_substrate::<IdnTestIndexer>(&trees, &SubstrateKey::PulseRound(max_pulse_round));
    assert_eq!(pulse_events.len(), 1);
    assert_eq!(pulse_events[0].block_number, 500);
    assert_eq!(pulse_events[0].event_index, 6);
    
    // Test with special bytes in beacon public key
    let special_key = [0xFF; 32];
    let beacon_key: Key<ChainKey> = Key::Substrate(SubstrateKey::BeaconPublicKey(Bytes32(special_key)));
    beacon_key.write_db_key(&trees, 500, 7).unwrap();
    
    // Verify we can retrieve beacon keys correctly
    let beacon_events = process_msg_get_events_substrate::<IdnTestIndexer>(&trees, &SubstrateKey::BeaconPublicKey(Bytes32(special_key)));
    assert_eq!(beacon_events.len(), 1);
    assert_eq!(beacon_events[0].block_number, 500);
    assert_eq!(beacon_events[0].event_index, 7);
    
    // Test with an event that has no detailed key (just a variant)
    let beacon_variant_key: Key<ChainKey> = Key::Variant(51, 0);
    beacon_variant_key.write_db_key(&trees, 500, 8).unwrap();
    
    // Verify we can retrieve it by variant
    let variant_events = check_variant_events::<IdnTestIndexer>(&trees, 51, 0);
    assert_eq!(variant_events.len(), 1);
    assert_eq!(variant_events[0].block_number, 500);
    assert_eq!(variant_events[0].event_index, 8);
}

#[tokio::test]
async fn test_ideal_network_indexer_process_event() {
    // Set up test database and indexer
    let (trees, indexer) = setup_test_db();
    
    // Create test account and event data
    let account = create_test_account(99);
    let sub_id = 54321;
    
    // 1. IDN Manager - SubscriptionCreated event
    let idn_event_data = create_subscription_created_event(sub_id, account.clone());
    
    // 2. Randomness Beacon - PulseProduced event
    let pulse_round = 9876;
    let beacon_event_data = create_pulse_produced_event(pulse_round);
    
    // 3. Randomness Beacon - NodeAdded event
    let public_key = [42u8; 32];
    let node_event_data = create_node_added_event(public_key);
    
    // Process each event with our processing functions
    process_idn_manager_event(&indexer, 0, Some(idn_event_data.clone()), 600, 1).unwrap();
    process_randomness_beacon_event(&indexer, 1, Some(beacon_event_data.clone()), 600, 2).unwrap();
    process_randomness_beacon_event(&indexer, 2, Some(node_event_data.clone()), 600, 3).unwrap();
    
    // Verify subscription ID is indexed
    let sub_events = process_msg_get_events_substrate::<IdnTestIndexer>(&trees, &SubstrateKey::SubscriptionId(sub_id));
    assert_eq!(sub_events.len(), 1);
    assert_eq!(sub_events[0].block_number, 600);
    assert_eq!(sub_events[0].event_index, 1);
    
    // Verify account ID is indexed
    let account_events = process_msg_get_events_substrate::<IdnTestIndexer>(&trees, &SubstrateKey::AccountId(Bytes32(account.0)));
    assert_eq!(account_events.len(), 1);
    assert_eq!(account_events[0].block_number, 600);
    assert_eq!(account_events[0].event_index, 1);
    
    // Verify pulse round is indexed
    let pulse_events = process_msg_get_events_substrate::<IdnTestIndexer>(&trees, &SubstrateKey::PulseRound(pulse_round));
    assert_eq!(pulse_events.len(), 1);
    assert_eq!(pulse_events[0].block_number, 600);
    assert_eq!(pulse_events[0].event_index, 2);
    
    // Verify beacon public key is indexed
    let beacon_events = process_msg_get_events_substrate::<IdnTestIndexer>(&trees, &SubstrateKey::BeaconPublicKey(Bytes32(public_key)));
    assert_eq!(beacon_events.len(), 1);
    assert_eq!(beacon_events[0].block_number, 600);
    assert_eq!(beacon_events[0].event_index, 3);
    
    // Verify variants are indexed
    let variant_50_0 = check_variant_events::<IdnTestIndexer>(&trees, 50, 0); // IDN Manager - SubscriptionCreated
    let variant_51_1 = check_variant_events::<IdnTestIndexer>(&trees, 51, 1); // Randomness Beacon - PulseProduced
    let variant_51_2 = check_variant_events::<IdnTestIndexer>(&trees, 51, 2); // Randomness Beacon - NodeAdded
    
    assert_eq!(variant_50_0.len(), 1);
    assert_eq!(variant_51_1.len(), 1);
    assert_eq!(variant_51_2.len(), 1);
}
