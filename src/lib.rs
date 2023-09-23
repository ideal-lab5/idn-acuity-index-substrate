//! # Hybrid Indexer
//!
//! A library for indexing events from Substrate blockchains.

use std::{error::Error, path::PathBuf, process::exit};
use tokio::{join, spawn, sync::mpsc::unbounded_channel};

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

    // Create the channel for the websockets threads to send subscribe messages to the head thread.
    let (sub_tx, sub_rx) = unbounded_channel();
    // Start Substrate tasks.
    let substrate_index = spawn(substrate_index::<R>(
        api.clone(),
        trees.clone(),
        block_number,
        queue_depth.into(),
        sub_rx,
    ));
    // Spawn websockets task.
    let websockets_task = spawn(websockets_listen::<R>(api, trees.clone(), sub_tx, port));
    // Wait to exit.
    let _result = join!(substrate_index, websockets_task);
    Ok(())
}
