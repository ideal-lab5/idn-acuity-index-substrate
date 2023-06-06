
use subxt::{
    OnlineClient,
    PolkadotConfig,
    utils::AccountId32,
    metadata::Metadata,
};

use futures::{
    StreamExt,
};
use std::{
    collections::HashMap,
    time::SystemTime,
};

use tokio::sync:: {
    mpsc::Sender,
    RwLock,
};

use crate::shared::*;
use crate::pallets::bags_list::*;
use crate::pallets::balances::*;
use crate::pallets::bounties::*;
use crate::pallets::child_bounties::*;
use crate::pallets::claims::*;
use crate::pallets::council::*;
use crate::pallets::democracy::*;
use crate::pallets::election_provider_multi_phase::*;
use crate::pallets::elections_phragmen::*;
use crate::pallets::fast_unstake::*;
use crate::pallets::identity::*;
use crate::pallets::indices::*;
use crate::pallets::multisig::*;
use crate::pallets::nomination_pools::*;
use crate::pallets::preimage::*;
use crate::pallets::proxy::*;
use crate::pallets::session::*;
use crate::pallets::staking::*;
use crate::pallets::system::*;
use crate::pallets::technical_committee::*;
use crate::pallets::tips::*;
use crate::pallets::transaction_payment::*;
use crate::pallets::treasury::*;
use crate::pallets::vesting::*;

use crate::pallets::polkadot::auctions::*;
use crate::pallets::polkadot::crowdloan::*;
use crate::pallets::polkadot::parachains_disputes::*;
use crate::pallets::polkadot::parachains_hrmp::*;
use crate::pallets::polkadot::parachains_paras::*;
use crate::pallets::polkadot::parachains_ump::*;
use crate::pallets::polkadot::paras_registrar::*;
use crate::pallets::polkadot::slots::*;

use tokio::sync::mpsc::Receiver;

pub async fn substrate_head(api: OnlineClient<PolkadotConfig>, trees: Trees, mut sub_rx: Receiver<SubscribeMessage>) {
    let mut indexer = Indexer::new(trees.clone(), api.clone()).await;

    // Subscribe to all finalized blocks:
    let mut blocks_sub = api.blocks().subscribe_finalized().await.unwrap();

    loop {
        tokio::select! {
            block = blocks_sub.next() => {
                let block = block.unwrap().unwrap();
                let block_number = block.header().number;
                println!(" ✨ #{block_number}");
                indexer.index_block(block_number).await.unwrap();
                trees.root.insert("last_head_block", &block_number.to_be_bytes()).unwrap();
            }
            Some(msg) = sub_rx.recv() => {
                match indexer.sub_map.get_mut(&msg.key) {
                    Some(txs) => {
                        txs.push(msg.sub_response_tx);
                    },
                    None => {
                        let mut txs = Vec::new();
                        txs.push(msg.sub_response_tx);
                        indexer.sub_map.insert(msg.key, txs);
                    },
                };
            }
        }
    }
}

pub struct Indexer {
    trees: Trees,
    api: OnlineClient<PolkadotConfig>,
    metadata_map_lock: RwLock<HashMap<u32, Metadata>>,
    sub_map: HashMap<Key, Vec<Sender<ResponseMessage>>>,
}

#[derive(Debug)]
enum IndexBlockError {
    BlockNotFound,
}

impl Indexer {
    async fn new(trees: Trees, api: OnlineClient<PolkadotConfig>) -> Self {
        Indexer {
            trees,
            api,
            metadata_map_lock: RwLock::new(HashMap::new()),
            sub_map: HashMap::new(),
        }
    }

    async fn index_block(&self, block_number: u32) -> Result<(), IndexBlockError> {
        
        let block_hash = match self.api.rpc().block_hash(Some(block_number.into())).await.unwrap() {
            Some(block_hash) => block_hash,
            None => return Err(IndexBlockError::BlockNotFound),
        };
        // Get the runtime version of the block.
        let runtime_version = self.api.rpc().runtime_version(Some(block_hash)).await.unwrap();

        let metadata_map = self.metadata_map_lock.read().await;
        let metadata = match metadata_map.get(&runtime_version.spec_version) {
            Some(metadata) => {
                let metadata = metadata.clone();
                drop(metadata_map);
                metadata
            },
            None => {
                drop(metadata_map);
                let mut metadata_map  = self.metadata_map_lock.write().await;

                match metadata_map.get(&runtime_version.spec_version) {
                    Some(metadata) => metadata.clone(),
                    None => {
                        println!("Downloading metadata for spec version {}", runtime_version.spec_version);
                        let metadata = self.api.rpc().metadata_legacy(Some(block_hash)).await.unwrap();
                        metadata_map.insert(runtime_version.spec_version, metadata.clone());
                        metadata
                    },
                }
            },
        };
            
        let events = subxt::events::Events::new_from_client(metadata, block_hash, self.api.clone()).await.unwrap();
    
        for (i, evt) in events.iter().enumerate() {
            match evt {
                Ok(evt) => {
                    self.index_event(block_number, i.try_into().unwrap(), evt).await;
                },
                Err(error) => println!("Block: {}, error: {}", block_number, error),
            }
        }
            
        Ok(())
    }
    
