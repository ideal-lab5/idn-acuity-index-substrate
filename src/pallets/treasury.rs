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
pub enum Treasury {
    #[serde(rename_all = "camelCase")]
	Proposed {
	    proposal_index: u32,
	},
    #[serde(rename_all = "camelCase")]
	Awarded {
	    proposal_index: u32,
	    award: u128,
	    account: AccountId32,
	},
    #[serde(rename_all = "camelCase")]
	Rejected {
	    proposal_index: u32,
	    slashed: u128,
	},
    #[serde(rename_all = "camelCase")]
	SpendApproved {
		proposal_index: u32,
		amount: u128,
		beneficiary: AccountId32,
	},
}

pub fn treasury_index_event(trees: Trees, block_number: u32, event_index: u32, event: subxt::events::EventDetails) -> Result<(), subxt::Error> {
    match event.variant_name() {
        "Proposed" => {
            let event = event.as_event::<polkadot::treasury::events::Proposed>().unwrap().unwrap();
            let event_db = Event::Treasury(
                Treasury::Proposed {
		            proposal_index: event.proposal_index,
                }
            );
            let value = Event::encode(&event_db);
            index_event_proposal_index(trees.clone(), event.proposal_index, block_number, event_index, &value);
            Ok(())
        },
        "Awarded" => {
            let event = event.as_event::<polkadot::treasury::events::Awarded>().unwrap().unwrap();
            let event_db = Event::Treasury(
                Treasury::Awarded {
		            proposal_index: event.proposal_index,
	                award: event.award,
	                account: event.account.clone(),
                }
            );
            let value = Event::encode(&event_db);
            index_event_proposal_index(trees.clone(), event.proposal_index, block_number, event_index, &value);
            index_event_account_id(trees.clone(), event.account, block_number, event_index, &value);
            Ok(())
        },
        "Rejected" => {
            let event = event.as_event::<polkadot::treasury::events::Rejected>().unwrap().unwrap();
            let event_db = Event::Treasury(
                Treasury::Rejected {
		            proposal_index: event.proposal_index,
	                slashed: event.slashed,
                }
            );
            let value = Event::encode(&event_db);
            index_event_proposal_index(trees.clone(), event.proposal_index, block_number, event_index, &value);
            Ok(())
        },
        "SpendApproved" => {
            let event = event.as_event::<polkadot::treasury::events::SpendApproved>().unwrap().unwrap();
            let event_db = Event::Treasury(
                Treasury::SpendApproved {
		            proposal_index: event.proposal_index,
	                amount: event.amount,
	                beneficiary: event.beneficiary.clone(),
                }
            );
            let value = Event::encode(&event_db);
            index_event_proposal_index(trees.clone(), event.proposal_index, block_number, event_index, &value);
            index_event_account_id(trees.clone(), event.beneficiary, block_number, event_index, &value);
            Ok(())
        },
        _ => Ok(()),
    }
}
