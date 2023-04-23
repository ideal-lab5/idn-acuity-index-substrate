use crate::shared::*;
use crate::substrate::*;

pub fn democracy_index_event(trees: Trees, block_number: u32, event_index: u32, event: subxt::events::EventDetails) -> Result<(), subxt::Error> {
    match event.variant_name() {
        "Proposed" => {
            let event = event.as_event::<polkadot::democracy::events::Proposed>()?.unwrap();
            index_event_proposal_index(trees, event.proposal_index, block_number, event_index);
            Ok(())
        },
        "Tabled" => {
            let event = event.as_event::<polkadot::democracy::events::Tabled>()?.unwrap();
            index_event_proposal_index(trees, event.proposal_index, block_number, event_index);
            Ok(())
        },
        "Started" => {
            let event = event.as_event::<polkadot::democracy::events::Started>()?.unwrap();
            index_event_ref_index(trees, event.ref_index, block_number, event_index);
            Ok(())
        },
        "Passed" => {
            let event = event.as_event::<polkadot::democracy::events::Passed>()?.unwrap();
            index_event_ref_index(trees, event.ref_index, block_number, event_index);
            Ok(())
        },
        "NotPassed" => {
            let event = event.as_event::<polkadot::democracy::events::NotPassed>()?.unwrap();
            index_event_ref_index(trees, event.ref_index, block_number, event_index);
            Ok(())
        },
        "Cancelled" => {
            let event = event.as_event::<polkadot::democracy::events::Cancelled>()?.unwrap();
            index_event_ref_index(trees, event.ref_index, block_number, event_index);
            Ok(())
        },
        "Delegated" => {
            let event = event.as_event::<polkadot::democracy::events::Delegated>()?.unwrap();
            index_event_account_id(trees.clone(), event.who, block_number, event_index);
            index_event_account_id(trees, event.target, block_number, event_index);
            Ok(())
        },
        "Undelegated" => {
            let event = event.as_event::<polkadot::democracy::events::Undelegated>()?.unwrap();
            index_event_account_id(trees, event.account, block_number, event_index);
            Ok(())
        },
        "Vetoed" => {
            let event = event.as_event::<polkadot::democracy::events::Vetoed>()?.unwrap();
            index_event_account_id(trees, event.who, block_number, event_index);
            Ok(())
        },
        "Voted" => {
            let event = event.as_event::<polkadot::democracy::events::Voted>()?.unwrap();
            index_event_account_id(trees.clone(), event.voter, block_number, event_index);
            index_event_ref_index(trees, event.ref_index, block_number, event_index);
            Ok(())
        },
        "Seconded" => {
            let event = event.as_event::<polkadot::democracy::events::Seconded>()?.unwrap();
            index_event_account_id(trees.clone(), event.seconder, block_number, event_index);
            index_event_proposal_index(trees, event.prop_index, block_number, event_index);
            Ok(())
        },
        "ProposalCanceled" => {
            let event = event.as_event::<polkadot::democracy::events::ProposalCanceled>()?.unwrap();
            index_event_proposal_index(trees, event.prop_index, block_number, event_index);
            Ok(())
        },
        _ => Ok(()),
    }
}