    async fn index_event(&self, block_number: u32, event_index: u32, event: subxt::events::EventDetails<PolkadotConfig>) {
        
        // Generate key
        let key = VariantKey {
            pallet_index: event.pallet_index(),
            variant_index: event.variant_index(),
            block_number,
            i: event_index,
        }.serialize();
        // Insert record.
        self.trees.variant.insert(key, &[]).unwrap();

        let search_key = Key::Variant(event.pallet_index(), event.variant_index());

        match self.sub_map.get(&search_key) {
            Some(txs) => {
                let msg = ResponseMessage::Events {
                    key: search_key,
                    events: vec![Event{block_number, i: event_index}],
                };
                for tx in txs.iter() {
                    match tx.send(msg.clone()).await {
                        Ok(_) => (),
                        Err(_) => (),
                    }
                }
            }
            None => (),
        }

        let pallet_name = event.pallet_name().to_owned();
    //    let variant_name = event.variant_name().to_owned();
    
        let result = match pallet_name.as_str() {
            "Auctions" => auctions_index_event(self, block_number, event_index, event),
            "Balances" => balance_index_event(self, block_number, event_index, event),
            "Bounties" => bounties_index_event(self, block_number, event_index, event),
            "ChildBounties" => child_bounties_index_event(self, block_number, event_index, event),
            "Claims" => claims_index_event(self, block_number, event_index, event),
            "Council" => council_index_event(self, block_number, event_index, event),
            "TechnicalCommittee" => technical_committee_index_event(self, block_number, event_index, event),
            "Crowdloan" => crowdloan_index_event(self, block_number, event_index, event),
            "Democracy" => democracy_index_event(self, block_number, event_index, event),
            "ElectionProviderMultiPhase" => election_provider_multi_phase_index_event(self, block_number, event_index, event),
            "FastUnstake" => fast_unstake_index_event(self, block_number, event_index, event),
            "Hrmp" => parachains_hrmp_index_event(self, block_number, event_index, event),
            "Identity" => identity_index_event(self, block_number, event_index, event),
            "Indices" => indices_index_event(self, block_number, event_index, event),
            "Multisig" => multisig_index_event(self, block_number, event_index, event),
            "NominationPools" => nomination_pools_index_event(self, block_number, event_index, event),
            "Paras" => parachains_paras_index_event(self, block_number, event_index, event),
            "Ump" => parachains_ump_index_event(self, block_number, event_index, event),
            "ParasDisputes" => parachains_disputes_index_event(self, block_number, event_index, event),
            "PhragmenElection" => elections_phragmen_index_event(self, block_number, event_index, event),
            "Preimage" => preimage_index_event(self, block_number, event_index, event),
            "Proxy" => proxy_index_event(self, block_number, event_index, event),
            "Registrar" => paras_registrar_index_event(self, block_number, event_index, event),
            "Session" => session_index_event(self, block_number, event_index, event),
            "Slots" => slots_index_event(self, block_number, event_index, event),
            "Staking" => staking_index_event(self, block_number, event_index, event),
            "System" => system_index_event(self, block_number, event_index, event),
            "Tips" => tips_index_event(self, block_number, event_index, event),
            "TransactionPayment" => transaction_payment_index_event(self, block_number, event_index, event),
            "Treasury" => treasury_index_event(self, block_number, event_index, event),
            "Vesting" => vesting_index_event(self, block_number, event_index, event),
            "VoterList" => bags_list_index_event(self, block_number, event_index, event),
            _ => Ok(()),
        };
    
        match result  {
            Ok(()) => (),
            Err(error) => {
            //    println!("Block: {}, pallet: {}, variant: {}, error: {}", block_number, pallet_name, variant_name, error);
            }
        };
    }
    
    pub fn index_event_account_id(&self, account_id: AccountId32, block_number: u32, i: u32) {
        // Generate key
        let key = AccountIdKey {
            account_id,
            block_number,
            i,
        }.serialize();
        // Insert record.
        self.trees.account_id.insert(key, &[]).unwrap();
    }
    
