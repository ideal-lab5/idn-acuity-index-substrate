use parity_scale_codec::{Encode, Decode};
use serde::{Serialize, Deserialize};

use crate::shared::*;
use crate::substrate::*;

pub type MessageId = [u8; 32];
pub type OverweightIndex = u64;

#[derive(Encode, Decode, Serialize, Deserialize, Debug, Clone)]
pub struct Weight {
	ref_time: u64,
	proof_size: u64,
}

use crate::shared::polkadot::runtime_types::sp_weights::weight_v2::Weight as SubWeight;

impl From<SubWeight> for Weight {
    fn from(x: SubWeight) -> Self {
        Weight {
            ref_time: x.ref_time,
            proof_size: x.proof_size,
        }
    }
}

#[derive(Encode, Decode, Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub enum Ump {
	InvalidFormat(MessageId),
	UnsupportedVersion(MessageId),
	ExecutedUpward(MessageId, /*Outcome*/),
	WeightExhausted(MessageId, Weight, Weight),
	UpwardMessagesReceived(ParaId, u32, u32),
	OverweightEnqueued(ParaId, MessageId, OverweightIndex, Weight),
	OverweightServiced(OverweightIndex, Weight),
}

pub fn parachains_ump_index_event(trees: Trees, block_number: u32, event_index: u32, event: subxt::events::EventDetails) {
    match event.variant_name() {
        "InvalidFormat" => {
            let event = event.as_event::<polkadot::ump::events::InvalidFormat>().unwrap().unwrap();
            let event_db = Event::Ump(
                Ump::InvalidFormat(event.0)
            );
            let value = Event::encode(&event_db);
            index_event_message_id(trees.clone(), event.0, block_number, event_index, &value);
        },
        "UnsupportedVersion" => {
            let event = event.as_event::<polkadot::ump::events::UnsupportedVersion>().unwrap().unwrap();
            let event_db = Event::Ump(
                Ump::UnsupportedVersion(event.0)
            );
            let value = Event::encode(&event_db);
            index_event_message_id(trees.clone(), event.0, block_number, event_index, &value);
        },
        "ExecutedUpward" => {
            let event = event.as_event::<polkadot::ump::events::ExecutedUpward>().unwrap().unwrap();
            let event_db = Event::Ump(
                Ump::ExecutedUpward(event.0)
            );
            let value = Event::encode(&event_db);
            index_event_message_id(trees.clone(), event.0, block_number, event_index, &value);
        },
        "WeightExhausted" => {
            let event = event.as_event::<polkadot::ump::events::WeightExhausted>().unwrap().unwrap();
            let event_db = Event::Ump(
                Ump::WeightExhausted(event.0, event.1.into(), event.2.into())
            );
            let value = Event::encode(&event_db);
            index_event_message_id(trees.clone(), event.0, block_number, event_index, &value);
        },
        "UpwardMessagesReceived" => {
            let event = event.as_event::<polkadot::ump::events::UpwardMessagesReceived>().unwrap().unwrap();
            let event_db = Event::Ump(
                Ump::UpwardMessagesReceived(ParaId(event.0.0), event.1, event.2)
            );
            let value = Event::encode(&event_db);
            index_event_para_id(trees.clone(), event.0.0, block_number, event_index, &value);
        },
        "OverweightEnqueued" => {
            let event = event.as_event::<polkadot::ump::events::OverweightEnqueued>().unwrap().unwrap();
            let event_db = Event::Ump(
                Ump::OverweightEnqueued(ParaId(event.0.0), event.1, event.2, event.3.into())
            );
            let value = Event::encode(&event_db);
            index_event_para_id(trees.clone(), event.0.0, block_number, event_index, &value);
            index_event_message_id(trees.clone(), event.1, block_number, event_index, &value);
        },
        _ => {},
    }
}
