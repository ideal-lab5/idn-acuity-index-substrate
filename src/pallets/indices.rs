use subxt::PolkadotConfig;
use crate::shared::*;
use crate::substrate::*;

pub fn indices_index_event(trees: Trees, block_number: u32, event_index: u32, event: subxt::events::EventDetails<PolkadotConfig>) -> Result<(), subxt::Error> {
    match event.variant_name() {
        "IndexAssigned" => {
            let event = event.as_event::<polkadot::indices::events::IndexAssigned>()?.unwrap();
            index_event_account_id(trees.clone(), event.who, block_number, event_index);
            index_event_account_index(trees, event.index, block_number, event_index);
            Ok(())
        },
        "IndexFreed" => {
            let event = event.as_event::<polkadot::indices::events::IndexFreed>()?.unwrap();
            index_event_account_index(trees, event.index, block_number, event_index);
            Ok(())
        },
        "IndexFrozen" => {
            let event = event.as_event::<polkadot::indices::events::IndexFrozen>()?.unwrap();
            index_event_account_index(trees.clone(), event.index, block_number, event_index);
            index_event_account_id(trees, event.who, block_number, event_index);
            Ok(())
        },
        _ => Ok(()),
    }
}
