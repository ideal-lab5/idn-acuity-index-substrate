use subxt::{metadata::Metadata, utils::AccountId32, OnlineClient};

use futures::{future::select_all, StreamExt};
use std::{collections::HashMap, sync::Mutex, time::SystemTime};

use tokio::{
    sync::{mpsc, watch, RwLock},
    time::{self, Duration, MissedTickBehavior},
};

use log::*;

use crate::shared::*;

pub struct Indexer<R: RuntimeIndexer> {
    trees: Trees,
    api: Option<OnlineClient<R::RuntimeConfig>>,
    metadata_map_lock: RwLock<HashMap<u32, Metadata>>,
    sub_map: Mutex<HashMap<Key, Vec<mpsc::UnboundedSender<ResponseMessage>>>>,
}

impl<R: RuntimeIndexer> Indexer<R> {
    fn new(trees: Trees, api: OnlineClient<R::RuntimeConfig>) -> Self {
        Indexer {
            trees,
            api: Some(api),
            metadata_map_lock: RwLock::new(HashMap::new()),
            sub_map: HashMap::new().into(),
        }
    }

    #[cfg(test)]
    pub fn new_test(trees: Trees) -> Self {
        Indexer {
            trees,
            api: None,
            metadata_map_lock: RwLock::new(HashMap::new()),
            sub_map: HashMap::new().into(),
        }
    }

    async fn index_block(&self, block_number: u32) -> Result<u32, IndexError> {
        debug!("Indexing #{}", block_number);
        let api = self.api.as_ref().unwrap();

        let block_hash = match api.rpc().block_hash(Some(block_number.into())).await? {
            Some(block_hash) => block_hash,
            None => return Err(IndexError::BlockNotFound(block_number)),
        };
        // Get the runtime version of the block.
        let runtime_version = api.rpc().runtime_version(Some(block_hash)).await?;

        let metadata_map = self.metadata_map_lock.read().await;
        let metadata = match metadata_map.get(&runtime_version.spec_version) {
            Some(metadata) => {
                let metadata = metadata.clone();
                drop(metadata_map);
                metadata
            }
            None => {
                drop(metadata_map);
                let mut metadata_map = self.metadata_map_lock.write().await;

                match metadata_map.get(&runtime_version.spec_version) {
                    Some(metadata) => metadata.clone(),
                    None => {
                        info!(
                            "Downloading metadata for spec version {}",
                            runtime_version.spec_version
                        );
                        let metadata = api.rpc().metadata_legacy(Some(block_hash)).await?;
                        metadata_map.insert(runtime_version.spec_version, metadata.clone());
                        metadata
                    }
                }
            }
        };

        let events =
            subxt::events::Events::new_from_client(metadata, block_hash, api.clone()).await?;

        for (i, event) in events.iter().enumerate() {
            match event {
                Ok(event) => {
                    let event_index = i.try_into().unwrap();
                    self.index_event_variant(
                        event.pallet_index(),
                        event.variant_index(),
                        block_number,
                        event_index,
                    )?;
                    let _ = R::process_event(self, block_number, event_index, event);
                }
                Err(error) => error!("Block: {}, error: {}", block_number, error),
            }
        }

        Ok(events.len())
    }

    pub fn notify_subscribers(&self, search_key: Key, event: Event) {
        let sub_map = self.sub_map.lock().unwrap();
        if let Some(txs) = sub_map.get(&search_key) {
            let msg = ResponseMessage::Events {
                key: search_key,
                events: vec![event],
            };
            for tx in txs.iter() {
                if tx.send(msg.clone()).is_ok() {}
            }
        }
    }

    pub fn index_event_variant(
        &self,
        pallet_index: u8,
        variant_index: u8,
        block_number: u32,
        event_index: u16,
    ) -> Result<(), sled::Error> {
        // Generate key
        let key = VariantKey {
            pallet_index,
            variant_index,
            block_number,
            event_index,
        }
        .serialize();
        // Insert record.
        self.trees.variant.insert(key, &[])?;
        // Notify subscribers.
        let search_key = Key::Variant(pallet_index, variant_index);
        self.notify_subscribers(
            search_key,
            Event {
                block_number,
                event_index,
            },
        );
        Ok(())
    }

    pub fn index_event_account_id(
        &self,
        account_id: AccountId32,
        block_number: u32,
        event_index: u16,
    ) -> Result<(), sled::Error> {
        // Generate key
        let key = AccountIdKey {
            account_id: account_id.clone(),
            block_number,
            event_index,
        }
        .serialize();
        // Insert record.
        self.trees.account_id.insert(key, &[])?;
        // Notify subscribers.
        let search_key = Key::AccountId(Bytes32(account_id.0));
        self.notify_subscribers(
            search_key,
            Event {
                block_number,
                event_index,
            },
        );
        Ok(())
    }

