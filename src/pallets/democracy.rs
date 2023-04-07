use subxt::{
    utils::AccountId32,
};

use parity_scale_codec::{Encode, Decode};
use serde::{Serialize, Deserialize};

use crate::shared::*;
use crate::substrate::*;

#[derive(Encode, Decode, Serialize, Deserialize, Debug, Clone)]
pub enum VoteThreshold {
	SuperMajorityApprove,
	SuperMajorityAgainst,
	SimpleMajority,
}

use crate::polkadot::runtime_types::pallet_democracy::vote_threshold::VoteThreshold as SubVoteThreshold;

impl From<SubVoteThreshold> for VoteThreshold {
    fn from(x: SubVoteThreshold) -> Self {
        match x {
            SubVoteThreshold::SuperMajorityApprove => VoteThreshold::SuperMajorityApprove,
            SubVoteThreshold::SuperMajorityAgainst => VoteThreshold::SuperMajorityAgainst,
            SubVoteThreshold::SimpleMajority => VoteThreshold::SimpleMajority,
        }
    }
}

#[derive(Encode, Decode, Serialize, Deserialize, Debug, Clone)]
pub struct H256([u8; 32]);

#[derive(Encode, Decode, Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "variant", content = "details")]
pub enum Democracy {
    #[serde(rename_all = "camelCase")]
	Proposed {
	    proposal_index: u32,
	    deposit: u128,
	},
    #[serde(rename_all = "camelCase")]
	Tabled {
	    proposal_index: u32,
	    deposit: u128,
	},
    #[serde(rename_all = "camelCase")]
	Started {
	    ref_index: u32,
	    threshold: VoteThreshold,
	},
    #[serde(rename_all = "camelCase")]
	Passed {
	    ref_index: u32,
	},
    #[serde(rename_all = "camelCase")]
	NotPassed {
	    ref_index: u32,
	},
    #[serde(rename_all = "camelCase")]
	Cancelled {
	    ref_index: u32,
	},
    #[serde(rename_all = "camelCase")]
	Delegated {
	    who: AccountId32,
	    target: AccountId32,
	},
    #[serde(rename_all = "camelCase")]
	Undelegated {
	    account: AccountId32,
	},
    #[serde(rename_all = "camelCase")]
	Vetoed {
	    who: AccountId32,
	    proposal_hash: H256,
	    until: u32,
	},
    #[serde(rename_all = "camelCase")]
	Voted {
	    voter: AccountId32,
	    ref_index: u32,
	},
    #[serde(rename_all = "camelCase")]
	Seconded {
	    seconder: AccountId32,
	    prop_index: u32,
	},
    #[serde(rename_all = "camelCase")]
	ProposalCanceled {
	    prop_index: u32,
	},
}

