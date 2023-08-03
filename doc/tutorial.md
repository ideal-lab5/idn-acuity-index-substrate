# Tutorial

To learn how to build an indexer for a Substrate chain with Hybrid Indexer it is best to examine Polkadot Indexer.

Follow the subxt [instructions](https://github.com/paritytech/subxt#downloading-metadata-from-a-substrate-node) to download the metadata from the chain to be indexed:

```
subxt metadata --url <URL> > metadata.scale
```

It's generally a good idea to have the metadata in a separate workspace from the indexer. This avoids lengthly rebuilds during development.

'Cargo.toml'
```toml
[workspace]
resolver = "2"

members = [
  "metadata",
  "indexer",
]
```

'metadata/Cargo.toml'
```toml
[package]
name = "metadata"
version = "0.1.0"
edition = "2021"

[dependencies]
subxt = "0.30.1"
```

`metadata/src/lib/rs`
```rust
#[subxt::subxt(runtime_metadata_path = "metadata.scale")]
pub mod metadata {}
```

Copy and modify the boilerplate `Cargo.toml` and `main.rs` from polkadot-indexer.

Create a struct that implements the subxt [Config](https://docs.rs/subxt/latest/subxt/config/trait.Config.html) trait. For example:

```rust
pub enum MyChainConfig {}

impl Config for MyChainConfig {
    type Hash = H256;
    type AccountId = AccountId32;
    type Address = MultiAddress<Self::AccountId, u32>;
    type Signature = MultiSignature;
    type Hasher = BlakeTwo256;
    type Header = SubstrateHeader<u32, BlakeTwo256>;
    type ExtrinsicParams = SubstrateExtrinsicParams<Self>;
}
```

Each chain to be indexed by the indexer has a separate struct that implements the RuntimeIndexer trait. For example, look at [KusamaIndexer](https://github.com/hybrid-explorer/polkadot-indexer/blob/main/indexer/src/kusama/mod.rs#L40).

subxt may not be able to process older blocks. Experiment with get_default_start_block()

Every event to be indexed is passed to `process_event()`. It needs to determine which pallet the event is from and use the correct macro to index it. Macros for Substrate pallets are provided by hybrid-indexer. Additional pallet macros can be provided.

```rust
impl hybrid_indexer::shared::RuntimeIndexer for MyChainIndexer {
    type RuntimeConfig = MyChainConfig;

    fn get_name() -> &'static str {
        "mychain"
    }

    fn get_default_url() -> &'static str {
        "wss://rpc.mychain.io:443"
    }

    fn get_default_start_block() -> u32 {
        0
    }

    fn process_event(
        indexer: &hybrid_indexer::substrate::Indexer<Self>,
        block_number: u32,
        event_index: u32,
        event: subxt::events::EventDetails<Self::RuntimeConfig>,
    ) -> Result<(), subxt::Error> {
        match event.as_root_event::<Event>()? {
            // Substrate pallets.
            Event::Balances(event) => {
                index_balances_event![BalancesEvent, event, indexer, block_number, event_index]
            }
            // MyChain pallets.
            Event::MyPallet(event) => {
                index_mypallet_event![MyPalletEvent, event, indexer, block_number, event_index]
            }
            _ => {}
        };
        Ok(())
    }
```

Custom pallet indexer macros look something like this:

```rust
#[macro_export]
macro_rules! index_mypallet_event {
    ($event_enum: ty, $event: ident, $indexer: ident, $block_number: ident, $event_index: ident) => {
        match $event {
            <$event_enum>::MyEvent { who, .. } => {
                $indexer.index_event_account_id(who, $block_number, $event_index);
            }
        }
    };
}
```