    pub fn index_event_account_index(
        &self,
        account_index: u32,
        block_number: u32,
        event_index: u16,
    ) -> Result<(), sled::Error> {
        // Generate key
        let key = U32Key {
            key: account_index,
            block_number,
            event_index,
        }
        .serialize();
        // Insert record.
        self.trees.account_index.insert(key, &[])?;
        // Notify subscribers.
        let search_key = Key::AccountIndex(account_index);
        self.notify_subscribers(
            search_key,
            Event {
                block_number,
                event_index,
            },
        );
        Ok(())
    }

    pub fn index_event_auction_index(
        &self,
        auction_index: u32,
        block_number: u32,
        event_index: u16,
    ) -> Result<(), sled::Error> {
        // Generate key
        let key = U32Key {
            key: auction_index,
            block_number,
            event_index,
        }
        .serialize();
        // Insert record.
        self.trees.auction_index.insert(key, &[])?;
        // Notify subscribers.
        let search_key = Key::AuctionIndex(auction_index);
        self.notify_subscribers(
            search_key,
            Event {
                block_number,
                event_index,
            },
        );
        Ok(())
    }

    pub fn index_event_bounty_index(
        &self,
        bounty_index: u32,
        block_number: u32,
        event_index: u16,
    ) -> Result<(), sled::Error> {
        // Generate key
        let key = U32Key {
            key: bounty_index,
            block_number,
            event_index,
        }
        .serialize();
        // Insert record.
        self.trees.bounty_index.insert(key, &[])?;
        // Notify subscribers.
        let search_key = Key::BountyIndex(bounty_index);
        self.notify_subscribers(
            search_key,
            Event {
                block_number,
                event_index,
            },
        );
        Ok(())
    }

    pub fn index_event_candidate_hash(
        &self,
        candidate_hash: [u8; 32],
        block_number: u32,
        event_index: u16,
    ) -> Result<(), sled::Error> {
        // Generate key
        let key = CandidateHashKey {
            candidate_hash,
            block_number,
            event_index,
        }
        .serialize();
        // Insert record.
        self.trees.candidate_hash.insert(key, &[])?;
        // Notify subscribers.
        let search_key = Key::CandidateHash(Bytes32(candidate_hash));
        self.notify_subscribers(
            search_key,
            Event {
                block_number,
                event_index,
            },
        );
        Ok(())
    }

    pub fn index_event_era_index(
        &self,
        era_index: u32,
        block_number: u32,
        event_index: u16,
    ) -> Result<(), sled::Error> {
        // Generate key
        let key = U32Key {
            key: era_index,
            block_number,
            event_index,
        }
        .serialize();
        // Insert record.
        self.trees.era_index.insert(key, &[])?;
        // Notify subscribers.
        let search_key = Key::EraIndex(era_index);
        self.notify_subscribers(
            search_key,
            Event {
                block_number,
                event_index,
            },
        );
        Ok(())
    }

    pub fn index_event_message_id(
        &self,
        message_id: [u8; 32],
        block_number: u32,
        event_index: u16,
    ) -> Result<(), sled::Error> {
        // Generate key
        let key = MessageIdKey {
            message_id,
            block_number,
            event_index,
        }
        .serialize();
        // Insert record.
        self.trees.message_id.insert(key, &[])?;
        // Notify subscribers.
        let search_key = Key::MessageId(Bytes32(message_id));
        self.notify_subscribers(
            search_key,
            Event {
                block_number,
                event_index,
            },
        );
        Ok(())
    }

    pub fn index_event_para_id(
        &self,
        para_id: u32,
        block_number: u32,
        event_index: u16,
    ) -> Result<(), sled::Error> {
        // Generate key
        let key = U32Key {
            key: para_id,
            block_number,
            event_index,
        }
        .serialize();
        // Insert record.
        self.trees.para_id.insert(key, &[])?;
        // Notify subscribers.
        let search_key = Key::ParaId(para_id);
        self.notify_subscribers(
            search_key,
            Event {
                block_number,
                event_index,
            },
        );
        Ok(())
    }

    pub fn index_event_pool_id(
        &self,
        pool_id: u32,
        block_number: u32,
        event_index: u16,
    ) -> Result<(), sled::Error> {
        // Generate key
        let key = U32Key {
            key: pool_id,
            block_number,
            event_index,
        }
        .serialize();
        // Insert record.
        self.trees.pool_id.insert(key, &[])?;
        // Notify subscribers.
        let search_key = Key::PoolId(pool_id);
        self.notify_subscribers(
            search_key,
            Event {
                block_number,
                event_index,
            },
        );
        Ok(())
    }