pub fn democracy_index_event(trees: Trees, block_number: u32, event_index: u32, event: subxt::events::EventDetails) {
    match event.variant_name() {
        "Proposed" => {
            let event = event.as_event::<polkadot::democracy::events::Proposed>().unwrap().unwrap();
            let event_db = Event::Democracy(
                Democracy::Proposed {
	                proposal_index: event.proposal_index,
	                deposit: event.deposit,
                }
            );
            let value = Event::encode(&event_db);
            index_event_proposal_index(trees.clone(), event.proposal_index, block_number, event_index, &value);
        },
        "Tabled" => {
            let event = match event.as_event::<polkadot::democracy::events::Tabled>() {
                Ok(event) => event.unwrap(),
                Err(error) => {
                    println!("{}", error);
                    return;
                }
            };
            let event_db = Event::Democracy(
                Democracy::Tabled {
	                proposal_index: event.proposal_index,
	                deposit: event.deposit,
                }
            );
            let value = Event::encode(&event_db);
            index_event_proposal_index(trees.clone(), event.proposal_index, block_number, event_index, &value);
        },
        "Started" => {
            let event = event.as_event::<polkadot::democracy::events::Started>().unwrap().unwrap();
            let event_db = Event::Democracy(
                Democracy::Started {
	                ref_index: event.ref_index,
	                threshold: VoteThreshold::from(event.threshold),
                }
            );
            let value = Event::encode(&event_db);
            index_event_ref_index(trees.clone(), event.ref_index, block_number, event_index, &value);
        },
        "Passed" => {
            let event = event.as_event::<polkadot::democracy::events::Passed>().unwrap().unwrap();
            let event_db = Event::Democracy(
                Democracy::Passed {
	                ref_index: event.ref_index,
                }
            );
            let value = Event::encode(&event_db);
            index_event_ref_index(trees.clone(), event.ref_index, block_number, event_index, &value);
        },
        "NotPassed" => {
            let event = event.as_event::<polkadot::democracy::events::NotPassed>().unwrap().unwrap();
            let event_db = Event::Democracy(
                Democracy::NotPassed {
	                ref_index: event.ref_index,
                }
            );
            let value = Event::encode(&event_db);
            index_event_ref_index(trees.clone(), event.ref_index, block_number, event_index, &value);
        },
        "Cancelled" => {
            let event = event.as_event::<polkadot::democracy::events::Cancelled>().unwrap().unwrap();
            let event_db = Event::Democracy(
                Democracy::Cancelled {
	                ref_index: event.ref_index,
                }
            );
            let value = Event::encode(&event_db);
            index_event_ref_index(trees.clone(), event.ref_index, block_number, event_index, &value);
        },
        "Delegated" => {
            let event = event.as_event::<polkadot::democracy::events::Delegated>().unwrap().unwrap();
            let event_db = Event::Democracy(
                Democracy::Delegated {
	                who: event.who.clone(),
	                target: event.target.clone(),
                }
            );
            let value = Event::encode(&event_db);
            index_event_account_id(trees.clone(), event.who, block_number, event_index, &value);
            index_event_account_id(trees.clone(), event.target, block_number, event_index, &value);
        },
        "Undelegated" => {
            let event = event.as_event::<polkadot::democracy::events::Undelegated>().unwrap().unwrap();
            let event_db = Event::Democracy(
                Democracy::Undelegated {
	                account: event.account.clone(),
                }
            );
            let value = Event::encode(&event_db);
            index_event_account_id(trees.clone(), event.account, block_number, event_index, &value);
        },
        "Vetoed" => {
            let event = event.as_event::<polkadot::democracy::events::Vetoed>().unwrap().unwrap();
            let event_db = Event::Democracy(
                Democracy::Vetoed {
	                who: event.who.clone(),
	                proposal_hash: H256(event.proposal_hash.0),
	                until: event.until,
                }
            );
            let value = Event::encode(&event_db);
            index_event_account_id(trees.clone(), event.who, block_number, event_index, &value);
        },
        "Voted" => {
            let event = event.as_event::<polkadot::democracy::events::Voted>().unwrap().unwrap();
            let event_db = Event::Democracy(
                Democracy::Voted {
	                voter: event.voter.clone(),
	                ref_index: event.ref_index,
                }
            );
            let value = Event::encode(&event_db);
            index_event_account_id(trees.clone(), event.voter, block_number, event_index, &value);
            index_event_ref_index(trees.clone(), event.ref_index, block_number, event_index, &value);
        },
        "Seconded" => {
            let event = event.as_event::<polkadot::democracy::events::Seconded>().unwrap().unwrap();
            let event_db = Event::Democracy(
                Democracy::Seconded {
	                seconder: event.seconder.clone(),
	                prop_index: event.prop_index,
                }
            );
            let value = Event::encode(&event_db);
            index_event_account_id(trees.clone(), event.seconder, block_number, event_index, &value);
            index_event_proposal_index(trees.clone(), event.prop_index, block_number, event_index, &value);
        },
        "ProposalCanceled" => {
            let event = event.as_event::<polkadot::democracy::events::ProposalCanceled>().unwrap().unwrap();
            let event_db = Event::Democracy(
                Democracy::ProposalCanceled {
	                prop_index: event.prop_index,
                }
            );
            let value = Event::encode(&event_db);
            index_event_proposal_index(trees.clone(), event.prop_index, block_number, event_index, &value);
        },
        _ => {},
    }
}
