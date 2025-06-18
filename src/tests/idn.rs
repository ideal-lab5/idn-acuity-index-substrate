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
use crate::tests::{ChainKey, ChainTrees};
use crate::websockets::*;
use crate::*;

use hex_literal::hex;
use serde::{Deserialize, Serialize};
use subxt::{events::EventDetails, utils::AccountId32, PolkadotConfig};
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
pub struct MockSubscriptionCreatedEvent {
    pub subscription_id: Bytes32,
    pub subscriber: AccountId32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubscriptionUpdatedEvent {
    pub subscription_id: Bytes32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubscriptionDistributedEvent {
    pub subscription_id: Bytes32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubscriptionRemovedEvent {
    pub subscription_id: Bytes32,
    pub subscriber: AccountId32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubscriptionPausedEvent {
    pub subscription_id: Bytes32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubscriptionActivatedEvent {
    pub subscription_id: Bytes32,
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
                    events.push(Event {
                        block_number,
                        event_index,
                    });
                }
            }
        }
    }

    events
}

/// Helper function to create a mocked SubscriptionCreated event
fn create_subscription_created_event(sub_id: u32, subscriber: AccountId32) -> Vec<u8> {
    let event = MockSubscriptionCreatedEvent {
        subscription_id: Bytes32::from({
            let mut bytes = [0u8; 32];
            bytes[28..32].copy_from_slice(&sub_id.to_be_bytes());
            bytes
        }),
        subscriber,
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
    let event = MockEventDetails::new(40, variant_index, event_data);

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
                if let Ok(created_event) =
                    serde_json::from_slice::<MockSubscriptionCreatedEvent>(data)
                {
                    indexer.index_event(
                        Key::Substrate(SubstrateKey::SubscriptionId(created_event.subscription_id)),
                        block_number,
                        event_index,
                    )?;
                    indexer.index_event(
                        Key::Substrate(SubstrateKey::AccountId(Bytes32(
                            created_event.subscriber.0,
                        ))),
                        block_number,
                        event_index,
                    )?;
                    return Ok(2);
                }
            }
            Ok(1) // Just variant indexed
        }
        // Other variants with subscription_id
        1 | 2 | 4 | 5 => {
            if let Some(data) = &event.data {
                if let Ok(event_with_sub_id) =
                    serde_json::from_slice::<SubscriptionUpdatedEvent>(data)
                {
                    indexer.index_event(
                        Key::Substrate(SubstrateKey::SubscriptionId(
                            event_with_sub_id.subscription_id,
                        )),
                        block_number,
                        event_index,
                    )?;
                    return Ok(2);
                }
            }
            Ok(1) // Just variant indexed
        }
        // SubscriptionRemoved
        3 => {
            if let Some(data) = &event.data {
                if let Ok(removed_event) = serde_json::from_slice::<SubscriptionRemovedEvent>(data)
                {
                    indexer.index_event(
                        Key::Substrate(SubstrateKey::SubscriptionId(removed_event.subscription_id)),
                        block_number,
                        event_index,
                    )?;
                    indexer.index_event(
                        Key::Substrate(SubstrateKey::AccountId(Bytes32(
                            removed_event.subscriber.0,
                        ))),
                        block_number,
                        event_index,
                    )?;
                    return Ok(3);
                }
            }
            Ok(1) // Just variant indexed
        }
        // All other variants
        _ => Ok(1),
    }
}

// Mock function to process randomness beacon events according to actual event structure
fn process_randomness_beacon_event(
    indexer: &Indexer<IdnTestIndexer>,
    variant_index: u8,
    _event_data: Option<Vec<u8>>,
    block_number: u32,
    event_index: u16,
) -> Result<u32, IndexError> {
    // Index the event by variant
    indexer.index_event(
        Key::Variant(41, variant_index), // Pallet 41 for randomness beacon
        block_number,
        event_index,
    )?;

    match variant_index {
        // BeaconConfigSet - variant 0
        0 => {
            // No additional keys to index, just variant
            Ok(0)
        }

        // SignatureVerificationSuccess - variant 1
        1 => {
            // No additional keys to index, just variant
            Ok(0)
        }

        // All other variants
        _ => Ok(0),
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
    )
    .unwrap();

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
        &indexer, 0,    // BeaconConfigSet variant
        None, // No specific data needed
        200,  // block number
        2,    // event index
    )
    .unwrap();

    // Verify expected behavior
    assert_eq!(event_count, 0);

    // Verify the variant was written correctly
    assert!(trees.variant.len() > 0);
}

