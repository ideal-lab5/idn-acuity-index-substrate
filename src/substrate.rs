
use subxt::{
    OnlineClient,
    PolkadotConfig,
    utils::AccountId32,
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
    println!("AccountId: {:}", account_id);
    // Generate key
    let key = AccountIdKey {
        account_id: account_id,
        block_number: block_number,
        i: i,
    }.serialize();
    // Insert record.
    trees.account_id.insert(key, bytes).unwrap();
}

pub fn index_event_account_index(trees: Trees, account_index: u32, block_number: u32, i: u32, bytes: &[u8]) {
    println!("AccountIndex: {:}", account_index);
    // Generate key
    let key = AccountIndexKey {
        account_index: account_index,
        block_number: block_number,
        i: i,
    }.serialize();
    // Insert record.
    trees.account_index.insert(key, bytes).unwrap();
}

pub fn index_event_auction_index(trees: Trees, auction_index: u32, block_number: u32, i: u32, bytes: &[u8]) {
    println!("AuctionIndex: {:}", auction_index);
    // Generate key
    let key = AuctionIndexKey {
        auction_index: auction_index,
        block_number: block_number,
        i: i,
    }.serialize();
    // Insert record.
    trees.auction_index.insert(key, bytes).unwrap();
}

pub fn index_event_bounty_index(trees: Trees, bounty_index: u32, block_number: u32, i: u32, bytes: &[u8]) {
    println!("BountyIndex: {:}", bounty_index);
    // Generate key
    let key = BountyIndexKey {
        bounty_index: bounty_index,
        block_number: block_number,
        i: i,
    }.serialize();
    // Insert record.
    trees.bounty_index.insert(key, bytes).unwrap();
}

pub fn index_event_candidate_hash(trees: Trees, candidate_hash: [u8; 32], block_number: u32, i: u32, bytes: &[u8]) {
//    println!("CandidateHash: {:}", candidate_hash);
    // Generate key
    let key = CandidateHashKey {
        candidate_hash: candidate_hash,
        block_number: block_number,
        i: i,
    }.serialize();
    // Insert record.
    trees.candidate_hash.insert(key, bytes).unwrap();
}

pub fn index_event_message_id(trees: Trees, message_id: [u8; 32], block_number: u32, i: u32, bytes: &[u8]) {
//  println!("MessageId: {:}", message_id);
    // Generate key
    let key = MessageIdKey {
        message_id: message_id,
        block_number: block_number,
        i: i,
    }.serialize();
    // Insert record.
    trees.message_id.insert(key, bytes).unwrap();
}

pub fn index_event_para_id(trees: Trees, para_id: u32, block_number: u32, i: u32, bytes: &[u8]) {
    println!("ParaId: {:}", para_id);
    // Generate key
    let key = ParaIdKey {
        para_id: para_id,
        block_number: block_number,
        i: i,
    }.serialize();
    // Insert record.
    trees.para_id.insert(key, bytes).unwrap();
}

pub fn index_event_pool_id(trees: Trees, pool_id: u32, block_number: u32, i: u32, bytes: &[u8]) {
    println!("PoolId: {:}", pool_id);
    // Generate key
    let key = PoolIdKey {
        pool_id: pool_id,
        block_number: block_number,
        i: i,
    }.serialize();
    // Insert record.
    trees.pool_id.insert(key, bytes).unwrap();
}

pub fn index_event_proposal_hash(trees: Trees, proposal_hash: [u8; 32], block_number: u32, i: u32, bytes: &[u8]) {
//    println!("ProposalHash: {:}", proposal_hash);
    // Generate key
    let key = ProposalHashKey {
        proposal_hash: proposal_hash,
        block_number: block_number,
        i: i,
    }.serialize();
    // Insert record.
    trees.proposal_hash.insert(key, bytes).unwrap();
}

pub fn index_event_proposal_index(trees: Trees, proposal_index: u32, block_number: u32, i: u32, bytes: &[u8]) {
    println!("ProposalIndex: {:}", proposal_index);
    // Generate key
    let key = ProposalIndexKey {
        proposal_index: proposal_index,
        block_number: block_number,
        i: i,
    }.serialize();
    // Insert record.
    trees.proposal_index.insert(key, bytes).unwrap();
}

pub fn index_event_ref_index(trees: Trees, ref_index: u32, block_number: u32, i: u32, bytes: &[u8]) {
    println!("RefIndex: {:}", ref_index);
    // Generate key
    let key = RefIndexKey {
        ref_index: ref_index,
        block_number: block_number,
        i: i,
    }.serialize();
    // Insert record.
    trees.ref_index.insert(key, bytes).unwrap();
}

