
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
    sync::Arc,
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
use crate::pallets::collective::*;
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
use crate::pallets::system::*;
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

pub fn index_event_account_id(trees: Trees, account_id: AccountId32, block_number: u32, i: u32) {
    // Generate key
    let key = AccountIdKey {
        account_id,
        block_number,
        i,
    }.serialize();
    // Insert record.
    trees.account_id.insert(key, &[]).unwrap();
}

pub fn index_event_account_index(trees: Trees, account_index: u32, block_number: u32, i: u32) {
    println!("AccountIndex: {}", account_index);
    // Generate key
    let key = AccountIndexKey {
        account_index,
        block_number,
        i,
    }.serialize();
    // Insert record.
    trees.account_index.insert(key, &[]).unwrap();
}

pub fn index_event_auction_index(trees: Trees, auction_index: u32, block_number: u32, i: u32) {
    println!("AuctionIndex: {}", auction_index);
    // Generate key
    let key = AuctionIndexKey {
        auction_index,
        block_number,
        i,
    }.serialize();
    // Insert record.
    trees.auction_index.insert(key, &[]).unwrap();
}

pub fn index_event_bounty_index(trees: Trees, bounty_index: u32, block_number: u32, i: u32) {
    println!("BountyIndex: {}", bounty_index);
    // Generate key
    let key = BountyIndexKey {
        bounty_index,
        block_number,
        i,
    }.serialize();
    // Insert record.
    trees.bounty_index.insert(key, &[]).unwrap();
}

pub fn index_event_candidate_hash(trees: Trees, candidate_hash: [u8; 32], block_number: u32, i: u32) {
    println!("CandidateHash: 0x{}", hex::encode(candidate_hash));
    // Generate key
    let key = CandidateHashKey {
        candidate_hash,
        block_number,
        i,
    }.serialize();
    // Insert record.
    trees.candidate_hash.insert(key, &[]).unwrap();
}

pub fn index_event_message_id(trees: Trees, message_id: [u8; 32], block_number: u32, i: u32) {
    println!("MessageId: 0x{}", hex::encode(message_id));
    // Generate key
    let key = MessageIdKey {
        message_id,
        block_number,
        i,
    }.serialize();
    // Insert record.
    trees.message_id.insert(key, &[]).unwrap();
}

pub fn index_event_para_id(trees: Trees, para_id: u32, block_number: u32, i: u32) {
    println!("ParaId: {}", para_id);
    // Generate key
    let key = ParaIdKey {
        para_id,
        block_number,
        i,
    }.serialize();
    // Insert record.
    trees.para_id.insert(key, &[]).unwrap();
}

pub fn index_event_pool_id(trees: Trees, pool_id: u32, block_number: u32, i: u32) {
    println!("PoolId: {}", pool_id);
    // Generate key
    let key = PoolIdKey {
        pool_id,
        block_number,
        i,
    }.serialize();
    // Insert record.
    trees.pool_id.insert(key, &[]).unwrap();
}

pub fn index_event_preimage_hash(trees: Trees, preimage_hash: [u8; 32], block_number: u32, i: u32) {
    println!("PreimageHash: 0x{}", hex::encode(preimage_hash));
    // Generate key
    let key = HashKey {
        hash: preimage_hash,
        block_number,
        i,
    }.serialize();
    // Insert record.
    trees.preimage_hash.insert(key, &[]).unwrap();
}

pub fn index_event_proposal_hash(trees: Trees, proposal_hash: [u8; 32], block_number: u32, i: u32) {
    println!("ProposalHash: 0x{}", hex::encode(proposal_hash));
    // Generate key
    let key = HashKey {
        hash: proposal_hash,
        block_number,
        i,
    }.serialize();
    // Insert record.
    trees.proposal_hash.insert(key, &[]).unwrap();
}

pub fn index_event_proposal_index(trees: Trees, proposal_index: u32, block_number: u32, i: u32) {
    println!("ProposalIndex: {}", proposal_index);
    // Generate key
    let key = ProposalIndexKey {
        proposal_index,
        block_number,
        i,
    }.serialize();
    // Insert record.
    trees.proposal_index.insert(key, &[]).unwrap();
}

pub fn index_event_ref_index(trees: Trees, ref_index: u32, block_number: u32, i: u32) {
    println!("RefIndex: {}", ref_index);
    // Generate key
    let key = RefIndexKey {
        ref_index,
        block_number,
        i,
    }.serialize();
    // Insert record.
    trees.ref_index.insert(key, &[]).unwrap();
}

pub fn index_event_registrar_index(trees: Trees, registrar_index: u32, block_number: u32, i: u32) {
    println!("RegistrarIndex: {}", registrar_index);
    // Generate key
    let key = RegistrarIndexKey {
        registrar_index,
        block_number,
        i,
    }.serialize();
    // Insert record.
    trees.registrar_index.insert(key, &[]).unwrap();
}