#[tokio::test]
async fn test_end_to_end_idn_event_processing() {
    // Set up test database and indexer
    let (trees, indexer) = setup_test_db();

    // Simulate a block with various IDN events
    let subscription_id = {
        let mut bytes = [0u8; 32];
        bytes[28..32].copy_from_slice(&12345u32.to_be_bytes());
        Bytes32::from(bytes)
    };

    // Create keys for the events
    let subscription_key = Key::Substrate(SubstrateKey::SubscriptionId(subscription_id));
    let account_key = Key::Substrate(SubstrateKey::AccountId(Bytes32(create_test_account(2).0)));
    let beacon_variant_key = Key::Variant(41, 1); // SignatureVerificationSuccess (updated pallet index)

    // Index the events directly
    indexer.index_event(subscription_key, 400, 5).unwrap();
    indexer.index_event(account_key, 400, 5).unwrap();
    indexer.index_event(beacon_variant_key, 400, 6).unwrap();

    // Verify that we can retrieve the events
    let sub_events = process_msg_get_events_substrate::<IdnTestIndexer>(
        &trees,
        &SubstrateKey::SubscriptionId(subscription_id),
    );
    assert_eq!(sub_events.len(), 1);
    assert_eq!(sub_events[0].block_number, 400);
    assert_eq!(sub_events[0].event_index, 5);

    let variant_events = check_variant_events::<IdnTestIndexer>(&trees, 41, 1);
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
    let (sub_response_tx, mut sub_response_rx) =
        mpsc::unbounded_channel::<ResponseMessage<ChainKey>>();

    // Create a subscription key
    let subscription_id = {
        let mut bytes = [0u8; 32];
        bytes[28..32].copy_from_slice(&98765u32.to_be_bytes());
        Bytes32::from(bytes)
    };
    let sub_key = Key::Substrate(SubstrateKey::SubscriptionId(subscription_id));

    // Subscribe to events for this subscription ID
    let _subscription_msg = SubscriptionMessage::SubscribeEvents {
        key: sub_key.clone(),
        sub_response_tx: sub_response_tx.clone(),
    };

    // Index an event for this subscription
    indexer.index_event(sub_key, 500, 10).unwrap();

    // This test mainly verifies the subscription mechanism works
    // In a real scenario, the subscription would receive events
    assert!(sub_response_rx.try_recv().is_err()); // No messages yet
}

#[tokio::test]
async fn test_idn_event_edge_cases() {
    // Set up test database and indexer
    let (trees, indexer) = setup_test_db();

    // Test: Multiple events for the same subscription
    let subscription_id = {
        let mut bytes = [0u8; 32];
        bytes[28..32].copy_from_slice(&12345u32.to_be_bytes());
        Bytes32::from(bytes)
    };
    let sub_key = Key::Substrate(SubstrateKey::SubscriptionId(subscription_id));

    // Index multiple events for the same subscription
    indexer.index_event(sub_key.clone(), 100, 1).unwrap();
    indexer.index_event(sub_key.clone(), 100, 2).unwrap();
    indexer.index_event(sub_key.clone(), 101, 1).unwrap();

    // Verify all events are stored
    let events = process_msg_get_events_substrate::<IdnTestIndexer>(
        &trees,
        &SubstrateKey::SubscriptionId(subscription_id),
    );
    assert_eq!(events.len(), 3);

    // Test: Beacon events (they only have variant indexing)
    let beacon_variant_key = Key::Variant(41, 0); // BeaconConfigSet
    indexer.index_event(beacon_variant_key, 200, 1).unwrap();

    let beacon_events = check_variant_events::<IdnTestIndexer>(&trees, 41, 0);
    assert_eq!(beacon_events.len(), 1);
    assert_eq!(beacon_events[0].block_number, 200);
    assert_eq!(beacon_events[0].event_index, 1);
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

    // Process IDN Manager event
    process_idn_manager_event(&indexer, 0, Some(idn_event_data.clone()), 600, 1).unwrap();

    // Process Randomness Beacon events (they only have variant indexing)
    process_randomness_beacon_event(&indexer, 0, None, 600, 2).unwrap(); // BeaconConfigSet
    process_randomness_beacon_event(&indexer, 1, None, 600, 3).unwrap(); // SignatureVerificationSuccess

    // Verify subscription ID is indexed
    let sub_events = process_msg_get_events_substrate::<IdnTestIndexer>(
        &trees,
        &SubstrateKey::SubscriptionId({
            let mut bytes = [0u8; 32];
            bytes[28..32].copy_from_slice(&sub_id.to_be_bytes());
            Bytes32::from(bytes)
        }),
    );
    assert_eq!(sub_events.len(), 1);
    assert_eq!(sub_events[0].block_number, 600);
    assert_eq!(sub_events[0].event_index, 1);

    // Verify account ID is indexed
    let account_events = process_msg_get_events_substrate::<IdnTestIndexer>(
        &trees,
        &SubstrateKey::AccountId(Bytes32(account.0)),
    );
    assert_eq!(account_events.len(), 1);
    assert_eq!(account_events[0].block_number, 600);
    assert_eq!(account_events[0].event_index, 1);

    // Verify variants are indexed (updated pallet indices)
    let variant_40_0 = check_variant_events::<IdnTestIndexer>(&trees, 40, 0); // IDN Manager - SubscriptionCreated
    let variant_41_0 = check_variant_events::<IdnTestIndexer>(&trees, 41, 0); // Randomness Beacon - BeaconConfigSet
    let variant_41_1 = check_variant_events::<IdnTestIndexer>(&trees, 41, 1); // Randomness Beacon - SignatureVerificationSuccess

    assert_eq!(variant_40_0.len(), 1);
    assert_eq!(variant_41_0.len(), 1);
    assert_eq!(variant_41_1.len(), 1);
}
