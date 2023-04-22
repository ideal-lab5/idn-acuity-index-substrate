use parity_scale_codec::{Encode, Decode};
use serde::Serialize;

use crate::shared::*;
use crate::substrate::*;

#[derive(Encode, Decode, Debug, Clone)]
pub struct CandidateHash(pub [u8; 32]);

impl Serialize for CandidateHash {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut hex_string = "0x".to_owned();
        hex_string.push_str(&hex::encode(self.0));
        serializer.serialize_str(&hex_string)
    }
}


#[derive(Encode, Decode, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub enum DisputeLocation {
    Local,
    Remote,
}

use crate::shared::polkadot::runtime_types::polkadot_runtime_parachains::disputes::DisputeLocation as SubDisputeLocation;

impl From<SubDisputeLocation> for DisputeLocation {
    fn from(x: SubDisputeLocation) -> Self {
        match x {
            SubDisputeLocation::Local => DisputeLocation::Local,
            SubDisputeLocation::Remote => DisputeLocation::Remote,
        }
    }
}

#[derive(Encode, Decode, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub enum DisputeResult {
    Valid,
    Invalid,
}

use crate::shared::polkadot::runtime_types::polkadot_runtime_parachains::disputes::DisputeResult as SubDisputeResult;

impl From<SubDisputeResult> for DisputeResult {
    fn from(x: SubDisputeResult) -> Self {
        match x {
            SubDisputeResult::Valid => DisputeResult::Valid,
            SubDisputeResult::Invalid => DisputeResult::Invalid,
        }
    }
}

#[derive(Encode, Decode, Serialize, Debug, Clone)]
#[serde(tag = "variant", content = "details")]
pub enum ParasDisputes {
    DisputeInitiated {
        candidate_hash: CandidateHash,
        dispute_location: DisputeLocation,
    },
    DisputeConcluded {
        candidate_hash: CandidateHash,
        dispute_location: DisputeResult,
    },
    DisputeTimedOut {
        candidate_hash: CandidateHash,
    },
}

pub fn parachains_disputes_index_event(trees: Trees, block_number: u32, event_index: u32, event: subxt::events::EventDetails) -> Result<(), subxt::Error> {
    match event.variant_name() {
        "DisputeInitiated" => {
            let event = event.as_event::<polkadot::paras_disputes::events::DisputeInitiated>()?.unwrap();
            let event_db = Event::ParasDisputes(
                ParasDisputes::DisputeInitiated {
                    candidate_hash: CandidateHash(event.0.0.0),
                    dispute_location: event.1.into(),
                }
            );
            let value = Event::encode(&event_db);
            index_event_candidate_hash(trees, event.0.0.0, block_number, event_index, &value);
            Ok(())
        },
        "DisputeConcluded" => {
            let event = event.as_event::<polkadot::paras_disputes::events::DisputeConcluded>()?.unwrap();
            let event_db = Event::ParasDisputes(
                ParasDisputes::DisputeConcluded {
                    candidate_hash: CandidateHash(event.0.0.0),
                    dispute_location: event.1.into(),
                }
            );
            let value = Event::encode(&event_db);
            index_event_candidate_hash(trees, event.0.0.0, block_number, event_index, &value);
            Ok(())
        },
        "DisputeTimedOut" => {
            let event = event.as_event::<polkadot::paras_disputes::events::DisputeTimedOut>()?.unwrap();
            let event_db = Event::ParasDisputes(
                ParasDisputes::DisputeTimedOut {
                    candidate_hash: CandidateHash(event.0.0.0),
                }
            );
            let value = Event::encode(&event_db);
            index_event_candidate_hash(trees, event.0.0.0, block_number, event_index, &value);
            Ok(())
        },
        _ => Ok(()),
    }
}
