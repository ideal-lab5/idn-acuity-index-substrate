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
pub enum Slots {
    #[serde(rename_all = "camelCase")]
	Leased {
		para_id: ParaId,
		leaser: AccountId32,
		period_begin: u32,
		period_count: u32,
		extra_reserved: u128,
		total_amount: u128,
	},
}

pub fn slots_index_event(trees: Trees, block_number: u32, event_index: u32, event: subxt::events::EventDetails) {
    match event.variant_name() {
        "Leased" => {
            let event = event.as_event::<polkadot::slots::events::Leased>().unwrap().unwrap();
            let event_db = Event::Slots(
                Slots::Leased {
		            para_id: ParaId(event.para_id.0),
		            leaser: event.leaser.clone(),
		            period_begin: event.period_begin,
		            period_count: event.period_count,
		            extra_reserved: event.extra_reserved,
		            total_amount: event.total_amount,
                }
            );
            let value = Event::encode(&event_db);
            index_event_para_id(trees.clone(), event.para_id.0, block_number, event_index, &value);
            index_event_account_id(trees.clone(), event.leaser, block_number, event_index, &value);
        },
        _ => {},
    }
}
