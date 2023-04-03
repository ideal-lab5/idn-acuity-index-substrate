
use clap::Parser;

use tokio::join;

mod shared;
mod substrate;
mod websockets;
mod pallets;

use crate::shared::*;
use substrate::substrate_listen;
use websockets::websockets_listen;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Check command line parameters.
    let args = Args::parse();
    // Open database.
    let path = "db";
    let db = sled::open(path)?;
    let trees = Trees {
        account_id: db.open_tree("account_id")?,
        account_index: db.open_tree("account_index")?,
        auction_index: db.open_tree("auction_index")?,
        bounty_index: db.open_tree("bounty_index")?,
        para_id: db.open_tree("para_id")?,
        pool_id: db.open_tree("bounty_index")?,
        proposal_hash: db.open_tree("proposal_hash")?,
        proposal_index: db.open_tree("proposal_index")?,
        ref_index: db.open_tree("ref_index")?,
        registrar_index: db.open_tree("registrar_index")?,
        tip_hash: db.open_tree("tip_hash")?,
    };
    println!("Opened database.");
    // Start Substrate task.
    let substrate_task = tokio::spawn(substrate_listen(trees.clone(), args));
    // Spawn websockets task.
    let websockets_task = tokio::spawn(websockets_listen(trees.clone()));
    // Wait to exit.
    let _result = join!(substrate_task, websockets_task);
    Ok(())
}
