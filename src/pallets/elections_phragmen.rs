use crate::shared::*;
use crate::substrate::*;

pub fn elections_phragmen_index_event(trees: Trees, block_number: u32, event_index: u32, event: subxt::events::EventDetails) -> Result<(), subxt::Error> {
    match event.variant_name() {
        "NewTerm" => {
            let event = event.as_event::<polkadot::phragmen_election::events::NewTerm>()?.unwrap();
            for member in &event.new_members {
                index_event_account_id(trees.clone(), member.0.clone(), block_number, event_index);
            }
            Ok(())
        },
        "MemberKicked" => {
            let event = event.as_event::<polkadot::phragmen_election::events::MemberKicked>()?.unwrap();
            index_event_account_id(trees.clone(), event.member, block_number, event_index);
            Ok(())
        },
        "Renounced" => {
            let event = event.as_event::<polkadot::phragmen_election::events::Renounced>()?.unwrap();
            index_event_account_id(trees.clone(), event.candidate, block_number, event_index);
            Ok(())
        },
        "CandidateSlashed" => {
            let event = event.as_event::<polkadot::phragmen_election::events::CandidateSlashed>()?.unwrap();
            index_event_account_id(trees.clone(), event.candidate, block_number, event_index);
            Ok(())
        },
        "SeatHolderSlashed" => {
            let event = event.as_event::<polkadot::phragmen_election::events::SeatHolderSlashed>()?.unwrap();
            index_event_account_id(trees.clone(), event.seat_holder, block_number, event_index);
            Ok(())
        },
        _ => Ok(()),
    }
}
