use crate::shared::*;
use crate::substrate::*;
use subxt::PolkadotConfig;

pub fn indices_index_event(
    indexer: &Indexer,
    block_number: u32,
    event_index: u32,
    event: subxt::events::EventDetails<PolkadotConfig>,
) -> Result<(), subxt::Error> {
    match event.variant_name() {
        "IndexAssigned" => {
            let event = event
                .as_event::<polkadot::indices::events::IndexAssigned>()?
                .unwrap();
            indexer.index_event_account_id(event.who, block_number, event_index);
            indexer.index_event_account_index(event.index, block_number, event_index);
            Ok(())
        }
        "IndexFreed" => {
            let event = event
                .as_event::<polkadot::indices::events::IndexFreed>()?
                .unwrap();
            indexer.index_event_account_index(event.index, block_number, event_index);
            Ok(())
        }
        "IndexFrozen" => {
            let event = event
                .as_event::<polkadot::indices::events::IndexFrozen>()?
                .unwrap();
            indexer.index_event_account_index(event.index, block_number, event_index);
            indexer.index_event_account_id(event.who, block_number, event_index);
            Ok(())
        }
        _ => Ok(()),
    }
}
