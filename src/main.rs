
use clap::Parser;

use tokio::{
    join,
    sync::mpsc,
};

mod shared;
mod substrate;
mod websockets;
mod pallets;

use crate::shared::*;
use substrate::*;
use websockets::websockets_listen;

use subxt::{
    OnlineClient,
    PolkadotConfig,
};

#[cfg(test)]
mod tests;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Check command line parameters.
    let args = Args::parse();
    // Open database.
    let path = "db";
    let db = sled::open(path)?;
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
    println!("Opened database.");
    // Determine url of Substrate node to connect to.
    let url = args.url.clone().unwrap_or_else(|| "wss://rpc.polkadot.io:443".to_string());
    let api = OnlineClient::<PolkadotConfig>::from_url(url).await.unwrap();
    println!("Connected to Substrate node.");

    // Create the channel for the websockets threads to send subscribe messages to the head thread.
    let (sub_tx, sub_rx) = mpsc::unbounded_channel();
    
    // Start Substrate tasks.
    let substrate_head = tokio::spawn(substrate_head(api.clone(), trees.clone(), sub_rx));
    let substrate_batch = tokio::spawn(substrate_batch(api.clone(), trees.clone(), args));
    // Spawn websockets task.
    let websockets_task = tokio::spawn(websockets_listen(api, trees.clone(), sub_tx));
    // Wait to exit.
    let _result = join!(substrate_head, substrate_batch, websockets_task);
    Ok(())
}
