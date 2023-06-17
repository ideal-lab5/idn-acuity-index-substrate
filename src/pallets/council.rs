use crate::shared::*;
use crate::substrate::*;
use subxt::PolkadotConfig;

pub fn council_index_event(
    indexer: &Indexer,
    block_number: u32,
    event_index: u32,
    event: subxt::events::EventDetails<PolkadotConfig>,
) -> Result<(), subxt::Error> {
    match event.variant_name() {
        "Proposed" => {
            let event = event
                .as_event::<polkadot::council::events::Proposed>()?
                .unwrap();
            indexer.index_event_account_id(event.account, block_number, event_index);
            indexer.index_event_proposal_index(event.proposal_index, block_number, event_index);
            indexer.index_event_proposal_hash(
                event.proposal_hash.into(),
                block_number,
                event_index,
            );
            Ok(())
        }
        "Voted" => {
            let event = event
                .as_event::<polkadot::council::events::Voted>()?
                .unwrap();
            indexer.index_event_account_id(event.account, block_number, event_index);
            indexer.index_event_proposal_hash(
                event.proposal_hash.into(),
                block_number,
                event_index,
            );
            Ok(())
        }
        "Approved" => {
            let event = event
                .as_event::<polkadot::council::events::Approved>()?
                .unwrap();
            indexer.index_event_proposal_hash(
                event.proposal_hash.into(),
                block_number,
                event_index,
            );
            Ok(())
        }
        "Disapproved" => {
            let event = event
                .as_event::<polkadot::council::events::Disapproved>()?
                .unwrap();
            indexer.index_event_proposal_hash(
                event.proposal_hash.into(),
                block_number,
                event_index,
            );
            Ok(())
        }
        "Executed" => {
            let event = event
                .as_event::<polkadot::council::events::Executed>()?
                .unwrap();
            indexer.index_event_proposal_hash(
                event.proposal_hash.into(),
                block_number,
                event_index,
            );
            Ok(())
        }
        "MemberExecuted" => {
            let event = event
                .as_event::<polkadot::council::events::MemberExecuted>()?
                .unwrap();
            indexer.index_event_proposal_hash(
                event.proposal_hash.into(),
                block_number,
                event_index,
            );
            Ok(())
        }
        "Closed" => {
            let event = event
                .as_event::<polkadot::council::events::Closed>()?
                .unwrap();
            indexer.index_event_proposal_hash(
                event.proposal_hash.into(),
                block_number,
                event_index,
            );
            Ok(())
        }
        _ => Ok(()),
    }
}
