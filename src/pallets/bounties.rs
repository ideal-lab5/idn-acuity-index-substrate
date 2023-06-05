use subxt::PolkadotConfig;
use crate::shared::*;
use crate::substrate::*;

pub fn bounties_index_event(indexer: &Indexer, block_number: u32, event_index: u32, event: subxt::events::EventDetails<PolkadotConfig>) -> Result<(), subxt::Error> {
    match event.variant_name() {
        "BountyProposed" => {
            let event = event.as_event::<polkadot::bounties::events::BountyProposed>()?.unwrap();
            indexer.index_event_bounty_index(event.index, block_number, event_index);
            Ok(())
        },
        "BountyRejected" => {
            let event = event.as_event::<polkadot::bounties::events::BountyRejected>()?.unwrap();
            indexer.index_event_bounty_index(event.index, block_number, event_index);
            Ok(())
        },
        "BountyBecameActive" => {
            let event = event.as_event::<polkadot::bounties::events::BountyBecameActive>()?.unwrap();
            indexer.index_event_bounty_index(event.index, block_number, event_index);
            Ok(())
        },
        "BountyAwarded" => {
            let event = event.as_event::<polkadot::bounties::events::BountyAwarded>()?.unwrap();
            indexer.index_event_bounty_index(event.index, block_number, event_index);
            indexer.index_event_account_id(event.beneficiary, block_number, event_index);
            Ok(())
        },
        "BountyClaimed" => {
            let event = event.as_event::<polkadot::bounties::events::BountyClaimed>()?.unwrap();
            indexer.index_event_bounty_index(event.index, block_number, event_index);
            indexer.index_event_account_id(event.beneficiary, block_number, event_index);
            Ok(())
        },
        "BountyCanceled" => {
            let event = event.as_event::<polkadot::bounties::events::BountyCanceled>()?.unwrap();
            indexer.index_event_bounty_index(event.index, block_number, event_index);
            Ok(())
        },
        "BountyExtended" => {
            let event = event.as_event::<polkadot::bounties::events::BountyExtended>()?.unwrap();
            indexer.index_event_bounty_index(event.index, block_number, event_index);
            Ok(())
        },
        _ => Ok(()),
    }
}
