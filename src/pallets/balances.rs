use crate::shared::*;
use crate::substrate::*;
use subxt::PolkadotConfig;

pub fn balance_index_event(
    indexer: &Indexer,
    block_number: u32,
    event_index: u32,
    event: subxt::events::EventDetails<PolkadotConfig>,
) -> Result<(), subxt::Error> {
    match event.variant_name() {
        "Endowed" => {
            let endowed_event = event
                .as_event::<polkadot::balances::events::Endowed>()?
                .unwrap();
            indexer.index_event_account_id(endowed_event.account, block_number, event_index);
            Ok(())
        }
        "DustLost" => {
            let dustlost_event = event
                .as_event::<polkadot::balances::events::DustLost>()?
                .unwrap();
            indexer.index_event_account_id(dustlost_event.account, block_number, event_index);
            Ok(())
        }
        "Transfer" => {
            let transfer_event = event
                .as_event::<polkadot::balances::events::Transfer>()?
                .unwrap();
            indexer.index_event_account_id(transfer_event.from, block_number, event_index);
            indexer.index_event_account_id(transfer_event.to, block_number, event_index);
            Ok(())
        }
        "BalanceSet" => {
            let balance_set_event = event
                .as_event::<polkadot::balances::events::BalanceSet>()?
                .unwrap();
            indexer.index_event_account_id(balance_set_event.who, block_number, event_index);
            Ok(())
        }
        "Reserved" => {
            let reserved_event = event
                .as_event::<polkadot::balances::events::Reserved>()?
                .unwrap();
            indexer.index_event_account_id(reserved_event.who, block_number, event_index);
            Ok(())
        }
        "Unreserved" => {
            let unreserved_event = event
                .as_event::<polkadot::balances::events::Unreserved>()?
                .unwrap();
            indexer.index_event_account_id(unreserved_event.who, block_number, event_index);
            Ok(())
        }
        "ReserveRepatriated" => {
            let reserve_repatriated_event = event
                .as_event::<polkadot::balances::events::ReserveRepatriated>()?
                .unwrap();
            indexer.index_event_account_id(
                reserve_repatriated_event.from,
                block_number,
                event_index,
            );
            indexer.index_event_account_id(reserve_repatriated_event.to, block_number, event_index);
            Ok(())
        }
        "Deposit" => {
            let deposit_event = event
                .as_event::<polkadot::balances::events::Deposit>()?
                .unwrap();
            indexer.index_event_account_id(deposit_event.who, block_number, event_index);
            Ok(())
        }
        "Withdraw" => {
            let withdraw_event = event
                .as_event::<polkadot::balances::events::Withdraw>()?
                .unwrap();
            indexer.index_event_account_id(withdraw_event.who, block_number, event_index);
            Ok(())
        }
        "Slashed" => {
            let slashed_event = event
                .as_event::<polkadot::balances::events::Slashed>()?
                .unwrap();
            indexer.index_event_account_id(slashed_event.who, block_number, event_index);
            Ok(())
        }
        _ => Ok(()),
    }
}
