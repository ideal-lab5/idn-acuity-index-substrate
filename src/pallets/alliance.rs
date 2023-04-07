use subxt::{
    utils::AccountId32,
};

use parity_scale_codec::{Encode, Decode};
use serde::{Serialize, Deserialize};

type Cid = Vec<u8>;
type UnscrupulousItemOf = Vec<u8>;

#[derive(Encode, Decode, Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "variant", content = "details")]
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
	    fellows: Vec<AccountId32>,
	    allies: Vec<AccountId32>,
	},
    #[serde(rename_all = "camelCase")]
	NewAllyJoined {
		ally: AccountId32,
		nominator: Option<AccountId32>,
		reserved: Option<u128>,
	},
    #[serde(rename_all = "camelCase")]
	AllyElevated {
	    ally: AccountId32,
	},
    #[serde(rename_all = "camelCase")]
	MemberRetirementPeriodStarted {
	    member: AccountId32,
	},
    #[serde(rename_all = "camelCase")]
	MemberRetired {
	    member: AccountId32,
	    unreserved: Option<u128>,
	},
    #[serde(rename_all = "camelCase")]
	MemberKicked {
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
	    fellow: AccountId32,
	},
}

