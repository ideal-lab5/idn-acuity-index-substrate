use clap::Parser;
use futures::StreamExt;
use subxt::{
    OnlineClient,
    PolkadotConfig,
    Config,
    SubstrateConfig,
    config::Header,
};

#[subxt::subxt(runtime_metadata_path = "metadata.scale")]
pub mod polkadot {}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
   /// URL of Substrate node to connect to.
   #[arg(short, long)]
   url: String,
}

struct AccountIdKey {
    account_id: <SubstrateConfig as Config>::AccountId,
    block_number: <<SubstrateConfig as Config>::Header as Header>::Number,
    idx: u32,
    i: u32,
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


            let transfer_event = events.find_first::<polkadot::balances::events::Transfer>()?;

            if let Some(ev) = transfer_event {
                println!("From: {:?}", ev.from);
                println!("To: {:?}", ev.to);
                println!("Amount: {:?}", ev.amount);

                let key = AccountIdKey {
                    account_id: ev.from,
                    block_number: block_number,
                    idx: idx,
                    i: 0,
                };

            } else {
                println!("  - No balance transfer event found in this xt");
            }

/*
            for evt in events.iter() {
                let evt = evt?;

                let pallet_name = evt.pallet_name();
                let event_name = evt.variant_name();

                println!("        {pallet_name}_{event_name}");

                if pallet_name != "Balances" { continue; }
                if event_name != "Transfer" { continue; }


                println!("balance: {:?}", evt.amount);

  */
        }
    }

    Ok(())
}
