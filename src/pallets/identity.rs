use subxt::{
    utils::AccountId32,
};

use bincode::{Encode, Decode};
use serde::{Serialize, Deserialize};

#[derive(Encode, Decode, Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "type")]
pub enum Identity {
    #[serde(rename_all = "camelCase")]
    IdentitySet {
        #[bincode(with_serde)]
        who: AccountId32,
    },
    #[serde(rename_all = "camelCase")]
    IdentityCleared {
        #[bincode(with_serde)]
        who: AccountId32,
        deposit: u128,
    },
    #[serde(rename_all = "camelCase")]
    IdentityKilled {
        #[bincode(with_serde)]
        who: AccountId32,
        deposit: u128,
    },
    #[serde(rename_all = "camelCase")]
    JudgementRequested {
        #[bincode(with_serde)]
        who: AccountId32,
        registrar_index: u32,
    },
    #[serde(rename_all = "camelCase")]
    JudgementUnrequested {
        #[bincode(with_serde)]
        who: AccountId32,
        registrar_index: u32,
    },
    #[serde(rename_all = "camelCase")]
    JudgementGiven {
        #[bincode(with_serde)]
        target: AccountId32,
        registrar_index: u32,
    },
    #[serde(rename_all = "camelCase")]
	RegistrarAdded {
	    registrar_index: u32,
	},
    #[serde(rename_all = "camelCase")]
	SubIdentityAdded {
        #[bincode(with_serde)]
	    sub: AccountId32,
        #[bincode(with_serde)]
	    main: AccountId32,
	    deposit: u128,
	},
    #[serde(rename_all = "camelCase")]
	SubIdentityRemoved {
        #[bincode(with_serde)]
	    sub: AccountId32,
        #[bincode(with_serde)]
	    main: AccountId32,
	    deposit: u128,
	},
    #[serde(rename_all = "camelCase")]
	SubIdentityRevoked {
        #[bincode(with_serde)]
	    sub: AccountId32,
        #[bincode(with_serde)]
	    main: AccountId32,
	    deposit: u128,
	},
}

