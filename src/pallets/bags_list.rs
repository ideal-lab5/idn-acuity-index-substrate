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
pub enum BagsList {
    #[serde(rename_all = "camelCase")]
	Rebagged {
	    who: AccountId32,
	    from: u64,
	    to: u64,
	},
    #[serde(rename_all = "camelCase")]
	ScoreUpdated {
	    who: AccountId32,
	    new_score: u64,
	},
}

pub fn bags_list_index_event(trees: Trees, block_number: u32, event_index: u32, event: subxt::events::EventDetails) -> Result<(), subxt::Error> {
    match event.variant_name() {
        "Rebagged" => {
            let event = event.as_event::<polkadot::voter_list::events::Rebagged>()?.unwrap();
            let event_db = Event::BagsList(
                BagsList::Rebagged {
	                who: event.who.clone(),
	                from: event.from,
	                to: event.to,
                }
            );
            let value = Event::encode(&event_db);
            index_event_account_id(trees.clone(), event.who, block_number, event_index, &value);
            Ok(())
        },
        "ScoreUpdated" => {
            let event = event.as_event::<polkadot::voter_list::events::ScoreUpdated>()?.unwrap();
            let event_db = Event::BagsList(
                BagsList::ScoreUpdated {
	                who: event.who.clone(),
	                new_score: event.new_score,
                }
            );
            let value = Event::encode(&event_db);
            index_event_account_id(trees.clone(), event.who, block_number, event_index, &value);
            Ok(())
        },
        _ => Ok(()),
    }
}
