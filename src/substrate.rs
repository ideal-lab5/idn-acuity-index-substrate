use subxt::{
    backend::legacy::LegacyRpcMethods, metadata::Metadata, utils::AccountId32, OnlineClient,
};

use futures::future;
use std::{collections::HashMap, sync::Mutex, time::SystemTime};

use tokio::{
    sync::{mpsc, watch, RwLock},
    time::{self, Duration, MissedTickBehavior},
};

use ahash::AHashMap;
use log::*;
use num_format::{Locale, ToFormattedString};
use zerocopy::AsBytes;

use crate::shared::*;

pub struct Indexer<R: RuntimeIndexer + ?Sized> {
    trees: Trees,
    api: Option<OnlineClient<R::RuntimeConfig>>,
    rpc: Option<LegacyRpcMethods<R::RuntimeConfig>>,
    metadata_map_lock: RwLock<AHashMap<u32, Metadata>>,
    sub_map: Mutex<HashMap<Key, Vec<mpsc::UnboundedSender<ResponseMessage>>>>,
}

impl<R: RuntimeIndexer> Indexer<R> {
    fn new(
        trees: Trees,
        api: OnlineClient<R::RuntimeConfig>,
        rpc: LegacyRpcMethods<R::RuntimeConfig>,
    ) -> Self {
        Indexer {
            trees,
            api: Some(api),
            rpc: Some(rpc),
            metadata_map_lock: RwLock::new(AHashMap::new()),
            sub_map: HashMap::new().into(),
        }
    }

    #[cfg(test)]
    pub fn new_test(trees: Trees) -> Self {
        Indexer {
            trees,
            api: None,
            rpc: None,
            metadata_map_lock: RwLock::new(AHashMap::new()),
            sub_map: HashMap::new().into(),
        }
    }

    async fn index_block(&self, block_number: u32) -> Result<(u32, u32, u32), IndexError> {
        let mut key_count = 0;
        let api = self.api.as_ref().unwrap();
        let rpc = self.rpc.as_ref().unwrap();

        let block_hash = match rpc.chain_get_block_hash(Some(block_number.into())).await? {
            Some(block_hash) => block_hash,
            None => return Err(IndexError::BlockNotFound(block_number)),
        };
        // Get the runtime version of the block.
        let runtime_version = rpc.state_get_runtime_version(Some(block_hash)).await?;

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
                        let metadata = rpc.state_get_metadata(Some(block_hash)).await?;
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
                    key_count += 1;
                    if let Ok(event_key_count) =
                        R::process_event(self, block_number, event_index, event)
                    {
                        key_count += event_key_count;
                    }
                }
                Err(error) => error!("Block: {}, error: {}", block_number, error),
            }
        }

        Ok((block_number, events.len(), key_count))
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
            block_number: block_number.into(),
            event_index: event_index.into(),
        };
        // Insert record.
        self.trees.variant.insert(key.as_bytes(), &[])?;
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
        let key = Bytes32Key {
            key: account_id.0,
            block_number: block_number.into(),
            event_index: event_index.into(),
        };
        // Insert record.
        self.trees.account_id.insert(key.as_bytes(), &[])?;
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
            key: account_index.into(),
            block_number: block_number.into(),
            event_index: event_index.into(),
        };
        // Insert record.
        self.trees.account_index.insert(key.as_bytes(), &[])?;
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
            key: auction_index.into(),
            block_number: block_number.into(),
            event_index: event_index.into(),
        };
        // Insert record.
        self.trees.auction_index.insert(key.as_bytes(), &[])?;
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
            key: bounty_index.into(),
            block_number: block_number.into(),
            event_index: event_index.into(),
        };
        // Insert record.
        self.trees.bounty_index.insert(key.as_bytes(), &[])?;
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
        let key = Bytes32Key {
            key: candidate_hash,
            block_number: block_number.into(),
            event_index: event_index.into(),
        };
        // Insert record.
        self.trees.candidate_hash.insert(key.as_bytes(), &[])?;
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
            key: era_index.into(),
            block_number: block_number.into(),
            event_index: event_index.into(),
        };
        // Insert record.
        self.trees.era_index.insert(key.as_bytes(), &[])?;
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
        let key = Bytes32Key {
            key: message_id,
            block_number: block_number.into(),
            event_index: event_index.into(),
        };
        // Insert record.
        self.trees.message_id.insert(key.as_bytes(), &[])?;
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
            key: para_id.into(),
            block_number: block_number.into(),
            event_index: event_index.into(),
        };
        // Insert record.
        self.trees.para_id.insert(key.as_bytes(), &[])?;
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
            key: pool_id.into(),
            block_number: block_number.into(),
            event_index: event_index.into(),
        };
        // Insert record.
        self.trees.pool_id.insert(key.as_bytes(), &[])?;
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
        let key = Bytes32Key {
            key: preimage_hash,
            block_number: block_number.into(),
            event_index: event_index.into(),
        };
        // Insert record.
        self.trees.preimage_hash.insert(key.as_bytes(), &[])?;
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
        let key = Bytes32Key {
            key: proposal_hash,
            block_number: block_number.into(),
            event_index: event_index.into(),
        };
        // Insert record.
        self.trees.proposal_hash.insert(key.as_bytes(), &[])?;
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
            key: proposal_index.into(),
            block_number: block_number.into(),
            event_index: event_index.into(),
        };
        // Insert record.
        self.trees.proposal_index.insert(key.as_bytes(), &[])?;
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
            key: ref_index.into(),
            block_number: block_number.into(),
            event_index: event_index.into(),
        };
        // Insert record.
        self.trees.ref_index.insert(key.as_bytes(), &[])?;
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
            key: registrar_index.into(),
            block_number: block_number.into(),
            event_index: event_index.into(),
        };
        // Insert record.
        self.trees.registrar_index.insert(key.as_bytes(), &[])?;
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
            key: session_index.into(),
            block_number: block_number.into(),
            event_index: event_index.into(),
        };
        // Insert record.
        self.trees.session_index.insert(key.as_bytes(), &[])?;
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
        let key = Bytes32Key {
            key: tip_hash,
            block_number: block_number.into(),
            event_index: event_index.into(),
        };
        // Insert record.
        self.trees.tip_hash.insert(key.as_bytes(), &[])?;
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

