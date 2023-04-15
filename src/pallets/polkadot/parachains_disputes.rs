use parity_scale_codec::{Encode, Decode};
use serde::{Serialize, Deserialize};

use crate::shared::*;
use crate::substrate::*;

#[derive(Encode, Decode, Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CandidateHash([u8; 32]);

#[derive(Encode, Decode, Serialize, Deserialize, Debug, Clone)]
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

#[derive(Encode, Decode, Serialize, Deserialize, Debug, Clone)]
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

#[derive(Encode, Decode, Serialize, Deserialize, Debug, Clone)]
pub enum ParasDisputes {
    DisputeInitiated(CandidateHash, DisputeLocation),
    DisputeConcluded(CandidateHash, DisputeResult),
    DisputeTimedOut(CandidateHash),
}

pub fn parachains_disputes_index_event(trees: Trees, block_number: u32, event_index: u32, event: subxt::events::EventDetails) -> Result<(), subxt::Error> {
    match event.variant_name() {
        "DisputeInitiated" => {
            let event = event.as_event::<polkadot::paras_disputes::events::DisputeInitiated>()?.unwrap();
            let event_db = Event::ParasDisputes(
                ParasDisputes::DisputeInitiated(CandidateHash(event.0.0.0), event.1.into())
            );
            let value = Event::encode(&event_db);
            index_event_candidate_hash(trees.clone(), event.0.0.0, block_number, event_index, &value);
            Ok(())
        },
        "DisputeConcluded" => {
            let event = event.as_event::<polkadot::paras_disputes::events::DisputeConcluded>()?.unwrap();
            let event_db = Event::ParasDisputes(
                ParasDisputes::DisputeConcluded(CandidateHash(event.0.0.0), event.1.into())
            );
            let value = Event::encode(&event_db);
            index_event_candidate_hash(trees.clone(), event.0.0.0, block_number, event_index, &value);
            Ok(())
        },
        "DisputeTimedOut" => {
            let event = event.as_event::<polkadot::paras_disputes::events::DisputeTimedOut>()?.unwrap();
            let event_db = Event::ParasDisputes(
                ParasDisputes::DisputeTimedOut(CandidateHash(event.0.0.0))
            );
            let value = Event::encode(&event_db);
            index_event_candidate_hash(trees.clone(), event.0.0.0, block_number, event_index, &value);
            Ok(())
        },
        _ => Ok(()),
    }
}
