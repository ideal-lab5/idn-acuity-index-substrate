use clap::Parser;
use futures::StreamExt;
use subxt::{
    OnlineClient,
    PolkadotConfig,
    Config,
    SubstrateConfig,
    config::Header,
    utils::AccountId32,
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

impl AccountIdKey {
    pub fn serialize(&self) -> Vec<u8> {
        [
            self.account_id.0.to_vec(),
            self.block_number.to_be_bytes().to_vec(),
            self.idx.to_be_bytes().to_vec(),
            self.i.to_be_bytes().to_vec(),
        ].concat()
    }

    pub fn unserialize(vec: Vec<u8>) -> AccountIdKey {
        AccountIdKey {
            account_id: AccountId32(vector_as_u8_32_array(&vec[0..32].to_vec())),
            block_number: u32::from_be_bytes(vector_as_u8_4_array(&vec[32..36].to_vec())),
            idx: u32::from_be_bytes(vector_as_u8_4_array(&vec[36..40].to_vec())),
            i: u32::from_be_bytes(vector_as_u8_4_array(&vec[40..44].to_vec())),
        }
    }
}

pub fn vector_as_u8_32_array(vector: &Vec<u8>) -> [u8; 32] {
    let mut arr = [0u8; 32];
    for i in 0..32 {
        arr[i] = vector[i];
    }
    arr
}

pub fn vector_as_u8_4_array(vector: &Vec<u8>) -> [u8; 4] {
    let mut arr = [0u8; 4];
    for i in 0..4 {
        arr[i] = vector[i];
    }
    arr
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
                }.serialize();

                let value = db.generate_id()?.to_be_bytes();

                db.insert(key, &value)?;
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
