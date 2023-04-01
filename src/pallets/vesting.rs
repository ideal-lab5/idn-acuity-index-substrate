use subxt::{
    utils::AccountId32,
};

use parity_scale_codec::{Encode, Decode};
use serde::{Serialize, Deserialize};

use crate::shared::*;
use crate::substrate::*;

#[derive(Encode, Decode, Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "variant")]
pub enum Vesting {
    #[serde(rename_all = "camelCase")]
    VestingUpdated {
        account: AccountId32,
        unvested: u128,
    },
    #[serde(rename_all = "camelCase")]
	VestingCompleted {
	    account: AccountId32,
	},
}

pub fn vesting_index_event(trees: Trees, block_number: u32, event_index: u32, event: subxt::events::EventDetails) {
    match event.variant_name() {
        "VestingUpdated" => {
            let event = event.as_event::<polkadot::vesting::events::VestingUpdated>().unwrap().unwrap();
            let event_db = Event::Vesting(
                Vesting::VestingUpdated {
                    account: event.account.clone(),
                    unvested: event.unvested,
                }
            );
            let value = Event::encode(&event_db);
            index_event_account_id(trees.clone(), event.account, block_number, event_index, &value);
        },
        "VestingCompleted" => {
            let event = event.as_event::<polkadot::vesting::events::VestingCompleted>().unwrap().unwrap();
            let event_db = Event::Vesting(
                Vesting::VestingCompleted {
                    account: event.account.clone(),
                }
            );
            let value = Event::encode(&event_db);
            index_event_account_id(trees.clone(), event.account, block_number, event_index, &value);
        },
        _ => {},
    }
}