#[derive(Debug)]
struct Span {
    start: u32,
    end: u32,
}

pub async fn substrate_index<R: RuntimeIndexer>(
    trees: Trees,
    api: OnlineClient<R::RuntimeConfig>,
    rpc: LegacyRpcMethods<R::RuntimeConfig>,
    queue_depth: u32,
    mut exit_rx: watch::Receiver<bool>,
    mut sub_rx: mpsc::UnboundedReceiver<SubscribeMessage>,
) -> Result<(), IndexError> {
    // Subscribe to all finalized blocks:
    let mut blocks_sub = api.blocks().subscribe_finalized().await?;
    // Determine the correct block to start batch indexing.
    let mut next_batch_block: u32 = blocks_sub
        .next()
        .await
        .ok_or(IndexError::BlockNotFound(0))??
        .number()
        .into()
        .try_into()
        .unwrap();
    info!(
        "📚 Batch indexing backwards from #{}",
        next_batch_block.to_formatted_string(&Locale::en)
    );
    // Load already indexed spans from the db.
    let mut spans = vec![];
    for span in &trees.span {
        if let Ok((end, start)) = span {
            let span = Span {
                start: u32::from_be_bytes(start.as_ref().try_into().unwrap()),
                end: u32::from_be_bytes(end.as_ref().try_into().unwrap()),
            };
            info!(
                "📚 Previous span of indexed blocks from #{} to #{}.",
                span.start.to_formatted_string(&Locale::en),
                span.end.to_formatted_string(&Locale::en)
            );
            spans.push(span);
        }
    }
    // Create a span of the first finalized block that will be indexed.
    let mut current_span = Span {
        start: next_batch_block + 1,
        end: next_batch_block + 1,
    };

    let indexer = Indexer::<R>::new(trees.clone(), api, rpc);

    info!("📚 Queue depth: {}", queue_depth);
    let mut futures = Vec::with_capacity(queue_depth.try_into().unwrap());

    for _ in 0..queue_depth {
        let mut i = spans.len();
        while i != 0 {
            i -= 1;
            if next_batch_block >= spans[i].start && next_batch_block <= spans[i].end {
                next_batch_block = spans[i].start - 1;
            }
        }
        futures.push(Box::pin(indexer.index_block(next_batch_block)));
        debug!(
            "⬆️  Block #{} queued.",
            next_batch_block.to_formatted_string(&Locale::en)
        );
        next_batch_block -= 1;
    }

    let mut orphans: AHashMap<u32, bool> = AHashMap::new();

    let mut stats_block_count = 0;
    let mut stats_event_count = 0;
    let mut stats_key_count = 0;
    let mut stats_start_time = SystemTime::now();

    let mut interval = time::interval(Duration::from_millis(2000));
    interval.set_missed_tick_behavior(MissedTickBehavior::Skip);

    let mut is_batching = true;

    loop {
        tokio::select! {
            biased;

            _ = exit_rx.changed() => {
                if current_span.start != current_span.end {
                    trees.span.insert(current_span.end.to_be_bytes(), &current_span.start.to_be_bytes())?;
                    info!(
                        "📚 Recording current indexed span from #{} to #{}",
                        current_span.start.to_formatted_string(&Locale::en),
                        current_span.end.to_formatted_string(&Locale::en)
                    );
                }
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
                match indexer.index_block(block.number().into().try_into().unwrap()).await {
                    Ok((block_number, event_count, key_count)) => {
                        trees.span.remove(current_span.end.to_be_bytes())?;
                        current_span.end = block_number;
                        trees.span.insert(current_span.end.to_be_bytes(), &current_span.start.to_be_bytes())?;
                        info!(
                            "✨ #{}: {} events, {} keys",
                            block_number.to_formatted_string(&Locale::en),
                            event_count.to_formatted_string(&Locale::en),
                            key_count.to_formatted_string(&Locale::en),
                        );
                    },
                    Err(error) => {
                        match error {
                            IndexError::BlockNotFound(block_number) => {
                                error!("✨ Block not found #{}", block_number.to_formatted_string(&Locale::en));
                            },
                            _ => {
                                error!("✨ Indexing failed.");
                            },
                        }
                    },
                };
            }
            _ = interval.tick(), if is_batching => {
                let current_time = SystemTime::now();
                let duration = (current_time.duration_since(stats_start_time)).unwrap().as_micros();
                if duration != 0 {
                    info!(
                        "📚 #{}: {} blocks/sec, {} events/sec, {} keys/sec",
                        current_span.start.to_formatted_string(&Locale::en),
                        (<u32 as Into<u128>>::into(stats_block_count) * 1_000_000 / duration).to_formatted_string(&Locale::en),
                        (<u32 as Into<u128>>::into(stats_event_count) * 1_000_000 / duration).to_formatted_string(&Locale::en),
                        (<u32 as Into<u128>>::into(stats_key_count) * 1_000_000 / duration).to_formatted_string(&Locale::en),
                    );
                }
                stats_block_count = 0;
                stats_event_count = 0;
                stats_key_count = 0;
                stats_start_time = current_time;
            }
            (result, index, _) = future::select_all(&mut futures), if is_batching => {
                match result {
                    Ok((block_number, event_count, key_count)) => {
                        // Is the new block contiguous to the current span or an orphan?
                        if block_number == current_span.start - 1 {
                            current_span.start = block_number;
                            debug!("⬇️  Block #{} indexed.", block_number.to_formatted_string(&Locale::en));
                            while let Some(span) = spans.last() {
                                // Have we indexed all the blocks after the span?
                                if current_span.start - 1 >= span.start && current_span.start - 1 <= span.end {
                                    let skipped = span.end - span.start + 1;
                                    info!(
                                        "📚 Skipping {} blocks from #{} to #{}",
                                        skipped.to_formatted_string(&Locale::en),
                                        span.start.to_formatted_string(&Locale::en),
                                        span.end.to_formatted_string(&Locale::en),
                                    );
                                    current_span.start = span.start;
                                    // Remove the span.
                                    trees.span.remove(span.end.to_be_bytes())?;
                                    spans.pop();
                                }
                                else {
                                    break;
                                }
                            }
                            // Check if any orphans are now contiguous.
                            while orphans.contains_key(&(current_span.start - 1)) {
                                current_span.start -= 1;
                                orphans.remove(&current_span.start);
                                debug!("➡️  Block #{} unorphaned.", current_span.start.to_formatted_string(&Locale::en));
                                while let Some(span) = spans.last() {
                                    // Have we indexed all the blocks after the span?
                                    if current_span.start - 1 >= span.start && current_span.start - 1 <= span.end {
                                        let skipped = span.end - span.start + 1;
                                        info!(
                                            "📚 Skipping {} blocks from #{} to #{}",
                                            skipped.to_formatted_string(&Locale::en),
                                            span.start.to_formatted_string(&Locale::en),
                                            span.end.to_formatted_string(&Locale::en),
                                        );
                                        current_span.start = span.start;
                                        // Remove the span.
                                        trees.span.remove(span.end.to_be_bytes())?;
                                        spans.pop();
                                    }
                                    else {
                                        break;
                                    }
                                }
                            }
                        }
                        else {
                            orphans.insert(block_number, true);
                            debug!("⬇️  Block #{} indexed and orphaned.", block_number.to_formatted_string(&Locale::en));
                        }
                        stats_block_count += 1;
                        stats_event_count += event_count;
                        stats_key_count += key_count;
                    },
                    Err(error) => {
                        match error {
                            IndexError::BlockNotFound(block_number) => {
                                error!("📚 Block not found #{}", block_number.to_formatted_string(&Locale::en));
                                is_batching = false;
                            },
                            _ => {
                                error!("📚 Batch indexing failed.");
                                is_batching = false;
                            },
                        }
                    }
                }
                // Figure out the next block to index, skipping the next span if we have reached it.
                let mut i = spans.len();
                while i != 0 {
                    i -= 1;
                    if next_batch_block >= spans[i].start && next_batch_block <= spans[i].end {
                        next_batch_block = spans[i].start - 1;
                    }
                }
                futures[index] = Box::pin(indexer.index_block(next_batch_block));
                debug!("⬆️  Block #{} queued.", next_batch_block.to_formatted_string(&Locale::en));
                next_batch_block -= 1;
            }
        }
    }
}
