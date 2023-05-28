use crate::shared::*;
use crate::substrate::*;

pub fn fast_unstake_index_event(trees: Trees, block_number: u32, event_index: u32, event: subxt::events::EventDetails) -> Result<(), subxt::Error> {
    match event.variant_name() {
        "Unstaked" => {
            let event = event.as_event::<polkadot::fast_unstake::events::Unstaked>()?.unwrap();
            index_event_account_id(trees, event.stash, block_number, event_index);
            Ok(())
        },
        "Slashed" => {
            let event = event.as_event::<polkadot::fast_unstake::events::Slashed>()?.unwrap();
            index_event_account_id(trees, event.stash, block_number, event_index);
            Ok(())
        },
        "BatchChecked" => {
            let event = event.as_event::<polkadot::fast_unstake::events::BatchChecked>()?.unwrap();
            for era in event.eras {
                index_event_era_index(trees.clone(), era, block_number, event_index);
            }
            Ok(())
        },
        _ => Ok(()),
    }
}
