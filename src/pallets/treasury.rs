use crate::shared::*;
use crate::substrate::*;

pub fn treasury_index_event(trees: Trees, block_number: u32, event_index: u32, event: subxt::events::EventDetails) -> Result<(), subxt::Error> {
    match event.variant_name() {
        "Proposed" => {
            let event = event.as_event::<polkadot::treasury::events::Proposed>().unwrap().unwrap();
            index_event_proposal_index(trees.clone(), event.proposal_index, block_number, event_index);
            Ok(())
        },
        "Awarded" => {
            let event = event.as_event::<polkadot::treasury::events::Awarded>().unwrap().unwrap();
            index_event_proposal_index(trees.clone(), event.proposal_index, block_number, event_index);
            index_event_account_id(trees.clone(), event.account, block_number, event_index);
            Ok(())
        },
        "Rejected" => {
            let event = event.as_event::<polkadot::treasury::events::Rejected>().unwrap().unwrap();
            index_event_proposal_index(trees.clone(), event.proposal_index, block_number, event_index);
            Ok(())
        },
        "SpendApproved" => {
            let event = event.as_event::<polkadot::treasury::events::SpendApproved>().unwrap().unwrap();
            index_event_proposal_index(trees.clone(), event.proposal_index, block_number, event_index);
            index_event_account_id(trees.clone(), event.beneficiary, block_number, event_index);
            Ok(())
        },
        _ => Ok(()),
    }
}
