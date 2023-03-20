
use futures::StreamExt;
use subxt::{
    OnlineClient,
    PolkadotConfig,
    Config,
    SubstrateConfig,
    config::Header,
    utils::AccountId32,
};

#[subxt::subxt(runtime_metadata_path = "metadata.scale")]
pub mod polkadot {}

use crate::shared::*;

pub async fn substrate_listen(db: sled::Db, args: Args) {
    // Connect to Substrate node.
    let api = OnlineClient::<PolkadotConfig>::from_url(args.url).await.unwrap();
    println!("Connected to Substrate node.");

    // Subscribe to all finalized blocks:
    let mut blocks_sub = api.blocks().subscribe_finalized().await.unwrap();

    while let Some(block) = blocks_sub.next().await {
        let block = block.unwrap();

        let block_number = block.header().number;
        let block_hash = block.hash();

        println!("Block #{block_number}:");
        println!("  Hash: {block_hash}");
        println!("  Extrinsics:");

        let body = block.body().await.unwrap();
        for ext in body.extrinsics() {
            let idx = ext.index();
            let events = ext.events().await.unwrap();

            println!("    Extrinsic #{idx}:");
            println!("      Events:");


            let transfer_event = events.find_first::<polkadot::balances::events::Transfer>().unwrap();

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
            } else {
                println!("  - No balance transfer event found in this xt");
            }

/*
            for evt in events.iter() {
                let evt = evt?;

                let pallet_name = evt.pallet_name();
                let event_name = evt.variant_name();

                println!("        {pallet_name}_{event_name}");

                if pallet_name != "Balances" { continue; }
                if event_name != "Transfer" { continue; }


                println!("balance: {:?}", evt.amount);

  */
        }
    }
}
