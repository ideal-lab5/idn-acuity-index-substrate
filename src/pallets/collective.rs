use subxt::{
    utils::AccountId32,
};

use parity_scale_codec::{Encode, Decode};
use serde::{Serialize, Deserialize};

use crate::shared::*;
use crate::substrate::*;

#[derive(Encode, Decode, Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "variant", content = "details")]
pub enum Collective {
    #[serde(rename_all = "camelCase")]
	Proposed {
		account: AccountId32,
		proposal_index: u32,
		proposal_hash: [u8; 32],
		threshold: u32,
	},
    #[serde(rename_all = "camelCase")]
	Voted {
		account: AccountId32,
		proposal_hash: [u8; 32],
		voted: bool,
		yes: u32,
		no: u32,
	},
    #[serde(rename_all = "camelCase")]
	Approved {
	    proposal_hash: [u8; 32],
	},
    #[serde(rename_all = "camelCase")]
	Disapproved {
	    proposal_hash: [u8; 32],
	},
    #[serde(rename_all = "camelCase")]
	Executed {
	    proposal_hash: [u8; 32],
	},
    #[serde(rename_all = "camelCase")]
	MemberExecuted {
	    proposal_hash: [u8; 32],
	},
    #[serde(rename_all = "camelCase")]
	Closed {
	    proposal_hash: [u8; 32],
	    yes: u32,
	    no: u32,
	},
}

pub fn collective_index_event(trees: Trees, block_number: u32, event_index: u32, event: subxt::events::EventDetails) -> Result<(), subxt::Error> {
    match event.variant_name() {
        "Proposed" => {
            let event = event.as_event::<polkadot::council::events::Proposed>()?
                .ok_or(subxt::Error::Other("Event not found.".to_string()))?;
            let event_db = Event::Collective(
                Collective::Proposed {
		            account: event.account.clone(),
		            proposal_index: event.proposal_index,
		            proposal_hash: event.proposal_hash.into(),
		            threshold: event.threshold,
                }
            );
            let value = Event::encode(&event_db);
            index_event_account_id(trees.clone(), event.account, block_number, event_index, &value);
            index_event_proposal_index(trees.clone(), event.proposal_index, block_number, event_index, &value);
            index_event_proposal_hash(trees.clone(), event.proposal_hash.into(), block_number, event_index, &value);
            Ok(())
        },
        "Voted" => {
            let event = event.as_event::<polkadot::council::events::Voted>()?
                .ok_or(subxt::Error::Other("Event not found.".to_string()))?;
            let event_db = Event::Collective(
                Collective::Voted {
		            account: event.account.clone(),
		            proposal_hash: event.proposal_hash.into(),
		            voted: event.voted,
		            yes: event.yes,
		            no: event.no,
                }
            );
            let value = Event::encode(&event_db);
            index_event_account_id(trees.clone(), event.account, block_number, event_index, &value);
            index_event_proposal_hash(trees.clone(), event.proposal_hash.into(), block_number, event_index, &value);
            Ok(())
        },
        "Approved" => {
            let event = event.as_event::<polkadot::council::events::Approved>()?
                .ok_or(subxt::Error::Other("Event not found.".to_string()))?;
            let event_db = Event::Collective(
                Collective::Approved {
            	    proposal_hash: event.proposal_hash.into(),
                }
            );
            let value = Event::encode(&event_db);
            index_event_proposal_hash(trees.clone(), event.proposal_hash.into(), block_number, event_index, &value);
            Ok(())
        },
        "Disapproved" => {
            let event = event.as_event::<polkadot::council::events::Disapproved>()?
                .ok_or(subxt::Error::Other("Event not found.".to_string()))?;
            let event_db = Event::Collective(
                Collective::Disapproved {
            	    proposal_hash: event.proposal_hash.into(),
                }
            );
            let value = Event::encode(&event_db);
            index_event_proposal_hash(trees.clone(), event.proposal_hash.into(), block_number, event_index, &value);
            Ok(())
        },
        "Executed" => {
            let event = event.as_event::<polkadot::council::events::Executed>()?
                .ok_or(subxt::Error::Other("Event not found.".to_string()))?;
            let event_db = Event::Collective(
                Collective::Executed {
            	    proposal_hash: event.proposal_hash.into(),
                }
            );
            let value = Event::encode(&event_db);
            index_event_proposal_hash(trees.clone(), event.proposal_hash.into(), block_number, event_index, &value);
            Ok(())
        },
        "MemberExecuted" => {
            let event = event.as_event::<polkadot::council::events::MemberExecuted>()?
                .ok_or(subxt::Error::Other("Event not found.".to_string()))?;
            let event_db = Event::Collective(
                Collective::MemberExecuted {
            	    proposal_hash: event.proposal_hash.into(),
                }
            );
            let value = Event::encode(&event_db);
            index_event_proposal_hash(trees.clone(), event.proposal_hash.into(), block_number, event_index, &value);
            Ok(())
        },
        "Closed" => {
            let event = event.as_event::<polkadot::council::events::Closed>()?
                .ok_or(subxt::Error::Other("Event not found.".to_string()))?;
            let event_db = Event::Collective(
                Collective::Closed {
            	    proposal_hash: event.proposal_hash.into(),
	                yes: event.yes,
	                no: event.no,
                }
            );
            let value = Event::encode(&event_db);
            index_event_proposal_hash(trees.clone(), event.proposal_hash.into(), block_number, event_index, &value);
            Ok(())
        },
        _ => Ok(()),
    }
}
