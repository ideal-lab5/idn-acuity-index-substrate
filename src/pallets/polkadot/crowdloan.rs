use subxt::{
    utils::AccountId32,
};

use parity_scale_codec::{Encode, Decode};
use serde::{Serialize, Deserialize};

use crate::shared::*;
use crate::substrate::*;

#[derive(Encode, Decode, Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "variant")]
pub enum Crowdloan {
    #[serde(rename_all = "camelCase")]
	Created {
	    para_id: u32,
	},
    #[serde(rename_all = "camelCase")]
	Contributed {
	    who: AccountId32,
	    fund_index: u32,
	    amount: u128
	},
    #[serde(rename_all = "camelCase")]
	Withdrew {
	    who: AccountId32,
	    fund_index: u32,
	    amount: u128,
	},
    #[serde(rename_all = "camelCase")]
	PartiallyRefunded {
	    para_id: u32,
	},
    #[serde(rename_all = "camelCase")]
	AllRefunded {
	    para_id: u32,
	},
    #[serde(rename_all = "camelCase")]
	Dissolved {
	    para_id: u32,
	},
    #[serde(rename_all = "camelCase")]
	HandleBidResult {
	    para_id: u32,
//	    result: DispatchResult,
	},
    #[serde(rename_all = "camelCase")]
	Edited {
	    para_id: u32,
	},
    #[serde(rename_all = "camelCase")]
	MemoUpdated {
	    who: AccountId32,
	    para_id: u32,
	    memo: Vec<u8>,
	},
    #[serde(rename_all = "camelCase")]
	AddedToNewRaise {
	    para_id: u32,
	},
}

pub fn crowdloan_index_event(trees: Trees, block_number: u32, event_index: u32, event: subxt::events::EventDetails) {
    match event.variant_name() {
        "Created" => {
            let event = event.as_event::<polkadot::crowdloan::events::Created>().unwrap().unwrap();
            let event_db = Event::Crowdloan(
                Crowdloan::Created {
            	    para_id: event.para_id.0,
                }
            );
            let value = Event::encode(&event_db);
            index_event_para_id(trees.clone(), event.para_id.0, block_number, event_index, &value);
        },
        "Contributed" => {
            let event = event.as_event::<polkadot::crowdloan::events::Contributed>().unwrap().unwrap();
            let event_db = Event::Crowdloan(
                Crowdloan::Contributed {
	                who: event.who.clone(),
	                fund_index: event.fund_index.0,
	                amount: event.amount,
                }
            );
            let value = Event::encode(&event_db);
            index_event_account_id(trees.clone(), event.who, block_number, event_index, &value);
            index_event_para_id(trees.clone(), event.fund_index.0, block_number, event_index, &value);
        },
        "Withdrew" => {
            let event = event.as_event::<polkadot::crowdloan::events::Withdrew>().unwrap().unwrap();
            let event_db = Event::Crowdloan(
                Crowdloan::Withdrew {
	                who: event.who.clone(),
	                fund_index: event.fund_index.0,
	                amount: event.amount,
                }
            );
            let value = Event::encode(&event_db);
            index_event_account_id(trees.clone(), event.who, block_number, event_index, &value);
            index_event_para_id(trees.clone(), event.fund_index.0, block_number, event_index, &value);
        },
        "PartiallyRefunded" => {
            let event = event.as_event::<polkadot::crowdloan::events::PartiallyRefunded>().unwrap().unwrap();
            let event_db = Event::Crowdloan(
                Crowdloan::PartiallyRefunded {
	                para_id: event.para_id.0,
                }
            );
            let value = Event::encode(&event_db);
            index_event_para_id(trees.clone(), event.para_id.0, block_number, event_index, &value);
        },
        "AllRefunded" => {
            let event = event.as_event::<polkadot::crowdloan::events::AllRefunded>().unwrap().unwrap();
            let event_db = Event::Crowdloan(
                Crowdloan::AllRefunded {
	                para_id: event.para_id.0,
                }
            );
            let value = Event::encode(&event_db);
            index_event_para_id(trees.clone(), event.para_id.0, block_number, event_index, &value);
        },
        "Dissolved" => {
            let event = event.as_event::<polkadot::crowdloan::events::Dissolved>().unwrap().unwrap();
            let event_db = Event::Crowdloan(
                Crowdloan::Dissolved {
	                para_id: event.para_id.0,
                }
            );
            let value = Event::encode(&event_db);
            index_event_para_id(trees.clone(), event.para_id.0, block_number, event_index, &value);
        },
        "HandleBidResult" => {
            let event = event.as_event::<polkadot::crowdloan::events::HandleBidResult>().unwrap().unwrap();
            let event_db = Event::Crowdloan(
                Crowdloan::HandleBidResult {
	                para_id: event.para_id.0,
                }
            );
            let value = Event::encode(&event_db);
            index_event_para_id(trees.clone(), event.para_id.0, block_number, event_index, &value);
        },
        "Edited" => {
            let event = event.as_event::<polkadot::crowdloan::events::Edited>().unwrap().unwrap();
            let event_db = Event::Crowdloan(
                Crowdloan::Edited {
	                para_id: event.para_id.0,
                }
            );
            let value = Event::encode(&event_db);
            index_event_para_id(trees.clone(), event.para_id.0, block_number, event_index, &value);
        },
        "MemoUpdated" => {
            let event = event.as_event::<polkadot::crowdloan::events::MemoUpdated>().unwrap().unwrap();
            let event_db = Event::Crowdloan(
                Crowdloan::MemoUpdated {
	                who: event.who.clone(),
	                para_id: event.para_id.0,
	                memo: event.memo,
                }
            );
            let value = Event::encode(&event_db);
            index_event_account_id(trees.clone(), event.who, block_number, event_index, &value);
            index_event_para_id(trees.clone(), event.para_id.0, block_number, event_index, &value);
        },
        "AddedToNewRaise" => {
            let event = event.as_event::<polkadot::crowdloan::events::AddedToNewRaise>().unwrap().unwrap();
            let event_db = Event::Crowdloan(
                Crowdloan::AddedToNewRaise {
	                para_id: event.para_id.0,
                }
            );
            let value = Event::encode(&event_db);
            index_event_para_id(trees.clone(), event.para_id.0, block_number, event_index, &value);
        },
        _ => {},
    }
}
