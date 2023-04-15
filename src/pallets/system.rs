use subxt::{
    utils::AccountId32,
};

use parity_scale_codec::{Encode, Decode};
use serde::{Serialize, Deserialize};

use crate::shared::*;
use crate::substrate::*;

type Hash = Vec<u8>;

#[derive(Encode, Decode, Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "variant", content = "details")]
pub enum System {
    #[serde(rename_all = "camelCase")]
    NewAccount {
        account: AccountId32,
    },
    #[serde(rename_all = "camelCase")]
    KilledAccount {
        account: AccountId32,
    },
    #[serde(rename_all = "camelCase")]
    Remarked {
        sender: AccountId32,
        hash: Hash,
    },
}

pub fn system_index_event(trees: Trees, block_number: u32, event_index: u32, event: subxt::events::EventDetails) -> Result<(), subxt::Error> {
    match event.variant_name() {
        "NewAccount" => {
            let event = event.as_event::<polkadot::system::events::NewAccount>()?.unwrap();
            let event_db = Event::System(
                System::NewAccount {
                    account: event.account.clone(),
                }
            );
            let value = Event::encode(&event_db);
            index_event_account_id(trees.clone(), event.account, block_number, event_index, &value);
            Ok(())
        },
        "KilledAccount" => {
            let event = event.as_event::<polkadot::system::events::KilledAccount>()?.unwrap();
            let event_db = Event::System(
                System::KilledAccount {
                    account: event.account.clone(),
                }
            );
            let value = Event::encode(&event_db);
            index_event_account_id(trees.clone(), event.account, block_number, event_index, &value);
            Ok(())
        },
        "Remarked" => {
            let event = event.as_event::<polkadot::system::events::Remarked>()?.unwrap();
            let event_db = Event::System(
                System::Remarked {
                    sender: event.sender.clone(),
                    hash: event.hash.as_ref().to_vec(),
                }
            );
            let value = Event::encode(&event_db);
            index_event_account_id(trees.clone(), event.sender, block_number, event_index, &value);
            Ok(())
        },
        _ => Ok(()),
    }
}