    pub fn index_event_preimage_hash(
        &self,
        preimage_hash: [u8; 32],
        block_number: u32,
        event_index: u16,
    ) -> Result<(), sled::Error> {
        // Generate key
        let key = HashKey {
            hash: preimage_hash,
            block_number,
            event_index,
        }
        .serialize();
        // Insert record.
        self.trees.preimage_hash.insert(key, &[])?;
        // Notify subscribers.
        let search_key = Key::PreimageHash(Bytes32(preimage_hash));
        self.notify_subscribers(
            search_key,
            Event {
                block_number,
                event_index,
            },
        );
        Ok(())
    }

    pub fn index_event_proposal_hash(
        &self,
        proposal_hash: [u8; 32],
        block_number: u32,
        event_index: u16,
    ) -> Result<(), sled::Error> {
        // Generate key
        let key = HashKey {
            hash: proposal_hash,
            block_number,
            event_index,
        }
        .serialize();
        // Insert record.
        self.trees.proposal_hash.insert(key, &[])?;
        // Notify subscribers.
        let search_key = Key::ProposalHash(Bytes32(proposal_hash));
        self.notify_subscribers(
            search_key,
            Event {
                block_number,
                event_index,
            },
        );
        Ok(())
    }

    pub fn index_event_proposal_index(
        &self,
        proposal_index: u32,
        block_number: u32,
        event_index: u16,
    ) -> Result<(), sled::Error> {
        // Generate key
        let key = U32Key {
            key: proposal_index,
            block_number,
            event_index,
        }
        .serialize();
        // Insert record.
        self.trees.proposal_index.insert(key, &[])?;
        // Notify subscribers.
        let search_key = Key::ProposalIndex(proposal_index);
        self.notify_subscribers(
            search_key,
            Event {
                block_number,
                event_index,
            },
        );
        Ok(())
    }

    pub fn index_event_ref_index(
        &self,
        ref_index: u32,
        block_number: u32,
        event_index: u16,
    ) -> Result<(), sled::Error> {
        // Generate key
        let key = U32Key {
            key: ref_index,
            block_number,
            event_index,
        }
        .serialize();
        // Insert record.
        self.trees.ref_index.insert(key, &[])?;
        // Notify subscribers.
        let search_key = Key::RefIndex(ref_index);
        self.notify_subscribers(
            search_key,
            Event {
                block_number,
                event_index,
            },
        );
        Ok(())
    }

    pub fn index_event_registrar_index(
        &self,
        registrar_index: u32,
        block_number: u32,
        event_index: u16,
    ) -> Result<(), sled::Error> {
        // Generate key
        let key = U32Key {
            key: registrar_index,
            block_number,
            event_index,
        }
        .serialize();
        // Insert record.
        self.trees.registrar_index.insert(key, &[])?;
        // Notify subscribers.
        let search_key = Key::RegistrarIndex(registrar_index);
        self.notify_subscribers(
            search_key,
            Event {
                block_number,
                event_index,
            },
        );
        Ok(())
    }

    pub fn index_event_session_index(
        &self,
        session_index: u32,
        block_number: u32,
        event_index: u16,
    ) -> Result<(), sled::Error> {
        // Generate key
        let key = U32Key {
            key: session_index,
            block_number,
            event_index,
        }
        .serialize();
        // Insert record.
        self.trees.session_index.insert(key, &[])?;
        // Notify subscribers.
        let search_key = Key::SessionIndex(session_index);
        self.notify_subscribers(
            search_key,
            Event {
                block_number,
                event_index,
            },
        );
        Ok(())
    }

    pub fn index_event_tip_hash(
        &self,
        tip_hash: [u8; 32],
        block_number: u32,
        event_index: u16,
    ) -> Result<(), sled::Error> {
        // Generate key
        let key = TipHashKey {
            tip_hash,
            block_number,
            event_index,
        }
        .serialize();
        // Insert record.
        self.trees.tip_hash.insert(key, &[])?;
        // Notify subscribers.
        let search_key = Key::TipHash(Bytes32(tip_hash));
        self.notify_subscribers(
            search_key,
            Event {
                block_number,
                event_index,
            },
        );
        Ok(())
    }
}