pub fn index_event_registrar_index(trees: Trees, registrar_index: u32, block_number: u32, i: u32, bytes: &[u8]) {
    println!("RegistrarIndex: {:}", registrar_index);
    // Generate key
    let key = RegistrarIndexKey {
        registrar_index: registrar_index,
        block_number: block_number,
        i: i,
    }.serialize();
    // Insert record.
    trees.registrar_index.insert(key, bytes).unwrap();
}

pub fn index_event_tip_hash(trees: Trees, tip_hash: [u8; 32], block_number: u32, i: u32, bytes: &[u8]) {
//    println!("TipHash: {:}", tip_hash);
    // Generate key
    let key = TipHashKey {
        tip_hash: tip_hash,
        block_number: block_number,
        i: i,
    }.serialize();
    // Insert record.
    trees.tip_hash.insert(key, bytes).unwrap();
}

fn index_event(trees: Trees, block_number: u32, event_index: u32, event: subxt::events::EventDetails) {

    match event.pallet_name() {
        "Auctions" => auctions_index_event(trees, block_number, event_index, event),
        "BagsList" => bags_list_index_event(trees, block_number, event_index, event),
        "Balances" => balance_index_event(trees, block_number, event_index, event),
        "Bounties" => bounties_index_event(trees, block_number, event_index, event),
        "ChildBounties" => child_bounties_index_event(trees, block_number, event_index, event),
        "Claims" => claims_index_event(trees, block_number, event_index, event),
        "Collective" => collective_index_event(trees, block_number, event_index, event),
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
        _ => {},
    }
}

/*
pub async fn substrate_listen(trees: Trees, args: Args) {
    let api = OnlineClient::<PolkadotConfig>::from_url(args.url).await.unwrap();
    println!("Connected to Substrate node.");

    // Subscribe to all finalized blocks:
    let mut blocks_sub = api.blocks().subscribe_finalized().await.unwrap();

    while let Some(block) = blocks_sub.next().await {
        let block = block.unwrap();

        let block_number = block.header().number;
        let block_hash = block.hash();

        // Fetch the metadata of the given block.
//        let metadata = api.rpc().metadata(Some(block_hash)).await.unwrap();
//        let events = Events::new_from_client(metadata, block_hash, api.clone()).await.unwrap();

        println!("Block #{block_number}:");
        println!("  Hash: {block_hash}");
        println!("  Extrinsics:");

        let body = block.body().await.unwrap();
        for ext in body.extrinsics() {
            let idx = ext.index();
            let events = ext.events().await.unwrap();

            println!("    Extrinsic #{idx}:");
            println!("      Events:");

            let mut i = 0;

            for evt in events.iter() {
    //            println!("Event: {:#?}", evt.unwrap().field_values().unwrap());

                match evt {
                    Ok(evt) => {
                        index_event(trees.clone(), block_number, i, evt);
                    },
                    _ => {},
                }

                i += 1;
            }
        }
    }
}
*/

pub async fn substrate_listen(trees: Trees, args: Args) {
    let api = OnlineClient::<PolkadotConfig>::from_url(args.url).await.unwrap();
    println!("Connected to Substrate node.");

    let mut block_number: u32 = match args.block_height {
        Some(block_height) => block_height,
        None => {
            match trees.root.get("last_block").unwrap() {
                Some(value) => u32::from_be_bytes(vector_as_u8_4_array(&value.to_vec())),
                None => 0,
            }
        }
    };

    loop {
        let block_hash = api
            .rpc()
            .block_hash(Some(block_number.into()))
            .await.unwrap()
            .expect("didn't pass a block number; qed");

        println!("Block #{block_number}:");
        println!("  Hash: {}", hex::encode(block_hash.0));

        // Fetch the metadata of the given block.
//        let metadata = api.rpc().metadata(Some(block_hash)).await.unwrap();
//        let events = Events::new_from_client(metadata, block_hash, api.clone()).await.unwrap();


        let events = api.events().at(Some(block_hash)).await.unwrap();

        let mut i = 0;

        for evt in events.iter() {
//            println!("Event: {:#?}", evt.unwrap().field_values().unwrap());

            match evt {
                Ok(evt) => {
                    index_event(trees.clone(), block_number, i, evt);
                },
                _ => {},
            }

            i += 1;
        }

        trees.root.insert("last_block", &block_number.to_be_bytes()).unwrap();

        block_number += 1;
    }
}

