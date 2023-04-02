
use subxt::{
    OnlineClient,
    PolkadotConfig,
    utils::AccountId32,
};


use crate::shared::*;
use crate::pallets::balances::*;
use crate::pallets::bounties::*;
use crate::pallets::child_bounties::*;
use crate::pallets::claims::*;
use crate::pallets::collective::*;
use crate::pallets::democracy::*;
use crate::pallets::election_provider_multi_phase::*;
use crate::pallets::elections_phragmen::*;
use crate::pallets::identity::*;
use crate::pallets::indices::*;
use crate::pallets::multisig::*;
use crate::pallets::proxy::*;
use crate::pallets::system::*;
use crate::pallets::tips::*;
use crate::pallets::transaction_payment::*;
use crate::pallets::treasury::*;
use crate::pallets::vesting::*;

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
        "Balances" => balance_index_event(trees, block_number, event_index, event),
        "Bounties" => bounties_index_event(trees, block_number, event_index, event),
        "ChildBounties" => child_bounties_index_event(trees, block_number, event_index, event),
        "Claims" => claims_index_event(trees, block_number, event_index, event),
        "Collective" => collective_index_event(trees, block_number, event_index, event),
        "Democracy" => democracy_index_event(trees, block_number, event_index, event),
        "ElectionProviderMultiPhase" => election_provider_multi_phase_index_event(trees, block_number, event_index, event),
        "Identity" => identity_index_event(trees, block_number, event_index, event),
        "Indices" => indices_index_event(trees, block_number, event_index, event),
        "Multisig" => multisig_index_event(trees, block_number, event_index, event),
        "PhragmenElection" => elections_phragmen_index_event(trees, block_number, event_index, event),
        "Proxy" => proxy_index_event(trees, block_number, event_index, event),
        "System" => system_index_event(trees, block_number, event_index, event),
        "Tips" => tips_index_event(trees, block_number, event_index, event),
        "TransactionPayment" => transaction_payment_index_event(trees, block_number, event_index, event),
        "Treasury" => treasury_index_event(trees, block_number, event_index, event),
        "Vesting" => vesting_index_event(trees, block_number, event_index, event),
        _ => {},
    }
}

pub async fn substrate_listen(trees: Trees, args: Args) {
    let api = OnlineClient::<PolkadotConfig>::from_url(args.url).await.unwrap();
    println!("Connected to Substrate node.");

    let mut block_number: u32 = args.block_height;

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

        block_number += 1;
    }
/*
    return;

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

            for evt in events.iter() {
                let evt = evt.unwrap();

                let pallet_name = evt.pallet_name();
                let event_name = evt.variant_name();

println!("        {pallet_name}_{event_name}");

                match evt.pallet_name() {
                    "Balances" => match evt.variant_name() {
                        "Transfer" => {
                            let transfer_event = evt.as_event::<polkadot::balances::events::Transfer>().unwrap();

                            if let Some(ev) = transfer_event {
                                println!("From: {:}", ev.from);
                                println!("To: {:}", ev.to);
                                println!("Amount: {:}", ev.amount);

                                let key_from = AccountIdKey {
                                    account_id: ev.from.clone(),
                                    block_number: block_number,
                                    idx: idx,
                                    i: 0,
                                }.serialize();

                                let key_to = AccountIdKey {
                                    account_id: ev.to.clone(),
                                    block_number: block_number,
                                    idx: idx,
                                    i: 0,
                                }.serialize();

                                let value = TransferEventValue {
                                    from: ev.from,
                                    to: ev.to,
                                    value: ev.amount,
                                }.serialize();

                                db.insert(key_from, value.clone()).unwrap();
                                db.insert(key_to, value).unwrap();
                            }
                         },
                        _ => {},
                    },
                    _ => {},
                }
            }
        }
    }
        */
}
