use crate::shared::*;
use crate::substrate::*;

pub fn nomination_pools_index_event(trees: Trees, block_number: u32, event_index: u32, event: subxt::events::EventDetails) -> Result<(), subxt::Error> {
    match event.variant_name() {
        "Created" => {
            let event = event.as_event::<polkadot::nomination_pools::events::Created>()?.unwrap();
            index_event_account_id(trees.clone(), event.depositor, block_number, event_index);
            index_event_pool_id(trees.clone(), event.pool_id, block_number, event_index);
            Ok(())
        },
        "Bonded" => {
            let event = event.as_event::<polkadot::nomination_pools::events::Bonded>()?.unwrap();
            index_event_account_id(trees.clone(), event.member, block_number, event_index);
            index_event_pool_id(trees.clone(), event.pool_id, block_number, event_index);
            Ok(())
        },
        "PaidOut" => {
            let event = event.as_event::<polkadot::nomination_pools::events::PaidOut>()?.unwrap();
            index_event_account_id(trees.clone(), event.member, block_number, event_index);
            index_event_pool_id(trees.clone(), event.pool_id, block_number, event_index);
            Ok(())
        },
        "Unbonded" => {
            let event = event.as_event::<polkadot::nomination_pools::events::Unbonded>()?.unwrap();
            index_event_account_id(trees.clone(), event.member, block_number, event_index);
            index_event_pool_id(trees.clone(), event.pool_id, block_number, event_index);
            Ok(())
        },
        "Withdrawn" => {
            let event = event.as_event::<polkadot::nomination_pools::events::Withdrawn>()?.unwrap();
            index_event_account_id(trees.clone(), event.member, block_number, event_index);
            index_event_pool_id(trees.clone(), event.pool_id, block_number, event_index);
            Ok(())
        },
        "Destroyed" => {
            let event = event.as_event::<polkadot::nomination_pools::events::Destroyed>()?.unwrap();
            index_event_pool_id(trees.clone(), event.pool_id, block_number, event_index);
            Ok(())
        },
        "StateChanged" => {
            let event = event.as_event::<polkadot::nomination_pools::events::StateChanged>()?.unwrap();
            index_event_pool_id(trees.clone(), event.pool_id, block_number, event_index);
            Ok(())
        },
        "MemberRemoved" => {
            let event = event.as_event::<polkadot::nomination_pools::events::MemberRemoved>()?.unwrap();
            index_event_pool_id(trees.clone(), event.pool_id, block_number, event_index);
            index_event_account_id(trees.clone(), event.member, block_number, event_index);
            Ok(())
        },
        "RolesUpdated" => {
            let event = event.as_event::<polkadot::nomination_pools::events::RolesUpdated>()?.unwrap();
            match event.root {
                Some(account) => index_event_account_id(trees.clone(), account, block_number, event_index),
                None => {},
            }
            match event.state_toggler {
                Some(account) => index_event_account_id(trees.clone(), account, block_number, event_index),
                None => {},
            }
            match event.nominator {
                Some(account) => index_event_account_id(trees.clone(), account, block_number, event_index),
                None => {},
            }
            Ok(())
        },
        "PoolSlashed" => {
            let event = event.as_event::<polkadot::nomination_pools::events::PoolSlashed>()?.unwrap();
            index_event_pool_id(trees.clone(), event.pool_id, block_number, event_index);
            Ok(())
        },
        "UnbondingPoolSlashed" => {
            let event = event.as_event::<polkadot::nomination_pools::events::UnbondingPoolSlashed>()?.unwrap();
            index_event_pool_id(trees.clone(), event.pool_id, block_number, event_index);
            Ok(())
        },
/*
        "PoolCommissionUpdated" => {
            let event = event.as_event::<polkadot::nomination_pools::events::PoolCommissionUpdated>()?.unwrap();
            index_event_pool_id(trees.clone(), event.pool_id, block_number, event_index);
            match event.current {
                Some((i, account)) => index_event_account_id(trees.clone(), account, block_number, event_index),
                None => {},
            }
        },
        "PoolCommissionChangeRateUpdated" => {
            let event = event.as_event::<polkadot::nomination_pools::events::PoolCommissionChangeRateUpdated>()?.unwrap();
            index_event_pool_id(trees.clone(), event.pool_id, block_number, event_index);
        },
        "PoolCommissionClaimed" => {
            let event = event.as_event::<polkadot::nomination_pools::events::PoolCommissionClaimed>()?.unwrap();
            index_event_pool_id(trees.clone(), event.pool_id, block_number, event_index);
        },
*/
        _ => Ok(()),
    }
}
