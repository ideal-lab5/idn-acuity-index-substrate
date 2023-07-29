use tokio::{join, sync::mpsc};

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

pub async fn start<R: RuntimeIndexer + std::marker::Send + std::marker::Sync + 'static>(
    url: Option<String>,
    block_number: Option<u32>,
    async_blocks: Option<u32>,
) -> Result<(), Box<dyn std::error::Error>> {
    let name = R::get_name();
    println!("Indexing {}", name);
    // Open database.
    let mut path = home::home_dir().ok_or("No home directory.")?;
    path.push(".local/share/hybrid-indexer");
    path.push(name);
    path.push("db");
    println!("Opening db: {}", path.display());
    let db = sled::open(&path)?;
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
    let url = match url {
        Some(url) => url,
        None => R::get_url().to_owned(),
    };
    // Determine url of Substrate node to connect to.
    let api = OnlineClient::<R::RuntimeConfig>::from_url(&url)
        .await
        .unwrap();
    println!("Connected to: {}", url);
    // Create the channel for the websockets threads to send subscribe messages to the head thread.
    let (sub_tx, sub_rx) = mpsc::unbounded_channel();
    // Start Substrate tasks.
    let substrate_head = tokio::spawn(substrate_head::<R>(api.clone(), trees.clone(), sub_rx));
    let substrate_batch = tokio::spawn(substrate_batch::<R>(
        api.clone(),
        trees.clone(),
        block_number,
        async_blocks,
    ));
    // Spawn websockets task.
    let websockets_task = tokio::spawn(websockets_listen::<R>(api, trees.clone(), sub_tx));
    // Wait to exit.
    let _result = join!(substrate_head, substrate_batch, websockets_task);
    Ok(())
}