    pub fn index_event_account_index(&self, account_index: u32, block_number: u32, i: u32) {
        // Generate key
        let key = U32Key {
            key: account_index,
            block_number,
            i,
        }.serialize();
        // Insert record.
        self.trees.account_index.insert(key, &[]).unwrap();
    }
    
    pub fn index_event_auction_index(&self, auction_index: u32, block_number: u32, i: u32) {
        // Generate key
        let key = U32Key {
            key: auction_index,
            block_number,
            i,
        }.serialize();
        // Insert record.
        self.trees.auction_index.insert(key, &[]).unwrap();
    }
    
    pub fn index_event_bounty_index(&self, bounty_index: u32, block_number: u32, i: u32) {
        // Generate key
        let key = U32Key {
            key: bounty_index,
            block_number,
            i,
        }.serialize();
        // Insert record.
        self.trees.bounty_index.insert(key, &[]).unwrap();
    }
    
    pub fn index_event_candidate_hash(&self, candidate_hash: [u8; 32], block_number: u32, i: u32) {
        // Generate key
        let key = CandidateHashKey {
            candidate_hash,
            block_number,
            i,
        }.serialize();
        // Insert record.
        self.trees.candidate_hash.insert(key, &[]).unwrap();
    }
    
    pub fn index_event_era_index(&self, era_index: u32, block_number: u32, i: u32) {
        // Generate key
        let key = U32Key {
            key: era_index,
            block_number,
            i,
        }.serialize();
        // Insert record.
        self.trees.era_index.insert(key, &[]).unwrap();
    }
    
    pub fn index_event_message_id(&self, message_id: [u8; 32], block_number: u32, i: u32) {
        // Generate key
        let key = MessageIdKey {
            message_id,
            block_number,
            i,
        }.serialize();
        // Insert record.
        self.trees.message_id.insert(key, &[]).unwrap();
    }
    
    pub fn index_event_para_id(&self, para_id: u32, block_number: u32, i: u32) {
        // Generate key
        let key = U32Key {
            key: para_id,
            block_number,
            i,
        }.serialize();
        // Insert record.
        self.trees.para_id.insert(key, &[]).unwrap();
    }
    
    pub fn index_event_pool_id(&self, pool_id: u32, block_number: u32, i: u32) {
        // Generate key
        let key = U32Key {
            key: pool_id,
            block_number,
            i,
        }.serialize();
        // Insert record.
        self.trees.pool_id.insert(key, &[]).unwrap();
    }
    
    pub fn index_event_preimage_hash(&self, preimage_hash: [u8; 32], block_number: u32, i: u32) {
        // Generate key
        let key = HashKey {
            hash: preimage_hash,
            block_number,
            i,
        }.serialize();
        // Insert record.
        self.trees.preimage_hash.insert(key, &[]).unwrap();
    }
    
    pub fn index_event_proposal_hash(&self, proposal_hash: [u8; 32], block_number: u32, i: u32) {
        // Generate key
        let key = HashKey {
            hash: proposal_hash,
            block_number,
            i,
        }.serialize();
        // Insert record.
        self.trees.proposal_hash.insert(key, &[]).unwrap();
    }
    
    pub fn index_event_proposal_index(&self, proposal_index: u32, block_number: u32, i: u32) {
        // Generate key
        let key = U32Key {
            key: proposal_index,
            block_number,
            i,
        }.serialize();
        // Insert record.
        self.trees.proposal_index.insert(key, &[]).unwrap();
    }
    
    pub fn index_event_ref_index(&self, ref_index: u32, block_number: u32, i: u32) {
        // Generate key
        let key = U32Key {
            key: ref_index,
            block_number,
            i,
        }.serialize();
        // Insert record.
        self.trees.ref_index.insert(key, &[]).unwrap();
    }
    
    pub fn index_event_registrar_index(&self, registrar_index: u32, block_number: u32, i: u32) {
        // Generate key
        let key = U32Key {
            key: registrar_index,
            block_number,
            i,
        }.serialize();
        // Insert record.
        self.trees.registrar_index.insert(key, &[]).unwrap();
    }
    
    pub fn index_event_session_index(&self, session_index: u32, block_number: u32, i: u32) {
        // Generate key
        let key = U32Key {
            key: session_index,
            block_number,
            i,
        }.serialize();
        // Insert record.
        self.trees.session_index.insert(key, &[]).unwrap();
    }
    
