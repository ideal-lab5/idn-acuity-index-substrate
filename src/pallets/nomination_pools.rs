use subxt::{
    utils::AccountId32,
};

use parity_scale_codec::{Encode, Decode};
use serde::{Serialize, Deserialize};

use crate::shared::*;
use crate::substrate::*;

#[derive(Encode, Decode, Serialize, Deserialize, Debug, Clone)]
pub enum PoolState {
	Open,
	Blocked,
	Destroying,
}

use crate::shared::polkadot::runtime_types::pallet_nomination_pools::PoolState as SubPoolState;

impl From<SubPoolState> for PoolState {
    fn from(x: SubPoolState) -> Self {
       match x {
            SubPoolState::Open => PoolState::Open,
            SubPoolState::Blocked => PoolState::Blocked,
            SubPoolState::Destroying => PoolState::Destroying,
        }
    }
}

#[derive(Encode, Decode, Serialize, Deserialize, Debug, Clone)]
pub struct CommissionChangeRate {
	pub max_increase: u32,
	pub min_delay: u32,
}
/*
use crate::shared::polkadot::runtime_types::pallet_nomination_pools::CommissionChangeRate as SubCommissionChangeRate;

impl From<SubCommissionChangeRate> for CommissionChangeRate {
    fn from(x: SubCommissionChangeRate) -> Self {
        CommissionChangeRate {
            max_increase: x.max_increase,
            min_delay: x.min_delay,
        }
    }
}
*/
#[derive(Encode, Decode, Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "variant", content = "details")]
pub enum NominationPools {
    #[serde(rename_all = "camelCase")]
	Created {
	    depositor: AccountId32,
	    pool_id: u32,
	},
    #[serde(rename_all = "camelCase")]
	Bonded {
	    member: AccountId32,
	    pool_id: u32,
	    bonded: u128,
	    joined: bool,
	},
    #[serde(rename_all = "camelCase")]
	PaidOut {
	    member: AccountId32,
	    pool_id: u32,
	    payout: u128,
	},
    #[serde(rename_all = "camelCase")]
	Unbonded {
		member: AccountId32,
		pool_id: u32,
		balance: u128,
		points: u128,
		era: u32,
	},
    #[serde(rename_all = "camelCase")]
	Withdrawn {
		member: AccountId32,
		pool_id: u32,
		balance: u128,
		points: u128,
	},
    #[serde(rename_all = "camelCase")]
	Destroyed {
	    pool_id: u32,
	},
    #[serde(rename_all = "camelCase")]
	StateChanged {
	    pool_id: u32,
	    new_state: PoolState,
	},
    #[serde(rename_all = "camelCase")]
	MemberRemoved {
	    pool_id: u32,
	    member: AccountId32,
	},
    #[serde(rename_all = "camelCase")]
	RolesUpdated {
		root: Option<AccountId32>,
		state_toggler: Option<AccountId32>,
		nominator: Option<AccountId32>,
	},
    #[serde(rename_all = "camelCase")]
	PoolSlashed {
	    pool_id: u32,
	    balance: u128,
	},
    #[serde(rename_all = "camelCase")]
	UnbondingPoolSlashed {
	    pool_id: u32,
	    era: u32,
	    balance: u128,
	},
    #[serde(rename_all = "camelCase")]
	PoolCommissionUpdated {
	    pool_id: u32,
	    current: Option<(u32, AccountId32)>,
	},
    #[serde(rename_all = "camelCase")]
	PoolMaxCommissionUpdated {
	    pool_id: u32,
	    max_commission: u32,
	},
    #[serde(rename_all = "camelCase")]
	PoolCommissionChangeRateUpdated {
		pool_id: u32,
		change_rate: CommissionChangeRate,
	},
    #[serde(rename_all = "camelCase")]
	PoolCommissionClaimed {
	    pool_id: u32,
	    commission: u128,
	},
}

