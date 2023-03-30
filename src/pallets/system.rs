use subxt::{
    utils::AccountId32,
};

use bincode::{Encode, Decode};
use serde::{Serialize, Deserialize};

use crate::shared::*;
use crate::substrate::*;

type Hash = Vec<u8>;

#[derive(Encode, Decode, Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "variant")]
pub enum System {
    #[serde(rename_all = "camelCase")]
	NewAccount {
        #[bincode(with_serde)]
        account: AccountId32,
	},
    #[serde(rename_all = "camelCase")]
	KilledAccount {
        #[bincode(with_serde)]
        account: AccountId32,
	},
    #[serde(rename_all = "camelCase")]
	Remarked {
        #[bincode(with_serde)]
        sender: AccountId32,
	    hash: Hash,
	},
}

pub fn system_index_event(trees: Trees, block_number: u32, event_index: u32, event: subxt::events::EventDetails) {
    match event.variant_name() {
        "NewAccount" => {
            let event = event.as_event::<polkadot::system::events::NewAccount>().unwrap().unwrap();
            let event_db = Event::System(
                System::NewAccount {
                    account: event.account.clone(),
                }
            );
            let value = bincode::encode_to_vec(&event_db, bincode::config::standard()).unwrap();
            index_event_account_id(trees.clone(), event.account, block_number, event_index, &value);
        },
        "KilledAccount" => {
            let event = event.as_event::<polkadot::system::events::KilledAccount>().unwrap().unwrap();
            let event_db = Event::System(
                System::KilledAccount {
                    account: event.account.clone(),
                }
            );
            let value = bincode::encode_to_vec(&event_db, bincode::config::standard()).unwrap();
            index_event_account_id(trees.clone(), event.account, block_number, event_index, &value);
        },
        "Remarked" => {
            let event = event.as_event::<polkadot::system::events::Remarked>().unwrap().unwrap();
            let event_db = Event::System(
                System::Remarked {
                    sender: event.sender.clone(),
                    hash: event.hash.as_ref().to_vec(),
                }
            );
            let value = bincode::encode_to_vec(&event_db, bincode::config::standard()).unwrap();
            index_event_account_id(trees.clone(), event.sender, block_number, event_index, &value);
        },
        _ => {},
    }
}
