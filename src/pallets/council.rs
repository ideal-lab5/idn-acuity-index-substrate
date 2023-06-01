use subxt::PolkadotConfig;
use crate::shared::*;
use crate::substrate::*;

pub fn council_index_event(trees: Trees, block_number: u32, event_index: u32, event: subxt::events::EventDetails<PolkadotConfig>) -> Result<(), subxt::Error> {
    match event.variant_name() {
        "Proposed" => {
            let event = event.as_event::<polkadot::council::events::Proposed>()?.unwrap();
            index_event_account_id(trees.clone(), event.account, block_number, event_index);
            index_event_proposal_index(trees.clone(), event.proposal_index, block_number, event_index);
            index_event_proposal_hash(trees, event.proposal_hash.into(), block_number, event_index);
            Ok(())
        },
        "Voted" => {
            let event = event.as_event::<polkadot::council::events::Voted>()?.unwrap();
            index_event_account_id(trees.clone(), event.account, block_number, event_index);
            index_event_proposal_hash(trees, event.proposal_hash.into(), block_number, event_index);
            Ok(())
        },
        "Approved" => {
            let event = event.as_event::<polkadot::council::events::Approved>()?.unwrap();
            index_event_proposal_hash(trees, event.proposal_hash.into(), block_number, event_index);
            Ok(())
        },
        "Disapproved" => {
            let event = event.as_event::<polkadot::council::events::Disapproved>()?.unwrap();
            index_event_proposal_hash(trees, event.proposal_hash.into(), block_number, event_index);
            Ok(())
        },
        "Executed" => {
            let event = event.as_event::<polkadot::council::events::Executed>()?.unwrap();
            index_event_proposal_hash(trees, event.proposal_hash.into(), block_number, event_index);
            Ok(())
        },
        "MemberExecuted" => {
            let event = event.as_event::<polkadot::council::events::MemberExecuted>()?.unwrap();
            index_event_proposal_hash(trees, event.proposal_hash.into(), block_number, event_index);
            Ok(())
        },
        "Closed" => {
            let event = event.as_event::<polkadot::council::events::Closed>()?.unwrap();
            index_event_proposal_hash(trees, event.proposal_hash.into(), block_number, event_index);
            Ok(())
        },
        _ => Ok(()),
    }
}
