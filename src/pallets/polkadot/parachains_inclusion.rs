use parity_scale_codec::{Encode, Decode};
use serde::Serialize;

use crate::shared::*;
use crate::substrate::*;


pub struct CandidateReceipt(pub [u8; 32]);
pub struct HeadData(pub Vec<u8>);
pub struct CoreIndex(pub u32);
pub struct GroupIndex(pub u32);

#[derive(Encode, Decode, Serialize, Debug, Clone)]
#[serde(tag = "variant", content = "details")]
pub enum ParaInclusion {
    #[serde(rename_all = "camelCase")]
    CandidateBacked(CandidateReceipt, HeadData, CoreIndex, GroupIndex),
    #[serde(rename_all = "camelCase")]
    CandidateIncluded(CandidateReceipt, HeadData, CoreIndex, GroupIndex),
    #[serde(rename_all = "camelCase")]
    CandidateTimedOut(CandidateReceipt, HeadData, CoreIndex),
}

pub fn parachains_inclusion_index_event(trees: Trees, block_number: u32, event_index: u32, event: subxt::events::EventDetails) -> Result<(), subxt::Error> {
    match event.variant_name() {
        "CandidateBacked" => {
            let event = event.as_event::<polkadot::parachains_inclusion::events::CandidateBacked>()?.unwrap();
            let event_db = Event::ParaInclusion(
                ParaInclusion::CandidateBacked (
                    who: event.who.clone(),
                    ethereum_address: event.ethereum_address.0,
                    amount: event.amount,
                )
            );
            let value = Event::encode(&event_db);
//            index_event_account_id(trees, event.who, block_number, event_index, &value);
            Ok(())
        },
        _ => Ok(()),
    }
}

