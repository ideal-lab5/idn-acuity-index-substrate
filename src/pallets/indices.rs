use subxt::{
    utils::AccountId32,
};

use parity_scale_codec::{Encode, Decode};
use serde::{Serialize, Deserialize};

use crate::shared::*;
use crate::substrate::*;


#[derive(Encode, Decode, Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "variant", content = "details")]
pub enum Indices {
    #[serde(rename_all = "camelCase")]
    IndexAssigned {
        who: AccountId32,
        index: u32,
    },
    #[serde(rename_all = "camelCase")]
    IndexFreed {
        index: u32,
    },
    #[serde(rename_all = "camelCase")]
    IndexFrozen {
        index: u32,
        who: AccountId32,
    },
}

pub fn indices_index_event(trees: Trees, block_number: u32, event_index: u32, event: subxt::events::EventDetails) -> Result<(), subxt::Error> {
    match event.variant_name() {
        "IndexAssigned" => {
            let event = event.as_event::<polkadot::indices::events::IndexAssigned>()?.unwrap();
            let event_db = Event::Indices(
                Indices::IndexAssigned {
                    who: event.who.clone(),
                    index: event.index,
                }
            );
            let value = Event::encode(&event_db);
            index_event_account_id(trees.clone(), event.who, block_number, event_index, &value);
            index_event_account_index(trees.clone(), event.index, block_number, event_index, &value);
            Ok(())
        },
        "IndexFreed" => {
            let event = event.as_event::<polkadot::indices::events::IndexFreed>()?.unwrap();
            let event_db = Event::Indices(
                Indices::IndexFreed {
                    index: event.index,
                }
            );
            let value = Event::encode(&event_db);
            index_event_account_index(trees.clone(), event.index, block_number, event_index, &value);
            Ok(())
        },
        "IndexFrozen" => {
            let event = event.as_event::<polkadot::indices::events::IndexFrozen>()?.unwrap();
            let event_db = Event::Indices(
                Indices::IndexFrozen {
                    index: event.index,
                    who: event.who.clone(),
                }
            );
            let value = Event::encode(&event_db);
            index_event_account_index(trees.clone(), event.index, block_number, event_index, &value);
            index_event_account_id(trees.clone(), event.who, block_number, event_index, &value);
            Ok(())
        },
        _ => Ok(()),
    }
}
