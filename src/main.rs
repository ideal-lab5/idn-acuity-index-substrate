
use clap::Parser;

use tokio::join;

mod shared;
mod substrate;
mod websockets;

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
        registrar_index: db.open_tree("registrar_index")?,
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
