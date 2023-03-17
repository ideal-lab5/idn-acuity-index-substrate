
use clap::Parser;

use tokio::join;

mod shared;
mod substrate;

use substrate::substrate_listen;
use crate::shared::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Check command line parameters.
    let args = Args::parse();
    // Open database.
    let path = "db";
    let db = sled::open(path)?;
    println!("Opened database.");
    // Start Substrate task.
    let substrate_task = tokio::spawn(substrate_listen(db.clone(), args));
    // Wait to exit.
    let _result = join!(substrate_task);
    Ok(())
}
