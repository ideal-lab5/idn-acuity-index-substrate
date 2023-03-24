
use subxt::{
    OnlineClient,
    PolkadotConfig,
};

#[subxt::subxt(runtime_metadata_path = "metadata.scale")]
pub mod polkadot {}

use crate::shared::*;

fn index_event(db: sled::Db, block_number: u32, event_index: u32, event: subxt::events::EventDetails) {
    let pallet_name = event.pallet_name();
    let event_name = event.variant_name();

    println!("        {pallet_name}_{event_name}");

    match event.pallet_name() {
        "Balances" => match event.variant_name() {
            "Transfer" => {
                let transfer_event = event.as_event::<polkadot::balances::events::Transfer>().unwrap();

                if let Some(ev) = transfer_event {
                    println!("From: {:}", ev.from);
                    println!("To: {:}", ev.to);
                    println!("Amount: {:}", ev.amount);

                    let key_from = AccountIdKey {
                        account_id: ev.from.clone(),
                        block_number: block_number,
                        i: event_index,
                    }.serialize();

                    let key_to = AccountIdKey {
                        account_id: ev.to.clone(),
                        block_number: block_number,
                        i: event_index,
                    }.serialize();

                    let value = event.bytes();

                    db.insert(key_from, value.clone()).unwrap();
                    db.insert(key_to, value).unwrap();
                }
            },
            _ => {},
        },
        _ => {},
    }
}

pub async fn substrate_listen(db: sled::Db, args: Args) {
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
                    index_event(db.clone(), block_number, i, evt);
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
