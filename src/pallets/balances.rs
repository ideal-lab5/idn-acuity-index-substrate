use subxt::{
    utils::AccountId32,
};

use bincode::{Encode, Decode};
use serde::{Serialize, Deserialize};

#[derive(Encode, Decode, Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub enum Status {
    Free,
    Reserved,
}

use crate::polkadot::runtime_types::frame_support::traits::tokens::misc::BalanceStatus;

impl From<&BalanceStatus> for Status {
    fn from(x: &BalanceStatus) -> Status {
        match x {
            BalanceStatus::Free => Status::Free,
            BalanceStatus::Reserved => Status::Reserved,
        }
    }
}

#[derive(Encode, Decode, Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "variant")]
pub enum Balances {
    #[serde(rename_all = "camelCase")]
    Endowed {
        #[bincode(with_serde)]
        account: AccountId32,
        free_balance: u128,
    },
    #[serde(rename_all = "camelCase")]
    DustLost {
        #[bincode(with_serde)]
        account: AccountId32,
        amount: u128,
    },
    #[serde(rename_all = "camelCase")]
    Transfer {
        #[bincode(with_serde)]
        from: AccountId32,
        #[bincode(with_serde)]
        to: AccountId32,
        value: u128,
    },
    #[serde(rename_all = "camelCase")]
	BalanceSet {
        #[bincode(with_serde)]
	    who: AccountId32,
	    free: u128,
	    reserved: u128,
    },
    #[serde(rename_all = "camelCase")]
	Reserved {
        #[bincode(with_serde)]
	    who: AccountId32,
	    amount: u128,
	},
    #[serde(rename_all = "camelCase")]
	Unreserved {
        #[bincode(with_serde)]
	    who: AccountId32,
	    amount: u128,
    },
    #[serde(rename_all = "camelCase")]
	ReserveRepatriated {
        #[bincode(with_serde)]
		from: AccountId32,
        #[bincode(with_serde)]
		to: AccountId32,
		amount: u128,
		destination_status: Status,
	},
    #[serde(rename_all = "camelCase")]
	Deposit {
        #[bincode(with_serde)]
	    who: AccountId32,
	    amount: u128,
	},
    #[serde(rename_all = "camelCase")]
	Withdraw {
        #[bincode(with_serde)]
	    who: AccountId32,
	    amount: u128,
	},
    #[serde(rename_all = "camelCase")]
	Slashed {
        #[bincode(with_serde)]
	    who: AccountId32,
	    amount: u128,
	},
}

