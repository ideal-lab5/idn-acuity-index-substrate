use crate::shared::*;
use crate::substrate::*;

pub fn preimage_index_event(trees: Trees, block_number: u32, event_index: u32, event: subxt::events::EventDetails) -> Result<(), subxt::Error> {
    match event.variant_name() {
        "Noted" => {
            let event = event.as_event::<polkadot::preimage::events::Noted>()?.unwrap();
            index_event_preimage_hash(trees, event.hash.into(), block_number, event_index);
            Ok(())
        },
        "Requested" => {
            let event = event.as_event::<polkadot::preimage::events::Requested>()?.unwrap();
            index_event_preimage_hash(trees, event.hash.into(), block_number, event_index);
            Ok(())
        },
        "Cleared" => {
            let event = event.as_event::<polkadot::preimage::events::Cleared>()?.unwrap();
            index_event_preimage_hash(trees, event.hash.into(), block_number, event_index);
            Ok(())
        },
        _ => Ok(()),
    }
}
