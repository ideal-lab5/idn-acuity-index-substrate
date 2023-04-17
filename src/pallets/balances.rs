use crate::shared::*;
use crate::substrate::*;

pub fn balance_index_event(trees: Trees, block_number: u32, event_index: u32, event: subxt::events::EventDetails) -> Result<(), subxt::Error> {
    match event.variant_name() {
        "Endowed" => {
            let endowed_event = event.as_event::<polkadot::balances::events::Endowed>()?.unwrap();
            index_event_account_id(trees.clone(), endowed_event.account, block_number, event_index);
            Ok(())
        },
        "DustLost" => {
            let dustlost_event = event.as_event::<polkadot::balances::events::DustLost>()?.unwrap();
            index_event_account_id(trees.clone(), dustlost_event.account, block_number, event_index);
            Ok(())
        },
        "Transfer" => {
            let transfer_event = event.as_event::<polkadot::balances::events::Transfer>()?.unwrap();
            index_event_account_id(trees.clone(), transfer_event.from, block_number, event_index);
            index_event_account_id(trees.clone(), transfer_event.to, block_number, event_index);
            Ok(())
        },
        "BalanceSet" => {
            let balance_set_event = event.as_event::<polkadot::balances::events::BalanceSet>()?.unwrap();
            index_event_account_id(trees.clone(), balance_set_event.who, block_number, event_index);
            Ok(())
        },
        "Reserved" => {
            let reserved_event = event.as_event::<polkadot::balances::events::Reserved>()?.unwrap();
            index_event_account_id(trees.clone(), reserved_event.who, block_number, event_index);
            Ok(())
        },
        "Unreserved" => {
            let unreserved_event = event.as_event::<polkadot::balances::events::Unreserved>()?.unwrap();
            index_event_account_id(trees.clone(), unreserved_event.who, block_number, event_index);
            Ok(())
        },
        "ReserveRepatriated" => {
            let reserve_repatriated_event = event.as_event::<polkadot::balances::events::ReserveRepatriated>()?.unwrap();
            index_event_account_id(trees.clone(), reserve_repatriated_event.from, block_number, event_index);
            index_event_account_id(trees.clone(), reserve_repatriated_event.to, block_number, event_index);
            Ok(())
        },
        "Deposit" => {
            let deposit_event = event.as_event::<polkadot::balances::events::Deposit>()?.unwrap();
            index_event_account_id(trees.clone(), deposit_event.who, block_number, event_index);
            Ok(())
        },
        "Withdraw" => {
            let withdraw_event = event.as_event::<polkadot::balances::events::Withdraw>()?.unwrap();
            index_event_account_id(trees.clone(), withdraw_event.who, block_number, event_index);
            Ok(())
        },
        "Slashed" => {
            let slashed_event = event.as_event::<polkadot::balances::events::Slashed>()?.unwrap();
            index_event_account_id(trees.clone(), slashed_event.who, block_number, event_index);
            Ok(())
        },
        _ => Ok(()),
    }
}
