use subxt::PolkadotConfig;
use crate::shared::*;
use crate::substrate::*;

pub fn nomination_pools_index_event(indexer: &Indexer, block_number: u32, event_index: u32, event: subxt::events::EventDetails<PolkadotConfig>) -> Result<(), subxt::Error> {
    match event.variant_name() {
        "Created" => {
            let event = event.as_event::<polkadot::nomination_pools::events::Created>()?.unwrap();
            indexer.index_event_account_id(event.depositor, block_number, event_index);
            indexer.index_event_pool_id(event.pool_id, block_number, event_index);
            Ok(())
        },
        "Bonded" => {
            let event = event.as_event::<polkadot::nomination_pools::events::Bonded>()?.unwrap();
            indexer.index_event_account_id(event.member, block_number, event_index);
            indexer.index_event_pool_id(event.pool_id, block_number, event_index);
            Ok(())
        },
        "PaidOut" => {
            let event = event.as_event::<polkadot::nomination_pools::events::PaidOut>()?.unwrap();
            indexer.index_event_account_id(event.member, block_number, event_index);
            indexer.index_event_pool_id(event.pool_id, block_number, event_index);
            Ok(())
        },
        "Unbonded" => {
            let event = event.as_event::<polkadot::nomination_pools::events::Unbonded>()?.unwrap();
            indexer.index_event_account_id(event.member, block_number, event_index);
            indexer.index_event_pool_id(event.pool_id, block_number, event_index);
            indexer.index_event_era_index(event.era, block_number, event_index);
            Ok(())
        },
        "Withdrawn" => {
            let event = event.as_event::<polkadot::nomination_pools::events::Withdrawn>()?.unwrap();
            indexer.index_event_account_id(event.member, block_number, event_index);
            indexer.index_event_pool_id(event.pool_id, block_number, event_index);
            Ok(())
        },
        "Destroyed" => {
            let event = event.as_event::<polkadot::nomination_pools::events::Destroyed>()?.unwrap();
            indexer.index_event_pool_id(event.pool_id, block_number, event_index);
            Ok(())
        },
        "StateChanged" => {
            let event = event.as_event::<polkadot::nomination_pools::events::StateChanged>()?.unwrap();
            indexer.index_event_pool_id(event.pool_id, block_number, event_index);
            Ok(())
        },
        "MemberRemoved" => {
            let event = event.as_event::<polkadot::nomination_pools::events::MemberRemoved>()?.unwrap();
            indexer.index_event_pool_id(event.pool_id, block_number, event_index);
            indexer.index_event_account_id(event.member, block_number, event_index);
            Ok(())
        },
        "RolesUpdated" => {
            let event = event.as_event::<polkadot::nomination_pools::events::RolesUpdated>()?.unwrap();
            if let Some(account) = event.root {
                indexer.index_event_account_id(account, block_number, event_index);
            }
            if let Some(account) = event.state_toggler {
                indexer.index_event_account_id(account, block_number, event_index);
            }
            if let Some(account) = event.nominator {
                indexer.index_event_account_id(account, block_number, event_index);
            }
            Ok(())
        },
        "PoolSlashed" => {
            let event = event.as_event::<polkadot::nomination_pools::events::PoolSlashed>()?.unwrap();
            indexer.index_event_pool_id(event.pool_id, block_number, event_index);
            Ok(())
        },
        "UnbondingPoolSlashed" => {
            let event = event.as_event::<polkadot::nomination_pools::events::UnbondingPoolSlashed>()?.unwrap();
            indexer.index_event_pool_id(event.pool_id, block_number, event_index);
            indexer.index_event_era_index(event.era, block_number, event_index);
            Ok(())
        },
/*
        "PoolCommissionUpdated" => {
            let event = event.as_event::<polkadot::nomination_pools::events::PoolCommissionUpdated>()?.unwrap();
            indexer.index_event_pool_id(event.pool_id, block_number, event_index);
            match event.current {
                Some((i, account)) => indexer.index_event_account_id(account, block_number, event_index),
                None => {},
            }
        },
        "PoolCommissionChangeRateUpdated" => {
            let event = event.as_event::<polkadot::nomination_pools::events::PoolCommissionChangeRateUpdated>()?.unwrap();
            indexer.index_event_pool_id(event.pool_id, block_number, event_index);
        },
        "PoolCommissionClaimed" => {
            let event = event.as_event::<polkadot::nomination_pools::events::PoolCommissionClaimed>()?.unwrap();
            indexer.index_event_pool_id(event.pool_id, block_number, event_index);
        },
        "PoolCommissionClaimed" => {
            let event = event.as_event::<polkadot::nomination_pools::events::PoolCommissionClaimed>()?.unwrap();
            indexer.index_event_pool_id(event.pool_id, block_number, event_index, &value);
        },
*/
        _ => Ok(()),
    }
}
