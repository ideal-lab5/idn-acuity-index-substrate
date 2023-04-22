
use subxt::{
    OnlineClient,
    PolkadotConfig,
    utils::AccountId32,
    Error::Metadata,
    metadata::MetadataError::EventNotFound,
};

use futures::StreamExt;

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

pub fn index_event_account_id(trees: Trees, account_id: AccountId32, block_number: u32, i: u32, bytes: &[u8]) {
    // Generate key
    let key = AccountIdKey {
        account_id,
        block_number,
        i,
    }.serialize();
    // Insert record.
    trees.account_id.insert(key, bytes).unwrap();
}

pub fn index_event_account_index(trees: Trees, account_index: u32, block_number: u32, i: u32, bytes: &[u8]) {
    println!("AccountIndex: {}", account_index);
    // Generate key
    let key = AccountIndexKey {
        account_index,
        block_number,
        i,
    }.serialize();
    // Insert record.
    trees.account_index.insert(key, bytes).unwrap();
}

pub fn index_event_auction_index(trees: Trees, auction_index: u32, block_number: u32, i: u32, bytes: &[u8]) {
    println!("AuctionIndex: {}", auction_index);
    // Generate key
    let key = AuctionIndexKey {
        auction_index,
        block_number,
        i,
    }.serialize();
    // Insert record.
    trees.auction_index.insert(key, bytes).unwrap();
}

pub fn index_event_bounty_index(trees: Trees, bounty_index: u32, block_number: u32, i: u32, bytes: &[u8]) {
    println!("BountyIndex: {}", bounty_index);
    // Generate key
    let key = BountyIndexKey {
        bounty_index,
        block_number,
        i,
    }.serialize();
    // Insert record.
    trees.bounty_index.insert(key, bytes).unwrap();
}

pub fn index_event_candidate_hash(trees: Trees, candidate_hash: [u8; 32], block_number: u32, i: u32, bytes: &[u8]) {
    println!("CandidateHash: 0x{}", hex::encode(candidate_hash));
    // Generate key
    let key = CandidateHashKey {
        candidate_hash,
        block_number,
        i,
    }.serialize();
    // Insert record.
    trees.candidate_hash.insert(key, bytes).unwrap();
}

pub fn index_event_message_id(trees: Trees, message_id: [u8; 32], block_number: u32, i: u32, bytes: &[u8]) {
    println!("MessageId: 0x{}", hex::encode(message_id));
    // Generate key
    let key = MessageIdKey {
        message_id,
        block_number,
        i,
    }.serialize();
    // Insert record.
    trees.message_id.insert(key, bytes).unwrap();
}

pub fn index_event_para_id(trees: Trees, para_id: u32, block_number: u32, i: u32, bytes: &[u8]) {
    println!("ParaId: {}", para_id);
    // Generate key
    let key = ParaIdKey {
        para_id,
        block_number,
        i,
    }.serialize();
    // Insert record.
    trees.para_id.insert(key, bytes).unwrap();
}

pub fn index_event_pool_id(trees: Trees, pool_id: u32, block_number: u32, i: u32, bytes: &[u8]) {
    println!("PoolId: {}", pool_id);
    // Generate key
    let key = PoolIdKey {
        pool_id,
        block_number,
        i,
    }.serialize();
    // Insert record.
    trees.pool_id.insert(key, bytes).unwrap();
}

pub fn index_event_proposal_hash(trees: Trees, proposal_hash: [u8; 32], block_number: u32, i: u32, bytes: &[u8]) {
    println!("ProposalHash: 0x{}", hex::encode(proposal_hash));
    // Generate key
    let key = ProposalHashKey {
        proposal_hash,
        block_number,
        i,
    }.serialize();
    // Insert record.
    trees.proposal_hash.insert(key, bytes).unwrap();
}

pub fn index_event_proposal_index(trees: Trees, proposal_index: u32, block_number: u32, i: u32, bytes: &[u8]) {
    println!("ProposalIndex: {}", proposal_index);
    // Generate key
    let key = ProposalIndexKey {
        proposal_index,
        block_number,
        i,
    }.serialize();
    // Insert record.
    trees.proposal_index.insert(key, bytes).unwrap();
}

pub fn index_event_ref_index(trees: Trees, ref_index: u32, block_number: u32, i: u32, bytes: &[u8]) {
    println!("RefIndex: {}", ref_index);
    // Generate key
    let key = RefIndexKey {
        ref_index,
        block_number,
        i,
    }.serialize();
    // Insert record.
    trees.ref_index.insert(key, bytes).unwrap();
}

