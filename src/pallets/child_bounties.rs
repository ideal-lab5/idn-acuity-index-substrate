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
pub enum ChildBounties {
    #[serde(rename_all = "camelCase")]
	Added {
	    index: u32,
	    child_index: u32,
	},
    #[serde(rename_all = "camelCase")]
	Awarded {
	    index: u32,
	    child_index: u32,
	    beneficiary: AccountId32,
	},
    #[serde(rename_all = "camelCase")]
	Claimed {
		index: u32,
		child_index: u32,
		payout: u128,
		beneficiary: AccountId32,
	},
    #[serde(rename_all = "camelCase")]
	Canceled {
	    index: u32,
	    child_index: u32,
	},
}

pub fn child_bounties_index_event(trees: Trees, block_number: u32, event_index: u32, event: subxt::events::EventDetails) {
    match event.variant_name() {
        "Added" => {
            let event = event.as_event::<polkadot::child_bounties::events::Added>().unwrap().unwrap();
            let event_db = Event::ChildBounties(
                ChildBounties::Added {
	                index: event.index,
	                child_index: event.child_index,
                }
            );
            let value = Event::encode(&event_db);
            index_event_bounty_index(trees.clone(), event.index, block_number, event_index, &value);
            index_event_bounty_index(trees.clone(), event.child_index, block_number, event_index, &value);
        },
        "Awarded" => {
            let event = event.as_event::<polkadot::child_bounties::events::Awarded>().unwrap().unwrap();
            let event_db = Event::ChildBounties(
                ChildBounties::Awarded {
	                index: event.index,
	                child_index: event.child_index,
	                beneficiary: event.beneficiary.clone(),
                }
            );
            let value = Event::encode(&event_db);
            index_event_bounty_index(trees.clone(), event.index, block_number, event_index, &value);
            index_event_bounty_index(trees.clone(), event.child_index, block_number, event_index, &value);
            index_event_account_id(trees.clone(), event.beneficiary, block_number, event_index, &value);
        },
        "Claimed" => {
            let event = event.as_event::<polkadot::child_bounties::events::Claimed>().unwrap().unwrap();
            let event_db = Event::ChildBounties(
                ChildBounties::Claimed {
	                index: event.index,
	                child_index: event.child_index,
	                payout: event.payout,
	                beneficiary: event.beneficiary.clone(),
                }
            );
            let value = Event::encode(&event_db);
            index_event_bounty_index(trees.clone(), event.index, block_number, event_index, &value);
            index_event_bounty_index(trees.clone(), event.child_index, block_number, event_index, &value);
            index_event_account_id(trees.clone(), event.beneficiary, block_number, event_index, &value);
        },
        "Canceled" => {
            let event = event.as_event::<polkadot::child_bounties::events::Canceled>().unwrap().unwrap();
            let event_db = Event::ChildBounties(
                ChildBounties::Canceled {
	                index: event.index,
	                child_index: event.child_index,
                }
            );
            let value = Event::encode(&event_db);
            index_event_bounty_index(trees.clone(), event.index, block_number, event_index, &value);
            index_event_bounty_index(trees.clone(), event.child_index, block_number, event_index, &value);
        },
        _ => {},
    }
}