    pub fn index_event_tip_hash(&self, tip_hash: [u8; 32], block_number: u32, i: u32) {
        // Generate key
        let key = TipHashKey {
            tip_hash,
            block_number,
            i,
        }.serialize();
        // Insert record.
        self.trees.tip_hash.insert(key, &[]).unwrap();
    }

}

pub async fn substrate_batch(api: OnlineClient<PolkadotConfig>, trees: Trees, args: Args) {
    // Determine the correct block to start batch indexing.
    let mut block_number: u32 = match args.block_height {
        Some(block_height) => block_height,
        None => {
            match match trees.root.get("batch_indexing_complete").unwrap() {
                    Some(value) => value.to_vec()[0] == 1,
                    None => false,
                }
            {
                true => match trees.root.get("last_head_block").unwrap() {
                    Some(value) => u32::from_be_bytes(vector_as_u8_4_array(&value)),
                    None => 0,
                }
                false => match trees.root.get("last_batch_block").unwrap() {
                    Some(value) => u32::from_be_bytes(vector_as_u8_4_array(&value)),
                    None => 0,
                }
            }
        }
    };
    // Determine the correct block to start batch indexing.
    let async_blocks = args.async_blocks.unwrap_or(128);
    // Record in database that batch indexing has not finished.
    trees.root.insert("batch_indexing_complete", &0_u8.to_be_bytes()).unwrap();

    let substrate_batch = Indexer::new(trees.clone(), api).await;

    // AccountIndex: 9494
    substrate_batch.index_block(10013701).await.unwrap();
    // AuctionIndex: 15, ParaId: 2013
    substrate_batch.index_block(10018925).await.unwrap();
    // BountyIndex: 11
    substrate_batch.index_block(15104642).await.unwrap();
    // CandidateHash: 0x6a1cd467afb69aa2b23866538b1160a60d96228587c5d7efc1d3c1ce4e3efb63
    substrate_batch.index_block(10059744).await.unwrap();
    // EraIndex: 1076
    substrate_batch.index_block(15825858).await.unwrap();
    // MessageId: 0xc656c0814b4174d3fbae7b0dd3ae63a94ac858b9120f8dc13027d2ee89f54a46
    substrate_batch.index_block(15100192).await.unwrap();
    // PoolId: 12
    substrate_batch.index_block(15180584).await.unwrap();
    // PreimageHash: 0xdb2b6cb38c2f6704ed067da2e9001bc57314be4f0117f664a93c0d18610110c5
    substrate_batch.index_block(15764612).await.unwrap();
    // ProposalHash: 0x7c403355a3747fea8a84968a7a83b7f5d2b26ea0b5d63b317ae65c1b091cf07b
    substrate_batch.index_block(10025666).await.unwrap();
    // ProposalIndex: 103
    substrate_batch.index_block(10022400).await.unwrap();
    // RefIndex: 114
    substrate_batch.index_block(15100839).await.unwrap();
    // RegistrarIndex: 1
    substrate_batch.index_block(10027254).await.unwrap();
    // SessionIndex: 6552
    substrate_batch.index_block(15649648).await.unwrap();
    // TipHash: 0x729c6a740112abfc8cd143771f1f88518c3906e86f601a6c6a312fe7f7babf33
    substrate_batch.index_block(10146463).await.unwrap();

    let mut block_futures = Vec::new();

    for n in 0..async_blocks {
        block_futures.push(Box::pin(substrate_batch.index_block(block_number + n)));
    }
    
    let mut last_batch_block = block_number;
    block_number += async_blocks;
    let mut now = SystemTime::now();

    loop {
        if block_futures.is_empty() {
            trees.root.insert("batch_indexing_complete", &1_u8.to_be_bytes()).unwrap();
            println!(" 📚 Finished batch indexing.");
            break;
        }
        let result = futures::future::select_all(block_futures).await;
        
        block_futures = result.2;
        
        match result.0 {
            Ok(()) => {
                block_futures.push(Box::pin(substrate_batch.index_block(block_number)));
                
                if (block_number - async_blocks) > last_batch_block {
                    last_batch_block = block_number - async_blocks;
                    if last_batch_block % 100 == 0 {
                        trees.root.insert("last_batch_block", &last_batch_block.to_be_bytes()).unwrap();        
                        println!(" 📚 #{}, {:?} blocks/sec", last_batch_block, 100_000_000 / now.elapsed().unwrap().as_micros());
                        now = SystemTime::now();
                    }
                }
                
                block_number += 1;
            }
            Err(error) => match error {
                IndexBlockError::BlockNotFound => (),
            }
        }
    }
}

