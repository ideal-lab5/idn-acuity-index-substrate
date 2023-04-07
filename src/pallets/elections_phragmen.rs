use subxt::{
    utils::AccountId32,
};

use parity_scale_codec::{Encode, Decode};
use serde::{Serialize, Deserialize};

use crate::shared::*;
use crate::substrate::*;

#[derive(Encode, Decode, Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "variant", content = "details")]
pub enum PhragmenElection {
    #[serde(rename_all = "camelCase")]
	NewTerm {
	    new_members: Vec<(AccountId32, u128)>,
	},
    #[serde(rename_all = "camelCase")]
	MemberKicked {
	    member: AccountId32,
	},
    #[serde(rename_all = "camelCase")]
	Renounced {
	    candidate: AccountId32,
	},
    #[serde(rename_all = "camelCase")]
	CandidateSlashed {
	    candidate: AccountId32,
	    amount: u128,
	},
    #[serde(rename_all = "camelCase")]
	SeatHolderSlashed {
		seat_holder: AccountId32,
		amount: u128,
	},
}

pub fn elections_phragmen_index_event(trees: Trees, block_number: u32, event_index: u32, event: subxt::events::EventDetails) {
    match event.variant_name() {
       "NewTerm" => {
           let event = event.as_event::<polkadot::phragmen_election::events::NewTerm>().unwrap().unwrap();
           let event_db = Event::PhragmenElection(
               PhragmenElection::NewTerm {
           	       new_members: event.new_members.clone(),
               }
           );
           let value = Event::encode(&event_db);
           for member in &event.new_members {
               index_event_account_id(trees.clone(), member.0.clone(), block_number, event_index, &value);
           }
       },
       "MemberKicked" => {
           let event = event.as_event::<polkadot::phragmen_election::events::MemberKicked>().unwrap().unwrap();
           let event_db = Event::PhragmenElection(
               PhragmenElection::MemberKicked {
            	    member: event.member.clone(),
               }
           );
           let value = Event::encode(&event_db);
           index_event_account_id(trees.clone(), event.member, block_number, event_index, &value);
       },
       "Renounced" => {
           let event = event.as_event::<polkadot::phragmen_election::events::Renounced>().unwrap().unwrap();
           let event_db = Event::PhragmenElection(
               PhragmenElection::Renounced {
            	    candidate: event.candidate.clone(),
               }
           );
           let value = Event::encode(&event_db);
           index_event_account_id(trees.clone(), event.candidate, block_number, event_index, &value);
       },
       "CandidateSlashed" => {
           let event = event.as_event::<polkadot::phragmen_election::events::CandidateSlashed>().unwrap().unwrap();
           let event_db = Event::PhragmenElection(
               PhragmenElection::CandidateSlashed {
            	    candidate: event.candidate.clone(),
       	    	    amount: event.amount,
               }
           );
           let value = Event::encode(&event_db);
           index_event_account_id(trees.clone(), event.candidate, block_number, event_index, &value);
       },
       "SeatHolderSlashed" => {
           let event = event.as_event::<polkadot::phragmen_election::events::SeatHolderSlashed>().unwrap().unwrap();
           let event_db = Event::PhragmenElection(
               PhragmenElection::SeatHolderSlashed {
            	    seat_holder: event.seat_holder.clone(),
       	    	    amount: event.amount,
               }
           );
           let value = Event::encode(&event_db);
           index_event_account_id(trees.clone(), event.seat_holder, block_number, event_index, &value);
       },
       _ => {},
    }
}
