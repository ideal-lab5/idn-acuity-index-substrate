use crate::shared::*;
use crate::substrate::*;

pub fn child_bounties_index_event(trees: Trees, block_number: u32, event_index: u32, event: subxt::events::EventDetails) -> Result<(), subxt::Error> {
    match event.variant_name() {
        "Added" => {
            let event = event.as_event::<polkadot::child_bounties::events::Added>()?.unwrap();
            index_event_bounty_index(trees.clone(), event.index, block_number, event_index);
            index_event_bounty_index(trees, event.child_index, block_number, event_index);
            Ok(())
        },
        "Awarded" => {
            let event = event.as_event::<polkadot::child_bounties::events::Awarded>()?.unwrap();
            index_event_bounty_index(trees.clone(), event.index, block_number, event_index);
            index_event_bounty_index(trees.clone(), event.child_index, block_number, event_index);
            index_event_account_id(trees, event.beneficiary, block_number, event_index);
            Ok(())
        },
        "Claimed" => {
            let event = event.as_event::<polkadot::child_bounties::events::Claimed>()?.unwrap();
            index_event_bounty_index(trees.clone(), event.index, block_number, event_index);
            index_event_bounty_index(trees.clone(), event.child_index, block_number, event_index);
            index_event_account_id(trees, event.beneficiary, block_number, event_index);
            Ok(())
        },
        "Canceled" => {
            let event = event.as_event::<polkadot::child_bounties::events::Canceled>()?.unwrap();
            index_event_bounty_index(trees.clone(), event.index, block_number, event_index);
            index_event_bounty_index(trees, event.child_index, block_number, event_index);
            Ok(())
        },
        _ => Ok(()),
    }
}
