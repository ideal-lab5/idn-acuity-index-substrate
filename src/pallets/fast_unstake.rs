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
pub enum FastUnstake {
    #[serde(rename_all = "camelCase")]
	Unstaked {
	    stash: AccountId32,
//	    result: DispatchResult,
	},
    #[serde(rename_all = "camelCase")]
	Slashed {
	    stash: AccountId32,
	    amount: u128,
	},
}

pub fn fast_unstake_index_event(trees: Trees, block_number: u32, event_index: u32, event: subxt::events::EventDetails) -> Result<(), subxt::Error> {
    match event.variant_name() {
        "Unstaked" => {
            let event = event.as_event::<polkadot::fast_unstake::events::Unstaked>()?.unwrap();
            let event_db = Event::FastUnstake(
                FastUnstake::Unstaked {
	                stash: event.stash.clone(),
                }
            );
            let value = Event::encode(&event_db);
            index_event_account_id(trees.clone(), event.stash, block_number, event_index, &value);
            Ok(())
        },
        "Slashed" => {
            let event = event.as_event::<polkadot::fast_unstake::events::Slashed>()?.unwrap();
            let event_db = Event::FastUnstake(
                FastUnstake::Slashed {
	                stash: event.stash.clone(),
	                amount: event.amount,
                }
            );
            let value = Event::encode(&event_db);
            index_event_account_id(trees.clone(), event.stash, block_number, event_index, &value);
            Ok(())
        },
        _ => Ok(()),
    }
}
