//! # Hybrid Indexer
//!
//! A library for indexing events from Substrate blockchains.

use futures::StreamExt;
use signal_hook::{consts::TERM_SIGNALS, flag};
use signal_hook_tokio::Signals;
use std::{
    error::Error,
    path::PathBuf,
    process::exit,
    sync::{atomic::AtomicBool, Arc},
};
use tokio::{
    join, spawn,
    sync::{mpsc, watch},
};

pub mod shared;
pub mod substrate;
pub mod substrate_pallets;
mod websockets;

use crate::shared::*;
use substrate::*;
use websockets::websockets_listen;

use subxt::OnlineClient;

#[cfg(test)]
mod tests;

/// Starts the indexer. Chain is defined by `R`.
pub async fn start<R: RuntimeIndexer + 'static>(
    db_path: Option<String>,
    url: Option<String>,
    block_number: Option<u32>,
    queue_depth: u8,
    port: u16,
) -> Result<(), Box<dyn Error>> {
    let name = R::get_name();
    println!("Indexing {}", name);
    let genesis_hash_config = R::get_genesis_hash().as_ref().to_vec();
    // Open database.
    let db_path = match db_path {
        Some(db_path) => PathBuf::from(db_path),
        None => {
            let mut db_path = home::home_dir().ok_or("No home directory.")?;
            db_path.push(".local/share/hybrid-indexer");
            db_path.push(name);
            db_path.push("db");
            db_path
        }
    };
    println!("Opening db: {}", db_path.display());
    let db = sled::open(&db_path)?;
    let trees = Trees {
        root: db.clone(),
        variant: db.open_tree("variant")?,
        // Each event parameter to be indexed has its own tree.
        account_id: db.open_tree("account_id")?,
        account_index: db.open_tree("account_index")?,
        auction_index: db.open_tree("auction_index")?,
        bounty_index: db.open_tree("bounty_index")?,
        candidate_hash: db.open_tree("candiate_hash")?,
        era_index: db.open_tree("era_index")?,
        message_id: db.open_tree("para_id")?,
        para_id: db.open_tree("para_id")?,
        pool_id: db.open_tree("bounty_index")?,
        preimage_hash: db.open_tree("preimage_hash")?,
        proposal_hash: db.open_tree("proposal_hash")?,
        proposal_index: db.open_tree("proposal_index")?,
        ref_index: db.open_tree("ref_index")?,
        registrar_index: db.open_tree("registrar_index")?,
        session_index: db.open_tree("session_index")?,
        tip_hash: db.open_tree("tip_hash")?,
    };
    drop(db);

    let genesis_hash_db = match trees.root.get("genesis_hash").unwrap() {
        Some(value) => value.to_vec(),
        //    vector_as_u8_32_array(&value),
        None => {
            trees
                .root
                .insert("genesis_hash", genesis_hash_config.clone())
                .unwrap();
            genesis_hash_config.clone()
        }
    };

    if genesis_hash_db != genesis_hash_config {
        eprintln!("Database has wrong genesis hash.");
        eprintln!("Correct hash:  0x{}", hex::encode(genesis_hash_config));
        eprintln!("Database hash: 0x{}", hex::encode(genesis_hash_db));
        exit(1);
    }

    let url = match url {
        Some(url) => url,
        None => R::get_default_url().to_owned(),
    };
    println!("Connecting to: {}", url);
    // Determine url of Substrate node to connect to.
    let api = OnlineClient::<R::RuntimeConfig>::from_url(&url)
        .await
        .unwrap();
    let genesis_hash_api = api.genesis_hash().as_ref().to_vec();

    if genesis_hash_api != genesis_hash_config {
        eprintln!("Chain has wrong genesis hash.");
        eprintln!("Correct hash: 0x{}", hex::encode(genesis_hash_config));
        eprintln!("Chain hash:   0x{}", hex::encode(genesis_hash_api));
        exit(1);
    }
    // https://docs.rs/signal-hook/0.3.17/signal_hook/#a-complex-signal-handling-with-a-background-thread
    // Make sure double CTRL+C and similar kills.
    let term_now = Arc::new(AtomicBool::new(false));
    for sig in TERM_SIGNALS {
        // When terminated by a second term signal, exit with exit code 1.
        // This will do nothing the first time (because term_now is false).
        flag::register_conditional_shutdown(*sig, 1, Arc::clone(&term_now))?;
        // But this will "arm" the above for the second time, by setting it to true.
        // The order of registering these is important, if you put this one first, it will
        // first arm and then terminate ‒ all in the first round.
        flag::register(*sig, Arc::clone(&term_now))?;
    }
    // Create a watch channel to exit the program.
    let (exit_tx, exit_rx) = watch::channel(false);
    // Create the channel for the websockets threads to send subscribe messages to the head thread.
    let (sub_tx, sub_rx) = mpsc::unbounded_channel();
    // Start indexer thread.
    let substrate_index = spawn(substrate_index::<R>(
        trees.clone(),
        api.clone(),
        block_number,
        queue_depth.into(),
        exit_rx.clone(),
        sub_rx,
    ));
    // Spawn websockets task.
    let websockets_task = spawn(websockets_listen::<R>(
        trees.clone(),
        api,
        port,
        exit_rx,
        sub_tx,
    ));
    // Wait for signal.
    let mut signals = Signals::new(TERM_SIGNALS)?;
    signals.next().await;
    println!("Exiting.");
    let _ = exit_tx.send(true);
    // Wait to exit.
    let _result = join!(substrate_index, websockets_task);
    // Close db.
    println!("Closing db.");
    let _bytes = trees.root.flush();
    let _bytes = trees.variant.flush();
    let _bytes = trees.account_id.flush();
    let _bytes = trees.account_index.flush();
    let _bytes = trees.auction_index.flush();
    let _bytes = trees.bounty_index.flush();
    let _bytes = trees.candidate_hash.flush();
    let _bytes = trees.era_index.flush();
    let _bytes = trees.message_id.flush();
    let _bytes = trees.para_id.flush();
    let _bytes = trees.pool_id.flush();
    let _bytes = trees.preimage_hash.flush();
    let _bytes = trees.proposal_hash.flush();
    let _bytes = trees.proposal_index.flush();
    let _bytes = trees.ref_index.flush();
    let _bytes = trees.registrar_index.flush();
    let _bytes = trees.session_index.flush();
    let _bytes = trees.tip_hash.flush();
    drop(trees);
    Ok(())
}
