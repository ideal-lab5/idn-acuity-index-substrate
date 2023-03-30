use subxt::{
    utils::AccountId32,
};

use bincode::{Encode, Decode};
use serde::{Serialize, Deserialize};

use crate::shared::*;
use crate::substrate::*;

type Cid = Vec<u8>;
type UnscrupulousItemOf = Vec<u8>;

#[derive(Encode, Decode, Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "variant")]
pub enum Alliance {
    #[serde(rename_all = "camelCase")]
    NewRuleSet {
        rule: Cid,
    },
    #[serde(rename_all = "camelCase")]
	Announced {
	    announcement: Cid,
	},
    #[serde(rename_all = "camelCase")]
	AnnouncementRemoved {
	    announcement: Cid,
	},
    #[serde(rename_all = "camelCase")]
	MembersInitialized {
	    #[bincode(with_serde)]
	    fellows: Vec<AccountId32>,
	    #[bincode(with_serde)]
	    allies: Vec<AccountId32>,
	},
    #[serde(rename_all = "camelCase")]
	NewAllyJoined {
	    #[bincode(with_serde)]
		ally: AccountId32,
		#[bincode(with_serde)]
		nominator: Option<AccountId32>,
		reserved: Option<u128>,
	},
    #[serde(rename_all = "camelCase")]
	AllyElevated {
	    #[bincode(with_serde)]
	    ally: AccountId32,
	},
    #[serde(rename_all = "camelCase")]
	MemberRetirementPeriodStarted {
	    #[bincode(with_serde)]
	    member: AccountId32,
	},
    #[serde(rename_all = "camelCase")]
	MemberRetired {
	    #[bincode(with_serde)]
	    member: AccountId32,
	    unreserved: Option<u128>,
	},
    #[serde(rename_all = "camelCase")]
	MemberKicked {
	    #[bincode(with_serde)]
	    member: AccountId32,
	    slashed: Option<u128>,
	},
    #[serde(rename_all = "camelCase")]
	UnscrupulousItemAdded {
	    items: Vec<UnscrupulousItemOf>,
	},
    #[serde(rename_all = "camelCase")]
	UnscrupulousItemRemoved {
	    items: Vec<UnscrupulousItemOf>,
	},
    #[serde(rename_all = "camelCase")]
	AllianceDisbanded {
	    fellow_members: u32,
	    ally_members: u32,
	    unreserved: u32,
	},
    #[serde(rename_all = "camelCase")]
	FellowAbdicated {
	    #[bincode(with_serde)]
	    fellow: AccountId32,
	},
}

