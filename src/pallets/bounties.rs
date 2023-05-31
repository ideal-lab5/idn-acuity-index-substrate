use subxt::PolkadotConfig;
use crate::shared::*;
use crate::substrate::*;

pub fn bounties_index_event(trees: Trees, block_number: u32, event_index: u32, event: subxt::events::EventDetails<PolkadotConfig>) -> Result<(), subxt::Error> {
    match event.variant_name() {
        "BountyProposed" => {
            let event = event.as_event::<polkadot::bounties::events::BountyProposed>()?.unwrap();
            index_event_bounty_index(trees, event.index, block_number, event_index);
            Ok(())
        },
        "BountyRejected" => {
            let event = event.as_event::<polkadot::bounties::events::BountyRejected>()?.unwrap();
            index_event_bounty_index(trees, event.index, block_number, event_index);
            Ok(())
        },
        "BountyBecameActive" => {
            let event = event.as_event::<polkadot::bounties::events::BountyBecameActive>()?.unwrap();
            index_event_bounty_index(trees, event.index, block_number, event_index);
            Ok(())
        },
        "BountyAwarded" => {
            let event = event.as_event::<polkadot::bounties::events::BountyAwarded>()?.unwrap();
            index_event_bounty_index(trees.clone(), event.index, block_number, event_index);
            index_event_account_id(trees, event.beneficiary, block_number, event_index);
            Ok(())
        },
        "BountyClaimed" => {
            let event = event.as_event::<polkadot::bounties::events::BountyClaimed>()?.unwrap();
            index_event_bounty_index(trees.clone(), event.index, block_number, event_index);
            index_event_account_id(trees, event.beneficiary, block_number, event_index);
            Ok(())
        },
        "BountyCanceled" => {
            let event = event.as_event::<polkadot::bounties::events::BountyCanceled>()?.unwrap();
            index_event_bounty_index(trees, event.index, block_number, event_index);
            Ok(())
        },
        "BountyExtended" => {
            let event = event.as_event::<polkadot::bounties::events::BountyExtended>()?.unwrap();
            index_event_bounty_index(trees, event.index, block_number, event_index);
            Ok(())
        },
        _ => Ok(()),
    }
}