pub fn index_event_registrar_index(trees: Trees, registrar_index: u32, block_number: u32, i: u32, bytes: &[u8]) {
    println!("RegistrarIndex: {}", registrar_index);
    // Generate key
    let key = RegistrarIndexKey {
        registrar_index,
        block_number,
        i,
    }.serialize();
    // Insert record.
    trees.registrar_index.insert(key, bytes).unwrap();
}

pub fn index_event_tip_hash(trees: Trees, tip_hash: [u8; 32], block_number: u32, i: u32, bytes: &[u8]) {
    println!("TipHash: 0x{}", hex::encode(tip_hash));
    // Generate key
    let key = TipHashKey {
        tip_hash,
        block_number,
        i,
    }.serialize();
    // Insert record.
    trees.tip_hash.insert(key, bytes).unwrap();
}

fn index_event(trees: Trees, block_number: u32, event_index: u32, event: subxt::events::EventDetails) {

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
    let metadata = api.rpc().metadata(Some(block_hash)).await.unwrap();

    println!(" 📚 #{block_number}: 0x{}", hex::encode(block_hash.0));

    let events = subxt::events::Events::new_from_client(metadata, block_hash, api).await.unwrap();

    for (i, evt) in events.iter().enumerate() {
        if let Ok(evt) = evt {
            index_event(trees.clone(), block_number, i.try_into().unwrap(), evt);
        }
    }
}

pub async fn substrate_head(api: OnlineClient<PolkadotConfig>, trees: Trees) {
    // Subscribe to all finalized blocks:
    let mut blocks_sub = api.blocks().subscribe_finalized().await.unwrap();

    let block = blocks_sub.next().await.unwrap().unwrap();
    let mut block_number = block.header().number;
    let mut block_hash = block.hash();
    // Download the metadata of the starting block.
    let mut metadata = api.rpc().metadata(Some(block_hash)).await.unwrap();

    'blocks: loop {
        println!(" ✨ #{block_number}: 0x{}", hex::encode(block_hash.0));

        let events = subxt::events::Events::new_from_client(metadata.clone(), block_hash, api.clone()).await.unwrap();

        for (i, evt) in events.iter().enumerate() {
            match evt {
                Ok(evt) => {
                    index_event(trees.clone(), block_number, i.try_into().unwrap(), evt);
                },
                Err(error) => if let Metadata(EventNotFound(_, _)) = error {
                    println!(" ✨ Downloading new metadata.");
                    metadata = api.rpc().metadata(Some(block_hash)).await.unwrap();
                    continue 'blocks;
                }
            }
        }

        trees.root.insert("last_head_block", &block_number.to_be_bytes()).unwrap();

        let block = blocks_sub.next().await.unwrap().unwrap();
        block_number = block.header().number;
        block_hash = block.hash();
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
    // Record in database that batch indexing has not finished.
    trees.root.insert("batch_indexing_complete", &0_u8.to_be_bytes()).unwrap();
    // Get the hash of the starting block.
    let mut block_hash = api.rpc().block_hash(Some(block_number.into())).await.unwrap().unwrap();
    // Download the metadata of the starting block.
    let mut metadata = api.rpc().metadata(Some(block_hash)).await.unwrap();

    'blocks: loop {
        println!(" 📚 #{block_number}: 0x{}", hex::encode(block_hash.0));

        let events = subxt::events::Events::new_from_client(metadata.clone(), block_hash, api.clone()).await.unwrap();

        for (i, evt) in events.iter().enumerate() {
            match evt {
                Ok(evt) => {
                    index_event(trees.clone(), block_number, i.try_into().unwrap(), evt);
                },
                Err(error) => if let Metadata(EventNotFound(_, _)) = error {
                    println!(" 📚 Downloading new metadata.");
                    metadata = api.rpc().metadata(Some(block_hash)).await.unwrap();
                    continue 'blocks;
                }
            }
        }

        trees.root.insert("last_batch_block", &block_number.to_be_bytes()).unwrap();

        // Increment the block number.
        block_number += 1;
        // Get the new block hash.
        match api.rpc().block_hash(Some(block_number.into())).await.unwrap() {
            Some(new_hash) => block_hash = new_hash,
            None => {
                trees.root.insert("batch_indexing_complete", &1_u8.to_be_bytes()).unwrap();
                println!(" 📚 Finished batch indexing.");
                break;
            }
        }
    }
}

