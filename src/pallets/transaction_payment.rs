use subxt::{
    utils::AccountId32,
};

use bincode::{Encode, Decode};
use serde::{Serialize, Deserialize};

use crate::shared::*;
use crate::substrate::*;

#[derive(Encode, Decode, Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "variant")]
pub enum TransactionPayment {
    #[serde(rename_all = "camelCase")]
	TransactionFeePaid {
        #[bincode(with_serde)]
        who: AccountId32,
        actual_fee: u128,
        tip: u128,
	},
}

pub fn transaction_payment_index_event(trees: Trees, block_number: u32, event_index: u32, event: subxt::events::EventDetails) {
    match event.variant_name() {
        "TransactionFeePaid" => {
    println!("TransactionFeePaid");
            let event = event.as_event::<polkadot::transaction_payment::events::TransactionFeePaid>().unwrap().unwrap();
            let event_db = Event::TransactionPayment(
                TransactionPayment::TransactionFeePaid {
                    who: event.who.clone(),
                    actual_fee: event.actual_fee,
                    tip: event.tip,
                }
            );
            let value = bincode::encode_to_vec(&event_db, bincode::config::standard()).unwrap();
            index_event_account_id(trees.clone(), event.who, block_number, event_index, &value);
        },
        _ => {},
    }
}
