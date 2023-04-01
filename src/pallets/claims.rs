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
pub enum Claims {
    #[serde(rename_all = "camelCase")]
	Claimed {
	    who: AccountId32,
	    ethereum_address: [u8; 20],
	    amount: u128,
	},
}

pub fn claims_index_event(trees: Trees, block_number: u32, event_index: u32, event: subxt::events::EventDetails) {
    match event.variant_name() {
        "Claimed" => {
            let event = event.as_event::<polkadot::claims::events::Claimed>().unwrap().unwrap();
            let event_db = Event::Claims(
                Claims::Claimed {
	                who: event.who.clone(),
	                ethereum_address: event.ethereum_address.0,
	                amount: event.amount,
                }
            );
            let value = Event::encode(&event_db);
            index_event_account_id(trees.clone(), event.who, block_number, event_index, &value);
        },
        _ => {},
    }
}
