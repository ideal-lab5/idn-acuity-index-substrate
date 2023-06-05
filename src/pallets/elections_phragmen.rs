use subxt::PolkadotConfig;
use crate::shared::*;
use crate::substrate::*;

pub fn elections_phragmen_index_event(indexer: &Indexer, block_number: u32, event_index: u32, event: subxt::events::EventDetails<PolkadotConfig>) -> Result<(), subxt::Error> {
    match event.variant_name() {
        "NewTerm" => {
            let event = event.as_event::<polkadot::phragmen_election::events::NewTerm>()?.unwrap();
            for member in &event.new_members {
                indexer.index_event_account_id(member.0.clone(), block_number, event_index);
            }
            Ok(())
        },
        "MemberKicked" => {
            let event = event.as_event::<polkadot::phragmen_election::events::MemberKicked>()?.unwrap();
            indexer.index_event_account_id(event.member, block_number, event_index);
            Ok(())
        },
        "Renounced" => {
            let event = event.as_event::<polkadot::phragmen_election::events::Renounced>()?.unwrap();
            indexer.index_event_account_id(event.candidate, block_number, event_index);
            Ok(())
        },
        "CandidateSlashed" => {
            let event = event.as_event::<polkadot::phragmen_election::events::CandidateSlashed>()?.unwrap();
            indexer.index_event_account_id(event.candidate, block_number, event_index);
            Ok(())
        },
        "SeatHolderSlashed" => {
            let event = event.as_event::<polkadot::phragmen_election::events::SeatHolderSlashed>()?.unwrap();
            indexer.index_event_account_id(event.seat_holder, block_number, event_index);
            Ok(())
        },
        _ => Ok(()),
    }
}
