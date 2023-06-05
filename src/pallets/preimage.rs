use subxt::PolkadotConfig;
use crate::shared::*;
use crate::substrate::*;

pub fn preimage_index_event(indexer: &Indexer, block_number: u32, event_index: u32, event: subxt::events::EventDetails<PolkadotConfig>) -> Result<(), subxt::Error> {
    match event.variant_name() {
        "Noted" => {
            let event = event.as_event::<polkadot::preimage::events::Noted>()?.unwrap();
            indexer.index_event_preimage_hash(event.hash.into(), block_number, event_index);
            Ok(())
        },
        "Requested" => {
            let event = event.as_event::<polkadot::preimage::events::Requested>()?.unwrap();
            indexer.index_event_preimage_hash(event.hash.into(), block_number, event_index);
            Ok(())
        },
        "Cleared" => {
            let event = event.as_event::<polkadot::preimage::events::Cleared>()?.unwrap();
            indexer.index_event_preimage_hash(event.hash.into(), block_number, event_index);
            Ok(())
        },
        _ => Ok(()),
    }
}
