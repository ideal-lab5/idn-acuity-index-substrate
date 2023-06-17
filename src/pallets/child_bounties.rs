use crate::shared::*;
use crate::substrate::*;
use subxt::PolkadotConfig;

pub fn child_bounties_index_event(
    indexer: &Indexer,
    block_number: u32,
    event_index: u32,
    event: subxt::events::EventDetails<PolkadotConfig>,
) -> Result<(), subxt::Error> {
    match event.variant_name() {
        "Added" => {
            let event = event
                .as_event::<polkadot::child_bounties::events::Added>()?
                .unwrap();
            indexer.index_event_bounty_index(event.index, block_number, event_index);
            indexer.index_event_bounty_index(event.child_index, block_number, event_index);
            Ok(())
        }
        "Awarded" => {
            let event = event
                .as_event::<polkadot::child_bounties::events::Awarded>()?
                .unwrap();
            indexer.index_event_bounty_index(event.index, block_number, event_index);
            indexer.index_event_bounty_index(event.child_index, block_number, event_index);
            indexer.index_event_account_id(event.beneficiary, block_number, event_index);
            Ok(())
        }
        "Claimed" => {
            let event = event
                .as_event::<polkadot::child_bounties::events::Claimed>()?
                .unwrap();
            indexer.index_event_bounty_index(event.index, block_number, event_index);
            indexer.index_event_bounty_index(event.child_index, block_number, event_index);
            indexer.index_event_account_id(event.beneficiary, block_number, event_index);
            Ok(())
        }
        "Canceled" => {
            let event = event
                .as_event::<polkadot::child_bounties::events::Canceled>()?
                .unwrap();
            indexer.index_event_bounty_index(event.index, block_number, event_index);
            indexer.index_event_bounty_index(event.child_index, block_number, event_index);
            Ok(())
        }
        _ => Ok(()),
    }
}