pub async fn substrate_index<R: RuntimeIndexer>(
    trees: Trees,
    api: OnlineClient<R::RuntimeConfig>,
    block_number: Option<u32>,
    queue_depth: u32,
    mut exit_rx: watch::Receiver<bool>,
    mut sub_rx: mpsc::UnboundedReceiver<SubscribeMessage>,
) -> Result<(), IndexError> {
    // Subscribe to all finalized blocks:
    let mut blocks_sub = api.blocks().subscribe_finalized().await?;
    let head_start_block: u32 = blocks_sub
        .next()
        .await
        .ok_or(IndexError::BlockNotFound(0))??
        .number()
        .into()
        .try_into()
        .unwrap();
    // Determine the correct block to start batch indexing.
    let mut block_number = match block_number {
        Some(block_height) => block_height,
        None => {
            match match trees.root.get("batch_indexing_complete")? {
                Some(value) => value.to_vec()[0] == 1,
                None => false,
            } {
                true => match trees.root.get("last_head_block")? {
                    Some(value) => u32::from_be_bytes(vector_as_u8_4_array(&value)),
                    None => R::get_default_start_block(),
                },
                false => match trees.root.get("last_batch_block")? {
                    Some(value) => u32::from_be_bytes(vector_as_u8_4_array(&value)),
                    None => R::get_default_start_block(),
                },
            }
        }
    };
    info!("📚 Batch indexing from #{}", block_number);
    // Record in database that batch indexing has not finished.
    trees
        .root
        .insert("batch_indexing_complete", &0_u8.to_be_bytes())?;

    let indexer = Indexer::<R>::new(trees.clone(), api);

    let mut futures = Vec::with_capacity(queue_depth.try_into().unwrap());

    for n in 0..queue_depth {
        futures.push(Box::pin(indexer.index_block(block_number + n)));
    }

    let mut last_batch_block = block_number;
    let mut last_batch_time = SystemTime::now();
    let mut batch_event_count: u32 = 0;

    let mut interval = time::interval(Duration::from_millis(2000));
    interval.set_missed_tick_behavior(MissedTickBehavior::Skip);

    let mut is_batching = true;

    loop {
        tokio::select! {
            biased;

            _ = exit_rx.changed() => {
                return Ok(());
            }
            Some(msg) = sub_rx.recv() => {
                let mut sub_map = indexer.sub_map.lock().unwrap();
                match sub_map.get_mut(&msg.key) {
                    Some(txs) => {
                        txs.push(msg.sub_response_tx);
                    },
                    None => {
                        let txs = vec![msg.sub_response_tx];
                        sub_map.insert(msg.key, txs);
                    },
                };
            }
            Some(Ok(block)) = blocks_sub.next() => {
                let head_block_number:u32 = block.number().into().try_into().unwrap();
                match indexer.index_block(head_block_number).await {
                    Ok(event_count) => {
                        trees.root.insert("last_head_block", &head_block_number.to_be_bytes())?;
                        info!("✨ #{head_block_number}, {event_count} events");
                    },
                    Err(_) => {
                        error!("Failed to index #{}", head_block_number);
                    }
                }
            }
            _ = interval.tick(), if is_batching => {
                if block_number > last_batch_block {
                    let current_batch_time = SystemTime::now();
                    let duration = (current_batch_time.duration_since(last_batch_time)).unwrap().as_micros();
                    if duration != 0 {
                        info!(
                            "📚 #{}, {:?} blocks/sec, {:?} events/sec",
                            block_number,
                            <u32 as Into<u128>>::into(block_number - last_batch_block) * 1_000_000 / duration,
                            <u32 as Into<u128>>::into(batch_event_count) * 1_000_000 / duration,
                        );
                    }
                    last_batch_block = block_number;
                    batch_event_count = 0;
                    last_batch_time = current_batch_time;
                }
            }
            (result, index, _) = select_all(&mut futures), if is_batching => {
                if head_start_block == block_number {
                    trees.root.insert("batch_indexing_complete", &1_u8.to_be_bytes())?;
                    info!("📚 Finished batch indexing.");
                    is_batching = false;
                }
                else if block_number % 1000 == 0 {
                    trees.root.insert("last_batch_block", &block_number.to_be_bytes())?;
                }
                match result {
                    Ok(event_count) => batch_event_count += event_count,
                    Err(error) => {
                        match error {
                            IndexError::BlockNotFound(block_number_not_found) => {
                                if block_number > head_start_block + queue_depth {
                                    error!("📚 Block not found #{}", block_number_not_found);
                                    is_batching = false;
                                }
                            },
                            _ => {
                                error!("📚 Batch indexing failed.");
                                is_batching = false;
                            },
                        }
                    }
                }
                futures[index] = Box::pin(indexer.index_block(block_number + queue_depth));
                block_number += 1;
            }
        }
    }
}
