use clap::Parser;
use futures::StreamExt;
use subxt::{
    OnlineClient,
    PolkadotConfig,
};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
   /// URL of Substrate node to connect to.
   #[arg(short, long)]
   url: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Check command line parameters.
    let args = Args::parse();
    // Open database.
    let path = "db";
    let db = sled::open(path)?;
    println!("Opened database.");
    // Connect to Substrate node.
    let api = OnlineClient::<PolkadotConfig>::from_url(args.url).await.unwrap();
    println!("Connected to Substrate node.");

    // Subscribe to all finalized blocks:
    let mut blocks_sub = api.blocks().subscribe_finalized().await?;

    while let Some(block) = blocks_sub.next().await {
        let block = block?;

        let block_number = block.header().number;
        let block_hash = block.hash();

        println!("Block #{block_number}:");
        println!("  Hash: {block_hash}");
        println!("  Extrinsics:");

        let body = block.body().await?;
        for ext in body.extrinsics() {
            let idx = ext.index();
            let events = ext.events().await?;

            println!("    Extrinsic #{idx}:");
            println!("      Events:");

            for evt in events.iter() {
                let evt = evt?;

                let pallet_name = evt.pallet_name();
                let event_name = evt.variant_name();

                println!("        {pallet_name}_{event_name}");
            }
        }
    }

    Ok(())
}
