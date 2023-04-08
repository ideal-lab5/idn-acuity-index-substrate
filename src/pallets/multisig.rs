use subxt::{
    utils::AccountId32,
};

use parity_scale_codec::{Encode, Decode};
use serde::{Serialize, Deserialize};

use crate::shared::*;
use crate::substrate::*;

#[derive(Encode, Decode, Serialize, Deserialize, Debug, Clone)]
pub struct Timepoint {
	/// The height of the chain at the point in time.
	height: u32,
	/// The index of the extrinsic at the point in time.
	index: u32,
}

#[derive(Encode, Decode, Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "variant", content = "details")]
pub enum Multisig {
    #[serde(rename_all = "camelCase")]
	NewMultisig {
	    approving: AccountId32,
	    multisig: AccountId32,
	    call_hash: [u8; 32],
	},
    #[serde(rename_all = "camelCase")]
	MultisigApproval {
		approving: AccountId32,
		timepoint: Timepoint,
		multisig: AccountId32,
		call_hash: [u8; 32],
	},
    #[serde(rename_all = "camelCase")]
	MultisigExecuted {
		approving: AccountId32,
		timepoint: Timepoint,
		multisig: AccountId32,
		call_hash: [u8; 32],
//		result: DispatchResult,
	},
    #[serde(rename_all = "camelCase")]
	MultisigCancelled {
		cancelling: AccountId32,
		timepoint: Timepoint,
		multisig: AccountId32,
		call_hash: [u8; 32],
	},
}

pub fn multisig_index_event(trees: Trees, block_number: u32, event_index: u32, event: subxt::events::EventDetails) -> Result<(), subxt::Error> {
    match event.variant_name() {
        "NewMultisig" => {
            let event = event.as_event::<polkadot::multisig::events::NewMultisig>()?.unwrap();
            let event_db = Event::Multisig(
                Multisig::NewMultisig {
                    approving: event.approving.clone(),
                    multisig: event.multisig.clone(),
                    call_hash: event.call_hash,
                }
            );
            let value = Event::encode(&event_db);
            index_event_account_id(trees.clone(), event.approving, block_number, event_index, &value);
            index_event_account_id(trees.clone(), event.multisig, block_number, event_index, &value);
            Ok(())
        },
        "MultisigApproval" => {
            let event = event.as_event::<polkadot::multisig::events::MultisigApproval>()?.unwrap();
            let event_db = Event::Multisig(
                Multisig::MultisigApproval {
                    approving: event.approving.clone(),
		            timepoint: Timepoint { height: event.timepoint.height, index: event.timepoint.index },
                    multisig: event.multisig.clone(),
		            call_hash: event.call_hash,
                }
            );
            let value = Event::encode(&event_db);
            index_event_account_id(trees.clone(), event.approving, block_number, event_index, &value);
            index_event_account_id(trees.clone(), event.multisig, block_number, event_index, &value);
            Ok(())
        },
        "MultisigExecuted" => {
            let event = event.as_event::<polkadot::multisig::events::MultisigExecuted>()?.unwrap();
            let event_db = Event::Multisig(
                Multisig::MultisigExecuted {
                    approving: event.approving.clone(),
		            timepoint: Timepoint { height: event.timepoint.height, index: event.timepoint.index },
                    multisig: event.multisig.clone(),
		            call_hash: event.call_hash,
                }
            );
            let value = Event::encode(&event_db);
            index_event_account_id(trees.clone(), event.approving, block_number, event_index, &value);
            index_event_account_id(trees.clone(), event.multisig, block_number, event_index, &value);
            Ok(())
        },
        "MultisigCancelled" => {
            let event = event.as_event::<polkadot::multisig::events::MultisigCancelled>()?.unwrap();
            let event_db = Event::Multisig(
                Multisig::MultisigCancelled {
                    cancelling: event.cancelling.clone(),
		            timepoint: Timepoint { height: event.timepoint.height, index: event.timepoint.index },
                    multisig: event.multisig.clone(),
		            call_hash: event.call_hash,
                }
            );
            let value = Event::encode(&event_db);
            index_event_account_id(trees.clone(), event.cancelling, block_number, event_index, &value);
            index_event_account_id(trees.clone(), event.multisig, block_number, event_index, &value);
            Ok(())
        },
        _ => Ok(()),
    }
}
