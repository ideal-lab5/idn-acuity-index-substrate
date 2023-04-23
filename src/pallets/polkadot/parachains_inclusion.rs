use crate::shared::*;
use crate::substrate::*;

pub fn parachains_inclusion_index_event(trees: Trees, block_number: u32, event_index: u32, event: subxt::events::EventDetails) -> Result<(), subxt::Error> {
    match event.variant_name() {
        "CandidateBacked" => {
            let event = event.as_event::<polkadot::parachains_inclusion::events::CandidateBacked>()?.unwrap();
            index_event_account_id(trees, event.who, block_number, event_index);
            Ok(())
        },
        _ => Ok(()),
    }
}

