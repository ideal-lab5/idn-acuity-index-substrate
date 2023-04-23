use crate::shared::*;
use crate::substrate::*;

pub fn bags_list_index_event(trees: Trees, block_number: u32, event_index: u32, event: subxt::events::EventDetails) -> Result<(), subxt::Error> {
    match event.variant_name() {
        "Rebagged" => {
            let event = event.as_event::<polkadot::voter_list::events::Rebagged>()?.unwrap();
            index_event_account_id(trees, event.who, block_number, event_index);
            Ok(())
        },
        "ScoreUpdated" => {
            let event = event.as_event::<polkadot::voter_list::events::ScoreUpdated>()?.unwrap();
            index_event_account_id(trees, event.who, block_number, event_index);
            Ok(())
        },
        _ => Ok(()),
    }
}
