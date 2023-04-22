use parity_scale_codec::{Encode, Decode};
use serde::Serialize;

use crate::shared::*;
use crate::substrate::*;

#[derive(Encode, Decode, Debug, Clone)]
pub struct MessageId(pub [u8; 32]);

impl Serialize for MessageId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut hex_string = "0x".to_owned();
        hex_string.push_str(&hex::encode(self.0));
        serializer.serialize_str(&hex_string)
    }
}

pub type OverweightIndex = u64;

#[derive(Encode, Decode, Serialize, Debug, Clone)]
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

#[derive(Encode, Decode, Serialize, Debug, Clone)]
#[serde(tag = "variant", content = "details")]
pub enum Ump {
    InvalidFormat {
        id: MessageId,
    },
    UnsupportedVersion {
        id: MessageId,
    },
    ExecutedUpward {
        id: MessageId,
    //    outcome: Outcome,
    },
    WeightExhausted {
        id: MessageId,
        remaining: Weight,
        required: Weight,
    },
    UpwardMessagesReceived {
        para: ParaId,
        count: u32,
        size: u32,
    },
    OverweightEnqueued {
        para: ParaId,
        id: MessageId,
        overweight_index: OverweightIndex,
        required: Weight,
    },
    OverweightServiced {
        overweight_index: OverweightIndex,
        used: Weight,
    },
}

pub fn parachains_ump_index_event(trees: Trees, block_number: u32, event_index: u32, event: subxt::events::EventDetails) -> Result<(), subxt::Error> {
    match event.variant_name() {
        "InvalidFormat" => {
            let event = event.as_event::<polkadot::ump::events::InvalidFormat>()?.unwrap();
            let event_db = Event::Ump(
                Ump::InvalidFormat {
                    id: MessageId(event.0)
                }
            );
            let value = Event::encode(&event_db);
            index_event_message_id(trees, event.0, block_number, event_index, &value);
            Ok(())
        },
        "UnsupportedVersion" => {
            let event = event.as_event::<polkadot::ump::events::UnsupportedVersion>()?.unwrap();
            let event_db = Event::Ump(
                Ump::UnsupportedVersion {
                    id: MessageId(event.0)
                }
            );
            let value = Event::encode(&event_db);
            index_event_message_id(trees, event.0, block_number, event_index, &value);
            Ok(())
        },
        "ExecutedUpward" => {
            let event = event.as_event::<polkadot::ump::events::ExecutedUpward>()?.unwrap();
            let event_db = Event::Ump(
                Ump::ExecutedUpward {
                    id: MessageId(event.0)
                }
            );
            let value = Event::encode(&event_db);
            index_event_message_id(trees, event.0, block_number, event_index, &value);
            Ok(())
        },
        "WeightExhausted" => {
            let event = event.as_event::<polkadot::ump::events::WeightExhausted>()?.unwrap();
            let event_db = Event::Ump (
                Ump::WeightExhausted {
                    id: MessageId(event.0),
                    remaining: event.1.into(),
                    required: event.2.into(),
                }
            );
            let value = Event::encode(&event_db);
            index_event_message_id(trees, event.0, block_number, event_index, &value);
            Ok(())
        },
        "UpwardMessagesReceived" => {
            let event = event.as_event::<polkadot::ump::events::UpwardMessagesReceived>()?.unwrap();
            let event_db = Event::Ump(
                Ump::UpwardMessagesReceived {
                    para: ParaId(event.0.0),
                    count: event.1,
                    size: event.2,
                }
            );
            let value = Event::encode(&event_db);
            index_event_para_id(trees, event.0.0, block_number, event_index, &value);
            Ok(())
        },
        "OverweightEnqueued" => {
            let event = event.as_event::<polkadot::ump::events::OverweightEnqueued>()?.unwrap();
            let event_db = Event::Ump(
                Ump::OverweightEnqueued {
                    para: ParaId(event.0.0),
                    id: MessageId(event.1),
                    overweight_index: event.2,
                    required: event.3.into(),
                }
            );
            let value = Event::encode(&event_db);
            index_event_para_id(trees.clone(), event.0.0, block_number, event_index, &value);
            index_event_message_id(trees, event.1, block_number, event_index, &value);
            Ok(())
        },
        _ => Ok(()),
    }
}

