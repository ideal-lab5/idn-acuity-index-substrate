
use subxt::{
    OnlineClient,
    PolkadotConfig,
    utils::AccountId32,
};


use crate::shared::*;

fn index_event_account_id(db: sled::Db, account_id: AccountId32, block_number: u32, i: u32, bytes: &[u8]) {
    println!("AccountId: {:}", account_id);
    // Generate key
    let key = AccountIdKey {
        account_id: account_id,
        block_number: block_number,
        i: i,
    }.serialize();
    // Insert record.
    db.insert(key, bytes).unwrap();
}

fn index_event(db: sled::Db, block_number: u32, event_index: u32, event: subxt::events::EventDetails) {

    match event.pallet_name() {
        "Balances" => match event.variant_name() {
            "Endowed" => {
                let endowed_event = event.as_event::<polkadot::balances::events::Endowed>().unwrap().unwrap();
                let event = Event::Balances(
                    Balances::Endowed {
                        account: endowed_event.account.clone(),
                        free_balance: endowed_event.free_balance.clone(),
                    }
                );
                let value = bincode::encode_to_vec(&event, bincode::config::standard()).unwrap();
                index_event_account_id(db.clone(), endowed_event.account, block_number, event_index, &value);
            },
            "DustLost" => {
                let dustlost_event = event.as_event::<polkadot::balances::events::DustLost>().unwrap().unwrap();
                let event = Event::Balances(
                    Balances::DustLost {
                        account: dustlost_event.account.clone(),
                        amount: dustlost_event.amount,
                    }
                );
                let value = bincode::encode_to_vec(&event, bincode::config::standard()).unwrap();
                index_event_account_id(db.clone(), dustlost_event.account, block_number, event_index, &value);
            },
            "Transfer" => {
                let transfer_event = event.as_event::<polkadot::balances::events::Transfer>().unwrap().unwrap();
                let event = Event::Balances(
                    Balances::Transfer {
                        from: transfer_event.from.clone(),
                        to: transfer_event.to.clone(),
                        value: transfer_event.amount,
                    }
                );
                let value = bincode::encode_to_vec(&event, bincode::config::standard()).unwrap();
                index_event_account_id(db.clone(), transfer_event.from, block_number, event_index, &value);
                index_event_account_id(db.clone(), transfer_event.to, block_number, event_index, &value);
            },
            "BalanceSet" => {
                let balance_set_event = event.as_event::<polkadot::balances::events::BalanceSet>().unwrap().unwrap();
                let event = Event::Balances(
                    Balances::BalanceSet {
	                    who: balance_set_event.who.clone(),
	                    free: balance_set_event.free,
	                    reserved: balance_set_event.reserved,
                    }
                );
                let value = bincode::encode_to_vec(&event, bincode::config::standard()).unwrap();
                index_event_account_id(db.clone(), balance_set_event.who, block_number, event_index, &value);
            },
            "Reserved" => {
                let reserved_event = event.as_event::<polkadot::balances::events::Reserved>().unwrap().unwrap();
                let event = Event::Balances(
                    Balances::Reserved {
	                    who: reserved_event.who.clone(),
	                    amount: reserved_event.amount,
                    }
                );
                let value = bincode::encode_to_vec(&event, bincode::config::standard()).unwrap();
                index_event_account_id(db.clone(), reserved_event.who, block_number, event_index, &value);
            },
            "Unreserved" => {
                let unreserved_event = event.as_event::<polkadot::balances::events::Unreserved>().unwrap().unwrap();
                let event = Event::Balances(
                    Balances::Unreserved {
	                    who: unreserved_event.who.clone(),
	                    amount: unreserved_event.amount,
                    }
                );
                let value = bincode::encode_to_vec(&event, bincode::config::standard()).unwrap();
                index_event_account_id(db.clone(), unreserved_event.who, block_number, event_index, &value);
            },
            "ReserveRepatriated" => {
                let reserve_repatriated_event = event.as_event::<polkadot::balances::events::ReserveRepatriated>().unwrap().unwrap();
                let event = Event::Balances(
                    Balances::ReserveRepatriated {
		                from: reserve_repatriated_event.from.clone(),
		                to: reserve_repatriated_event.to.clone(),
		                amount: reserve_repatriated_event.amount,
		                destination_status: Status::from(&reserve_repatriated_event.destination_status),
	                }
	            );
                let value = bincode::encode_to_vec(&event, bincode::config::standard()).unwrap();
                index_event_account_id(db.clone(), reserve_repatriated_event.from, block_number, event_index, &value);
                index_event_account_id(db.clone(), reserve_repatriated_event.to, block_number, event_index, &value);
            },
            "Deposit" => {
                let deposit_event = event.as_event::<polkadot::balances::events::Deposit>().unwrap().unwrap();
                let event = Event::Balances(
                    Balances::Deposit {
	                    who: deposit_event.who.clone(),
	                    amount: deposit_event.amount,
                    }
                );
                let value = bincode::encode_to_vec(&event, bincode::config::standard()).unwrap();
                index_event_account_id(db.clone(), deposit_event.who, block_number, event_index, &value);
            },
            "Withdraw" => {
                let withdraw_event = event.as_event::<polkadot::balances::events::Withdraw>().unwrap().unwrap();
                let event = Event::Balances(
                    Balances::Withdraw {
	                    who: withdraw_event.who.clone(),
	                    amount: withdraw_event.amount,
                    }
                );
                let value = bincode::encode_to_vec(&event, bincode::config::standard()).unwrap();
                index_event_account_id(db.clone(), withdraw_event.who, block_number, event_index, &value);
            },
            "Slashed" => {
                let slashed_event = event.as_event::<polkadot::balances::events::Slashed>().unwrap().unwrap();
                let event = Event::Balances(
                    Balances::Slashed {
	                    who: slashed_event.who.clone(),
	                    amount: slashed_event.amount,
                    }
                );
                let value = bincode::encode_to_vec(&event, bincode::config::standard()).unwrap();
                index_event_account_id(db.clone(), slashed_event.who, block_number, event_index, &value);
            },
            _ => {},
        },
        "Identity" => match event.variant_name() {
            "IdentitySet" => {
                let event = event.as_event::<polkadot::identity::events::IdentitySet>().unwrap().unwrap();
                let event_db = Event::Identity(
                    Identity::IdentitySet {
                        who: event.who.clone(),
                    }
                );
                let value = bincode::encode_to_vec(&event_db, bincode::config::standard()).unwrap();
                index_event_account_id(db.clone(), event.who.clone(), block_number, event_index, &value);
            },
            "IdentityCleared" => {
                let event = event.as_event::<polkadot::identity::events::IdentityCleared>().unwrap().unwrap();
                let event_db = Event::Identity(
                    Identity::IdentityCleared {
                        who: event.who.clone(),
                        deposit: event.deposit,
                    }
                );
                let value = bincode::encode_to_vec(&event_db, bincode::config::standard()).unwrap();
                index_event_account_id(db.clone(), event.who, block_number, event_index, &value);
            },
            "IdentityKilled" => {
                let event = event.as_event::<polkadot::identity::events::IdentityKilled>().unwrap().unwrap();
                let event_db = Event::Identity(
                    Identity::IdentityKilled {
                        who: event.who.clone(),
                        deposit: event.deposit,
                    }
                );
                let value = bincode::encode_to_vec(&event_db, bincode::config::standard()).unwrap();
                index_event_account_id(db.clone(), event.who, block_number, event_index, &value);
            },
            "JudgementRequested" => {
                let event = event.as_event::<polkadot::identity::events::JudgementRequested>().unwrap().unwrap();
                let event_db = Event::Identity(
                    Identity::JudgementRequested {
                        who: event.who.clone(),
                        registrar_index: event.registrar_index,
                    }
                );
                let value = bincode::encode_to_vec(&event_db, bincode::config::standard()).unwrap();
                index_event_account_id(db.clone(), event.who, block_number, event_index, &value);
            },
            "JudgementUnrequested" => {
                let event = event.as_event::<polkadot::identity::events::JudgementUnrequested>().unwrap().unwrap();
                let event_db = Event::Identity(
                    Identity::JudgementUnrequested {
                        who: event.who.clone(),
                        registrar_index: event.registrar_index,
                    }
                );
                let value = bincode::encode_to_vec(&event_db, bincode::config::standard()).unwrap();
                index_event_account_id(db.clone(), event.who, block_number, event_index, &value);
            },
            "JudgementGiven" => {
                let event = event.as_event::<polkadot::identity::events::JudgementGiven>().unwrap().unwrap();
                let event_db = Event::Identity(
                    Identity::JudgementGiven {
                        target: event.target.clone(),
                        registrar_index: event.registrar_index,
                    }
                );
                let value = bincode::encode_to_vec(&event_db, bincode::config::standard()).unwrap();
                index_event_account_id(db.clone(), event.target, block_number, event_index, &value);
            },
            "RegistrarAdded" => {
                let event = event.as_event::<polkadot::identity::events::RegistrarAdded>().unwrap().unwrap();
                let event_db = Event::Identity(
                    Identity::RegistrarAdded {
                        registrar_index: event.registrar_index,
                    }
                );
                let value = bincode::encode_to_vec(&event_db, bincode::config::standard()).unwrap();
//                index_event_account_id(db.clone(), event.who, block_number, event_index, &value);
            },
            "SubIdentityAdded" => {
                let event = event.as_event::<polkadot::identity::events::SubIdentityAdded>().unwrap().unwrap();
                let event_db = Event::Identity(
                    Identity::SubIdentityAdded {
                        sub: event.sub.clone(),
                        main: event.main.clone(),
                        deposit: event.deposit,
                    }
                );
                let value = bincode::encode_to_vec(&event_db, bincode::config::standard()).unwrap();
                index_event_account_id(db.clone(), event.sub, block_number, event_index, &value);
                index_event_account_id(db.clone(), event.main, block_number, event_index, &value);
            },
            "SubIdentityRemoved" => {
                let event = event.as_event::<polkadot::identity::events::SubIdentityRemoved>().unwrap().unwrap();
                let event_db = Event::Identity(
                    Identity::SubIdentityRemoved {
                        sub: event.sub.clone(),
                        main: event.main.clone(),
                        deposit: event.deposit,
                    }
                );
                let value = bincode::encode_to_vec(&event_db, bincode::config::standard()).unwrap();
                index_event_account_id(db.clone(), event.sub, block_number, event_index, &value);
                index_event_account_id(db.clone(), event.main, block_number, event_index, &value);
            },
            "SubIdentityRevoked" => {
                let event = event.as_event::<polkadot::identity::events::SubIdentityRevoked>().unwrap().unwrap();
                let event_db = Event::Identity(
                    Identity::SubIdentityRevoked {
                        sub: event.sub.clone(),
                        main: event.main.clone(),
                        deposit: event.deposit,
                    }
                );
                let value = bincode::encode_to_vec(&event_db, bincode::config::standard()).unwrap();
                index_event_account_id(db.clone(), event.sub, block_number, event_index, &value);
                index_event_account_id(db.clone(), event.main, block_number, event_index, &value);
            },
            &_ => {},
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
