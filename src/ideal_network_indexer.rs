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

// Example implementation of RuntimeIndexer for an Ideal Network runtime
// This file provides an example of how to implement the RuntimeIndexer trait
// for an Ideal Network runtime that includes custom pallets

use crate::{
    index_balances_event, index_idn_manager_event, index_indices_event,
    index_preimage_event, index_randomness_beacon_event, index_session_event,
    index_staking_event, index_system_event, index_transaction_payment_event,
    shared::*, substrate::Indexer, IndexError,
};
use serde::{Deserialize, Serialize};
use sled::{Db, Tree};
use std::hash::Hash;

// ChainKey implementation for IDN-specific keys, if needed
#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, Hash)]
#[serde(tag = "type", content = "value")]
pub enum IdnChainKey {
    // Add any custom IDN-specific keys here
    // For example:
    // CustomSubscriptionId(u32),
    // CustomBeaconId(Bytes32),
}

// Your custom trees implementation for IDN-specific data
#[derive(Clone, Debug)]
pub struct IdnChainTrees {
    // Add your custom trees here
    // For example:
    // pub custom_subscription_tree: Tree,
    // pub custom_beacon_tree: Tree,
}

impl IndexTrees for IdnChainTrees {
    fn open(db: &Db) -> Result<Self, sled::Error> {
        Ok(IdnChainTrees {
            // Open your custom trees here
            // For example:
            // custom_subscription_tree: db.open_tree(b"custom_subscription_tree")?,
            // custom_beacon_tree: db.open_tree(b"custom_beacon_tree")?,
        })
    }

    fn flush(&self) -> Result<(), sled::Error> {
        // Flush your custom trees here
        // For example:
        // self.custom_subscription_tree.flush()?;
        // self.custom_beacon_tree.flush()?;
        Ok(())
    }
}

impl IndexKey for IdnChainKey {
    type ChainTrees = IdnChainTrees;

    fn write_db_key(
        &self,
        trees: &IdnChainTrees,
        block_number: u32,
        event_index: u16,
    ) -> Result<(), sled::Error> {
        match self {
            // Implement indexing for your custom keys here
            // For example:
            // IdnChainKey::CustomSubscriptionId(id) => {
            //     let key = U32Key {
            //         key: (*id).into(),
            //         block_number: block_number.into(),
            //         event_index: event_index.into(),
            //     };
            //     trees.custom_subscription_tree.insert(key.as_bytes(), &[])?
            // }
        };
        Ok(())
    }

    fn get_key_events(&self, trees: &IdnChainTrees) -> Vec<Event> {
        match self {
            // Implement event retrieval for your custom keys here
            // For example:
            // IdnChainKey::CustomSubscriptionId(id) => {
            //     // Logic to get events for this key
            //     Vec::new() // Replace with actual implementation
            // }
        }
    }
}

// The main RuntimeIndexer implementation for Ideal Network
pub struct IdealNetworkIndexer;

impl RuntimeIndexer for IdealNetworkIndexer {
    type RuntimeConfig = subxt::PolkadotConfig; // Or your custom runtime config
    type ChainKey = IdnChainKey; // Your implementation of ChainKey

    fn get_name() -> &'static str {
        "idn"
    }

    fn get_genesis_hash() -> <Self::RuntimeConfig as subxt::Config>::Hash {
        // Your genesis hash - replace with actual hash
        [0u8; 32].into()
    }

    fn get_versions() -> &'static [u32] {
        // Your runtime versions
        &[9110, 9111]
    }

    fn get_default_url() -> &'static str {
        // Your default node URL
        "wss://your-node-url.com"
    }

    fn process_event(
        indexer: &Indexer<Self>,
        block_number: u32,
        event_index: u16,
        event: subxt::events::EventDetails<Self::RuntimeConfig>,
    ) -> Result<u32, IndexError> {
        // Get the pallet and variant indices from the event
        let pallet_index = event.pallet_index();
        let variant_index = event.variant_index();

        // Index the event by variant
        indexer.index_event(
            Key::Variant(pallet_index, variant_index),
            block_number,
            event_index,
        )?;

        // Event handling based on pallet index
        let event_key_count = match pallet_index {
            // Standard substrate pallets - adjust indices as needed for your runtime
            0 => index_system_event!(
                subxt::events::static_types::SystemEvents,
                event,
                indexer,
                block_number,
                event_index
            ),
            4 => index_balances_event!(
                subxt::events::static_types::BalancesEvents,
                event,
                indexer,
                block_number,
                event_index
            ),
            // ... other standard pallets

            // IDeal Network pallets - use the actual pallet indices from your runtime
            // You'll need to determine the actual pallet indices from your runtime metadata
            //
            // IMPORTANT: The randomness beacon pallet only has 2 events with no indexable fields:
            // - BeaconConfigSet (no fields)
            // - SignatureVerificationSuccess (no fields)
            // So PulseRound and BeaconPublicKey keys have been removed from shared.rs
            40 => index_idn_manager_event!(
                pallet_idn_manager::pallet::Event,
                event,
                indexer,
                block_number,
                event_index
            ),
            41 => index_randomness_beacon_event!(
                pallet_randomness_beacon::pallet::Event,
                event,
                indexer,
                block_number,
                event_index
            ),
            _ => 0,
        };

        Ok(event_key_count)
    }
}