pub fn index_event_tip_hash(trees: Trees, tip_hash: [u8; 32], block_number: u32, i: u32) {
    println!("TipHash: 0x{}", hex::encode(tip_hash));
    // Generate key
    let key = TipHashKey {
        tip_hash,
        block_number,
        i,
    }.serialize();
    // Insert record.
    trees.tip_hash.insert(key, &[]).unwrap();
}

fn index_event(trees: Trees, block_number: u32, event_index: u32, event: subxt::events::EventDetails) {
    
    // Generate key
    let key = VariantKey {
        pallet_index: event.pallet_index(),
        variant_index: event.variant_index(),
        block_number,
        i: event_index,
    }.serialize();
    // Insert record.
    trees.variant.insert(key, &[]).unwrap();

    let result = match event.pallet_name() {
        "Auctions" => auctions_index_event(trees, block_number, event_index, event),
        "VoterList" => bags_list_index_event(trees, block_number, event_index, event),
        "Balances" => balance_index_event(trees, block_number, event_index, event),
        "Bounties" => bounties_index_event(trees, block_number, event_index, event),
        "ChildBounties" => child_bounties_index_event(trees, block_number, event_index, event),
        "Claims" => claims_index_event(trees, block_number, event_index, event),
        "Council" => collective_index_event(trees, block_number, event_index, event),
        "TechnicalCommittee" => collective_index_event(trees, block_number, event_index, event),
        "Crowdloan" => crowdloan_index_event(trees, block_number, event_index, event),
        "Democracy" => democracy_index_event(trees, block_number, event_index, event),
        "ElectionProviderMultiPhase" => election_provider_multi_phase_index_event(trees, block_number, event_index, event),
        "FastUnstake" => fast_unstake_index_event(trees, block_number, event_index, event),
        "Hrmp" => parachains_hrmp_index_event(trees, block_number, event_index, event),
        "Identity" => identity_index_event(trees, block_number, event_index, event),
        "Indices" => indices_index_event(trees, block_number, event_index, event),
        "Multisig" => multisig_index_event(trees, block_number, event_index, event),
        "NominationPools" => nomination_pools_index_event(trees, block_number, event_index, event),
        "Paras" => parachains_paras_index_event(trees, block_number, event_index, event),
        "Ump" => parachains_ump_index_event(trees, block_number, event_index, event),
        "ParasDisputes" => parachains_disputes_index_event(trees, block_number, event_index, event),
        "PhragmenElection" => elections_phragmen_index_event(trees, block_number, event_index, event),
        "Preimage" => preimage_index_event(trees, block_number, event_index, event),
        "Proxy" => proxy_index_event(trees, block_number, event_index, event),
        "Registrar" => paras_registrar_index_event(trees, block_number, event_index, event),
        "Slots" => slots_index_event(trees, block_number, event_index, event),
        "System" => system_index_event(trees, block_number, event_index, event),
        "Tips" => tips_index_event(trees, block_number, event_index, event),
        "TransactionPayment" => transaction_payment_index_event(trees, block_number, event_index, event),
        "Treasury" => treasury_index_event(trees, block_number, event_index, event),
        "Vesting" => vesting_index_event(trees, block_number, event_index, event),
        _ => Ok(()),
    };

    match result  {
        Ok(()) => (),
        Err(error) => {
            println!("{}", error);
        }
    };
}

pub async fn index_block(api: OnlineClient<PolkadotConfig>, trees: Trees, block_number: u32) {
    // Get block hash.
    let block_hash = api.rpc().block_hash(Some(block_number.into())).await.unwrap().unwrap();
    // Download the metadata of the starting block.
    let metadata = api.rpc().metadata_legacy(Some(block_hash)).await.unwrap();

    println!(" 📚 #{block_number}: 0x{}", hex::encode(block_hash.0));

    let events = subxt::events::Events::new_from_client(metadata, block_hash, api).await.unwrap();

    for (i, evt) in events.iter().enumerate() {
        if let Ok(evt) = evt {
            index_event(trees.clone(), block_number, i.try_into().unwrap(), evt);
        }
    }
}

use tokio::sync::mpsc::Receiver;

