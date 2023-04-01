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
pub enum Tips {
	NewTip {
	    tip_hash: [u8; 32],
	},
    #[serde(rename_all = "camelCase")]
	TipClosing {
	    tip_hash: [u8; 32],
	},
    #[serde(rename_all = "camelCase")]
	TipClosed {
	    tip_hash: [u8; 32],
	    who: AccountId32,
	    payout: u128,
	},
    #[serde(rename_all = "camelCase")]
	TipRetracted {
	    tip_hash: [u8; 32],
	},
    #[serde(rename_all = "camelCase")]
	TipSlashed {
	    tip_hash: [u8; 32],
	    finder: AccountId32,
	    deposit: u128,
	},
}

pub fn tips_index_event(trees: Trees, block_number: u32, event_index: u32, event: subxt::events::EventDetails) {
    match event.variant_name() {
        "NewTip" => {
            let event = event.as_event::<polkadot::tips::events::NewTip>().unwrap().unwrap();
            let event_db = Event::Tips(
                Tips::NewTip {
            	    tip_hash: event.tip_hash.into(),
                }
            );
            let value = Event::encode(&event_db);
            index_event_tip_hash(trees.clone(), event.tip_hash.into(), block_number, event_index, &value);
        },
        "TipClosing" => {
            let event = event.as_event::<polkadot::tips::events::TipClosing>().unwrap().unwrap();
            let event_db = Event::Tips(
                Tips::TipClosing {
            	    tip_hash: event.tip_hash.into(),
                }
            );
            let value = Event::encode(&event_db);
            index_event_tip_hash(trees.clone(), event.tip_hash.into(), block_number, event_index, &value);
        },
        "TipClosed" => {
            let event = event.as_event::<polkadot::tips::events::TipClosed>().unwrap().unwrap();
            let event_db = Event::Tips(
                Tips::TipClosed {
            	    tip_hash: event.tip_hash.into(),
	                who: event.who.clone(),
	                payout: event.payout,
                }
            );
            let value = Event::encode(&event_db);
            index_event_tip_hash(trees.clone(), event.tip_hash.into(), block_number, event_index, &value);
            index_event_account_id(trees.clone(), event.who, block_number, event_index, &value);
        },
        "TipRetracted" => {
            let event = event.as_event::<polkadot::tips::events::TipRetracted>().unwrap().unwrap();
            let event_db = Event::Tips(
                Tips::TipRetracted {
            	    tip_hash: event.tip_hash.into(),
                }
            );
            let value = Event::encode(&event_db);
            index_event_tip_hash(trees.clone(), event.tip_hash.into(), block_number, event_index, &value);
        },
        "TipSlashed" => {
            let event = event.as_event::<polkadot::tips::events::TipSlashed>().unwrap().unwrap();
            let event_db = Event::Tips(
                Tips::TipSlashed {
            	    tip_hash: event.tip_hash.into(),
	                finder: event.finder.clone(),
	                deposit: event.deposit,
                }
            );
            let value = Event::encode(&event_db);
            index_event_tip_hash(trees.clone(), event.tip_hash.into(), block_number, event_index, &value);
            index_event_account_id(trees.clone(), event.finder, block_number, event_index, &value);
        },
        _ => {},
    }
}