pub fn nomination_pools_index_event(trees: Trees, block_number: u32, event_index: u32, event: subxt::events::EventDetails) -> Result<(), subxt::Error> {
    match event.variant_name() {
        "Created" => {
            let event = event.as_event::<polkadot::nomination_pools::events::Created>()?.unwrap();
            let event_db = Event::NominationPools(
                NominationPools::Created {
	                depositor: event.depositor.clone(),
	                pool_id: event.pool_id,
                }
            );
            let value = Event::encode(&event_db);
            index_event_account_id(trees.clone(), event.depositor, block_number, event_index, &value);
            index_event_pool_id(trees.clone(), event.pool_id, block_number, event_index, &value);
            Ok(())
        },
        "Bonded" => {
            let event = event.as_event::<polkadot::nomination_pools::events::Bonded>()?.unwrap();
            let event_db = Event::NominationPools(
                NominationPools::Bonded {
	                member: event.member.clone(),
	                pool_id: event.pool_id,
	                bonded: event.bonded,
	                joined: event.joined,
                }
            );
            let value = Event::encode(&event_db);
            index_event_account_id(trees.clone(), event.member, block_number, event_index, &value);
            index_event_pool_id(trees.clone(), event.pool_id, block_number, event_index, &value);
            Ok(())
        },
        "PaidOut" => {
            let event = event.as_event::<polkadot::nomination_pools::events::PaidOut>()?.unwrap();
            let event_db = Event::NominationPools(
                NominationPools::PaidOut {
	                member: event.member.clone(),
	                pool_id: event.pool_id,
	                payout: event.payout,
                }
            );
            let value = Event::encode(&event_db);
            index_event_account_id(trees.clone(), event.member, block_number, event_index, &value);
            index_event_pool_id(trees.clone(), event.pool_id, block_number, event_index, &value);
            Ok(())
        },
        "Unbonded" => {
            let event = event.as_event::<polkadot::nomination_pools::events::Unbonded>()?.unwrap();
            let event_db = Event::NominationPools(
                NominationPools::Unbonded {
	                member: event.member.clone(),
	                pool_id: event.pool_id,
		            balance: event.balance,
		            points: event.points,
		            era: event.era,
                }
            );
            let value = Event::encode(&event_db);
            index_event_account_id(trees.clone(), event.member, block_number, event_index, &value);
            index_event_pool_id(trees.clone(), event.pool_id, block_number, event_index, &value);
            Ok(())
        },
        "Withdrawn" => {
            let event = event.as_event::<polkadot::nomination_pools::events::Withdrawn>()?.unwrap();
            let event_db = Event::NominationPools(
                NominationPools::Withdrawn {
	                member: event.member.clone(),
	                pool_id: event.pool_id,
		            balance: event.balance,
		            points: event.points,
                }
            );
            let value = Event::encode(&event_db);
            index_event_account_id(trees.clone(), event.member, block_number, event_index, &value);
            index_event_pool_id(trees.clone(), event.pool_id, block_number, event_index, &value);
            Ok(())
        },
        "Destroyed" => {
            let event = event.as_event::<polkadot::nomination_pools::events::Destroyed>()?.unwrap();
            let event_db = Event::NominationPools(
                NominationPools::Destroyed {
	                pool_id: event.pool_id,
                }
            );
            let value = Event::encode(&event_db);
            index_event_pool_id(trees.clone(), event.pool_id, block_number, event_index, &value);
            Ok(())
        },
        "StateChanged" => {
            let event = event.as_event::<polkadot::nomination_pools::events::StateChanged>()?.unwrap();
            let event_db = Event::NominationPools(
                NominationPools::StateChanged {
	                pool_id: event.pool_id,
	                new_state: event.new_state.into(),
                }
            );
            let value = Event::encode(&event_db);
            index_event_pool_id(trees.clone(), event.pool_id, block_number, event_index, &value);
            Ok(())
        },
        "MemberRemoved" => {
            let event = event.as_event::<polkadot::nomination_pools::events::MemberRemoved>()?.unwrap();
            let event_db = Event::NominationPools(
                NominationPools::MemberRemoved {
	                pool_id: event.pool_id,
               	    member: event.member.clone(),
                }
            );
            let value = Event::encode(&event_db);
            index_event_pool_id(trees.clone(), event.pool_id, block_number, event_index, &value);
            index_event_account_id(trees.clone(), event.member, block_number, event_index, &value);
            Ok(())
        },
        "RolesUpdated" => {
            let event = event.as_event::<polkadot::nomination_pools::events::RolesUpdated>()?.unwrap();
            let event_db = Event::NominationPools(
                NominationPools::RolesUpdated {
		            root: event.root.clone(),
		            state_toggler: event.state_toggler.clone(),
		            nominator: event.nominator.clone(),
                }
            );
            let value = Event::encode(&event_db);
            match event.root {
                Some(account) => index_event_account_id(trees.clone(), account, block_number, event_index, &value),
                None => {},
            }
            match event.state_toggler {
                Some(account) => index_event_account_id(trees.clone(), account, block_number, event_index, &value),
                None => {},
            }
            match event.nominator {
                Some(account) => index_event_account_id(trees.clone(), account, block_number, event_index, &value),
                None => {},
            }
            Ok(())
        },
        "PoolSlashed" => {
            let event = event.as_event::<polkadot::nomination_pools::events::PoolSlashed>()?.unwrap();
            let event_db = Event::NominationPools(
                NominationPools::PoolSlashed {
	                pool_id: event.pool_id,
	                balance: event.balance,
                }
            );
            let value = Event::encode(&event_db);
            index_event_pool_id(trees.clone(), event.pool_id, block_number, event_index, &value);
            Ok(())
        },
        "UnbondingPoolSlashed" => {
            let event = event.as_event::<polkadot::nomination_pools::events::UnbondingPoolSlashed>()?.unwrap();
            let event_db = Event::NominationPools(
                NominationPools::UnbondingPoolSlashed {
	                pool_id: event.pool_id,
	                era: event.era,
	                balance: event.balance,
                }
            );
            let value = Event::encode(&event_db);
            index_event_pool_id(trees.clone(), event.pool_id, block_number, event_index, &value);
            Ok(())
        },
/*
        "PoolCommissionUpdated" => {
            let event = event.as_event::<polkadot::nomination_pools::events::PoolCommissionUpdated>()?.unwrap();
            let event_db = Event::NominationPools(
                NominationPools::PoolCommissionUpdated {
	                pool_id: event.pool_id,
	                current: event.current.clone(),
                }
            );
            let value = Event::encode(&event_db);
            index_event_pool_id(trees.clone(), event.pool_id, block_number, event_index, &value);
            match event.current {
                Some((i, account)) => index_event_account_id(trees.clone(), account, block_number, event_index, &value),
                None => {},
            }
        },
        "PoolCommissionChangeRateUpdated" => {
            let event = event.as_event::<polkadot::nomination_pools::events::PoolCommissionChangeRateUpdated>()?.unwrap();
            let event_db = Event::NominationPools(
                NominationPools::PoolCommissionChangeRateUpdated {
	                pool_id: event.pool_id,
	                change_rate: event.change_rate,
                }
            );
            let value = Event::encode(&event_db);
            index_event_pool_id(trees.clone(), event.pool_id, block_number, event_index, &value);
        },
        "PoolCommissionClaimed" => {
            let event = event.as_event::<polkadot::nomination_pools::events::PoolCommissionClaimed>()?.unwrap();
            let event_db = Event::NominationPools(
                NominationPools::PoolCommissionClaimed {
	                pool_id: event.pool_id,
	                commission: event.commission,
                }
            );
            let value = Event::encode(&event_db);
            index_event_pool_id(trees.clone(), event.pool_id, block_number, event_index, &value);
        },
*/
        _ => Ok(()),
    }
}
