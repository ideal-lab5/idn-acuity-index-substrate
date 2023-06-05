use subxt::PolkadotConfig;
use crate::shared::*;
use crate::substrate::*;

pub fn treasury_index_event(indexer: &Indexer, block_number: u32, event_index: u32, event: subxt::events::EventDetails<PolkadotConfig>) -> Result<(), subxt::Error> {
    match event.variant_name() {
        "Proposed" => {
            let event = event.as_event::<polkadot::treasury::events::Proposed>().unwrap().unwrap();
            indexer.index_event_proposal_index(event.proposal_index, block_number, event_index);
            Ok(())
        },
        "Awarded" => {
            let event = event.as_event::<polkadot::treasury::events::Awarded>().unwrap().unwrap();
            indexer.index_event_proposal_index(event.proposal_index, block_number, event_index);
            indexer.index_event_account_id(event.account, block_number, event_index);
            Ok(())
        },
        "Rejected" => {
            let event = event.as_event::<polkadot::treasury::events::Rejected>().unwrap().unwrap();
            indexer.index_event_proposal_index(event.proposal_index, block_number, event_index);
            Ok(())
        },
        "SpendApproved" => {
            let event = event.as_event::<polkadot::treasury::events::SpendApproved>().unwrap().unwrap();
            indexer.index_event_proposal_index(event.proposal_index, block_number, event_index);
            indexer.index_event_account_id(event.beneficiary, block_number, event_index);
            Ok(())
        },
        _ => Ok(()),
    }
}