pub async fn substrate_head(api: OnlineClient<PolkadotConfig>, trees: Trees, mut sub_rx: Receiver<SubscribeMessage>) {
    let mut sub_map: HashMap<Key, Vec<Sender<ResponseMessage>>> = HashMap::new();
    
    // Subscribe to all finalized blocks:
    let mut blocks_sub = api.blocks().subscribe_finalized().await.unwrap();

    let block = blocks_sub.next().await.unwrap().unwrap();
    let mut block_number = block.header().number;
    let mut block_hash = block.hash();
    // Download the metadata of the starting block.
    let mut metadata = api.rpc().metadata_legacy(Some(block_hash)).await.unwrap();

    'blocks: loop {
        println!(" ✨ #{block_number}: 0x{}", hex::encode(block_hash.0));

        let events = subxt::events::Events::new_from_client(metadata.clone(), block_hash, api.clone()).await.unwrap();

        for (i, evt) in events.iter().enumerate() {
            match evt {
                Ok(evt) => {
                    index_event(trees.clone(), block_number, i.try_into().unwrap(), evt);
                },
                Err(error) => println!("Block: {}, error: {}", block_number, error),
            }
        }

        trees.root.insert("last_head_block", &block_number.to_be_bytes()).unwrap();

        loop {
            tokio::select! {
                block = blocks_sub.next() => {
                    let block = block.unwrap().unwrap();
                    block_number = block.header().number;
                    block_hash = block.hash();
                    continue 'blocks;
                }
                msg = sub_rx.recv() => {
                    let msg = msg.unwrap();
                    sub_map.entry(msg.key);
                }
            }
        }
    }
}


#[derive(Clone)]
struct SubstrateBatch {
    trees: Trees,
    api: OnlineClient<PolkadotConfig>,
}

enum IndexBlockError {
    BlockNotFound,
    WrongMetadata(u32),
}

impl SubstrateBatch {
    async fn new(trees: Trees, api: OnlineClient<PolkadotConfig>, block_number: u32) -> Self {
        SubstrateBatch {
            trees,
            api,
        }
    }

    async fn index_block(&self, block_number: u32, metadata_map_lock: Arc<RwLock<HashMap<u32, Metadata>>>) -> Result<(), IndexBlockError> {
        
        let block_hash = match self.api.rpc().block_hash(Some(block_number.into())).await.unwrap() {
            Some(block_hash) => block_hash,
            None => return Err(IndexBlockError::BlockNotFound),
        };
        // Get the runtime version of the block.
        let runtime_version = self.api.rpc().runtime_version(Some(block_hash)).await.unwrap();

        let metadata;
        {
            let metadata_map = metadata_map_lock.read().await;
            metadata = match metadata_map.get(&runtime_version.spec_version) {
                Some(metadata) => metadata.clone(),
                None => {
                    drop(metadata_map);
                    let metadata = self.api.rpc().metadata_legacy(Some(block_hash)).await.unwrap();
                    let mut metadata_map  = metadata_map_lock.write().await;
                    metadata_map.insert(runtime_version.spec_version, metadata.clone());
                    metadata
                },
            }
        }
            
        let events = subxt::events::Events::new_from_client(metadata, block_hash, self.api.clone()).await.unwrap();
    
        for (i, evt) in events.iter().enumerate() {
            match evt {
                Ok(evt) => {
                    index_event(self.trees.clone(), block_number, i.try_into().unwrap(), evt);
                },
                Err(error) => println!("Block: {}, error: {}", block_number, error),
            }
        }
            
        Ok(())
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

    let metadata_map_lock: Arc<RwLock<HashMap<u32, Metadata>>> = Arc::new(RwLock::new(HashMap::new()));
    // Get the hash of the starting block.
    let block_hash = api.rpc().block_hash(Some(block_number.into())).await.unwrap().unwrap();
    // Get the runtime version of the starting block.
    let runtime_version = api.rpc().runtime_version(Some(block_hash)).await.unwrap();
    // Download the metadata of the starting block.
//        let metadata = api.rpc().metadata_at_version(runtime_version.spec_version).await.unwrap();
    let metadata = api.rpc().metadata_legacy(Some(block_hash)).await.unwrap();
    {
        let mut metadata_map = metadata_map_lock.write().await;
        metadata_map.insert(runtime_version.spec_version, metadata);
    }
    
    let substrate_batch = SubstrateBatch::new(trees.clone(), api, block_number).await;

    let mut block_futures = Vec::new();

    for n in 0..async_blocks {
        block_futures.push(Box::pin(substrate_batch.index_block(block_number + n, Arc::clone(&metadata_map_lock))));
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
                block_futures.push(Box::pin(substrate_batch.index_block(block_number, Arc::clone(&metadata_map_lock))));
                
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
                IndexBlockError::WrongMetadata(_block_number) => {
                    println!(" 📚 Downloading new metadata.");
 //                   substrate_batches.push(SubstrateBatch::new(trees.clone(), api.clone(), block_number).await);
                    
//                    metadata = api.rpc().metadata_legacy(Some(block_hash)).await.unwrap();
                }
            }
        }
    }
}

