
use clap::Parser;

use tokio::join;

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
        account_id: db.open_tree("account_id")?,
        account_index: db.open_tree("account_index")?,
        auction_index: db.open_tree("auction_index")?,
        bounty_index: db.open_tree("bounty_index")?,
        candidate_hash: db.open_tree("candiate_hash")?,
        message_id: db.open_tree("para_id")?,
        para_id: db.open_tree("para_id")?,
        pool_id: db.open_tree("bounty_index")?,
        proposal_hash: db.open_tree("proposal_hash")?,
        proposal_index: db.open_tree("proposal_index")?,
        ref_index: db.open_tree("ref_index")?,
        registrar_index: db.open_tree("registrar_index")?,
        tip_hash: db.open_tree("tip_hash")?,
    };
    println!("Opened database.");
    let api = OnlineClient::<PolkadotConfig>::from_url(args.url.clone()).await.unwrap();
    println!("Connected to Substrate node.");

    // Start Substrate tasks.
    let substrate_head = tokio::spawn(substrate_head(api.clone(), trees.clone()));
    let substrate_batch = tokio::spawn(substrate_batch(api, trees.clone(), args));
    // Spawn websockets task.
    let websockets_task = tokio::spawn(websockets_listen(trees.clone()));
    // Wait to exit.
    let _result = join!(substrate_head, substrate_batch, websockets_task);
    Ok(())
}
