use crate::shared::*;
use crate::substrate::*;
use subxt::PolkadotConfig;

pub fn democracy_index_event(
    indexer: &Indexer,
    block_number: u32,
    event_index: u32,
    event: subxt::events::EventDetails<PolkadotConfig>,
) -> Result<(), subxt::Error> {
    match event.variant_name() {
        "Proposed" => {
            let event = event
                .as_event::<polkadot::democracy::events::Proposed>()?
                .unwrap();
            indexer.index_event_proposal_index(event.proposal_index, block_number, event_index);
            Ok(())
        }
        "Tabled" => {
            let event = event
                .as_event::<polkadot::democracy::events::Tabled>()?
                .unwrap();
            indexer.index_event_proposal_index(event.proposal_index, block_number, event_index);
            Ok(())
        }
        "Started" => {
            let event = event
                .as_event::<polkadot::democracy::events::Started>()?
                .unwrap();
            indexer.index_event_ref_index(event.ref_index, block_number, event_index);
            Ok(())
        }
        "Passed" => {
            let event = event
                .as_event::<polkadot::democracy::events::Passed>()?
                .unwrap();
            indexer.index_event_ref_index(event.ref_index, block_number, event_index);
            Ok(())
        }
        "NotPassed" => {
            let event = event
                .as_event::<polkadot::democracy::events::NotPassed>()?
                .unwrap();
            indexer.index_event_ref_index(event.ref_index, block_number, event_index);
            Ok(())
        }
        "Cancelled" => {
            let event = event
                .as_event::<polkadot::democracy::events::Cancelled>()?
                .unwrap();
            indexer.index_event_ref_index(event.ref_index, block_number, event_index);
            Ok(())
        }
        "Delegated" => {
            let event = event
                .as_event::<polkadot::democracy::events::Delegated>()?
                .unwrap();
            indexer.index_event_account_id(event.who, block_number, event_index);
            indexer.index_event_account_id(event.target, block_number, event_index);
            Ok(())
        }
        "Undelegated" => {
            let event = event
                .as_event::<polkadot::democracy::events::Undelegated>()?
                .unwrap();
            indexer.index_event_account_id(event.account, block_number, event_index);
            Ok(())
        }
        "Vetoed" => {
            let event = event
                .as_event::<polkadot::democracy::events::Vetoed>()?
                .unwrap();
            indexer.index_event_account_id(event.who, block_number, event_index);
            indexer.index_event_proposal_hash(
                event.proposal_hash.into(),
                block_number,
                event_index,
            );
            Ok(())
        }
        "Blacklisted" => {
            let event = event
                .as_event::<polkadot::democracy::events::Blacklisted>()?
                .unwrap();
            indexer.index_event_proposal_hash(
                event.proposal_hash.into(),
                block_number,
                event_index,
            );
            Ok(())
        }
        "Voted" => {
            let event = event
                .as_event::<polkadot::democracy::events::Voted>()?
                .unwrap();
            indexer.index_event_account_id(event.voter, block_number, event_index);
            indexer.index_event_ref_index(event.ref_index, block_number, event_index);
            Ok(())
        }
        "Seconded" => {
            let event = event
                .as_event::<polkadot::democracy::events::Seconded>()?
                .unwrap();
            indexer.index_event_account_id(event.seconder, block_number, event_index);
            indexer.index_event_proposal_index(event.prop_index, block_number, event_index);
            Ok(())
        }
        "ProposalCanceled" => {
            let event = event
                .as_event::<polkadot::democracy::events::ProposalCanceled>()?
                .unwrap();
            indexer.index_event_proposal_index(event.prop_index, block_number, event_index);
            Ok(())
        }
        _ => Ok(()),
    }
}
