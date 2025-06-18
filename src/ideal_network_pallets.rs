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

//! Event indexing macros for Ideal Network specific pallets
//!
//! This module contains macros for indexing events from Ideal Network custom pallets:
//! - IDN Manager pallet - handles subscription lifecycle events
//! - Randomness Beacon pallet - handles beacon configuration and signature verification

#[macro_export]
macro_rules! index_idn_manager_event {
    ($event_enum: ty, $event: ident, $indexer: ident, $block_number: ident, $event_index: ident) => {
        match $event {
            <$event_enum>::SubscriptionCreated { sub_id } => {
                $indexer.index_event(
                    Key::Substrate(SubstrateKey::SubscriptionId(SubscriptionId::from(*sub_id))),
                    $block_number,
                    $event_index,
                )?;
                1
            }
            <$event_enum>::SubscriptionTerminated { sub_id } => {
                $indexer.index_event(
                    Key::Substrate(SubstrateKey::SubscriptionId(SubscriptionId::from(*sub_id))),
                    $block_number,
                    $event_index,
                )?;
                1
            }
            <$event_enum>::SubscriptionPaused { sub_id } => {
                $indexer.index_event(
                    Key::Substrate(SubstrateKey::SubscriptionId(SubscriptionId::from(*sub_id))),
                    $block_number,
                    $event_index,
                )?;
                1
            }
            <$event_enum>::SubscriptionUpdated { sub_id } => {
                $indexer.index_event(
                    Key::Substrate(SubstrateKey::SubscriptionId(SubscriptionId::from(*sub_id))),
                    $block_number,
                    $event_index,
                )?;
                1
            }
            <$event_enum>::SubscriptionReactivated { sub_id } => {
                $indexer.index_event(
                    Key::Substrate(SubstrateKey::SubscriptionId(SubscriptionId::from(*sub_id))),
                    $block_number,
                    $event_index,
                )?;
                1
            }
            <$event_enum>::RandomnessDistributed { sub_id } => {
                $indexer.index_event(
                    Key::Substrate(SubstrateKey::SubscriptionId(SubscriptionId::from(*sub_id))),
                    $block_number,
                    $event_index,
                )?;
                1
            }
            <$event_enum>::FeesCollected { sub_id, .. } => {
                $indexer.index_event(
                    Key::Substrate(SubstrateKey::SubscriptionId(SubscriptionId::from(*sub_id))),
                    $block_number,
                    $event_index,
                )?;
                1
            }
            <$event_enum>::SubQuoted { .. } => {
                // This event might not have indexable fields, or we need to check the actual structure
                0
            }
            <$event_enum>::SubscriptionDistributed { sub_id } => {
                $indexer.index_event(
                    Key::Substrate(SubstrateKey::SubscriptionId(SubscriptionId::from(*sub_id))),
                    $block_number,
                    $event_index,
                )?;
                1
            }
            _ => 0,
        }
    };
}

#[macro_export]
macro_rules! index_randomness_beacon_event {
    ($event_enum: ty, $event: ident, $indexer: ident, $block_number: ident, $event_index: ident) => {
        match $event {
            <$event_enum>::BeaconConfigSet { .. } => {
                // This event might contain configuration data but no indexable keys
                0
            }
            <$event_enum>::SignatureVerificationSuccess { .. } => {
                // This event indicates successful signature verification but no indexable keys
                0
            }
            _ => 0,
        }
    };
}
