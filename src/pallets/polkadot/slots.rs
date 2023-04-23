use crate::shared::*;
use crate::substrate::*;

pub fn slots_index_event(trees: Trees, block_number: u32, event_index: u32, event: subxt::events::EventDetails) -> Result<(), subxt::Error> {
    match event.variant_name() {
        "Leased" => {
            let event = event.as_event::<polkadot::slots::events::Leased>()?.unwrap();
            index_event_para_id(trees.clone(), event.para_id.0, block_number, event_index);
            index_event_account_id(trees, event.leaser, block_number, event_index);
            Ok(())
        },
        _ => Ok(()),
    }
}
