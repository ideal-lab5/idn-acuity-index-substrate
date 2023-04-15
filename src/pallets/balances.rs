use subxt::{
    utils::AccountId32,
};

use parity_scale_codec::{Encode, Decode};
use serde::{Serialize, Deserialize};

use crate::shared::*;
use crate::substrate::*;

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
#[serde(tag = "variant", content = "details")]
pub enum Balances {
    #[serde(rename_all = "camelCase")]
    Endowed {
        account: AccountId32,
        free_balance: u128,
    },
    #[serde(rename_all = "camelCase")]
    DustLost {
        account: AccountId32,
        amount: u128,
    },
    #[serde(rename_all = "camelCase")]
    Transfer {
        from: AccountId32,
        to: AccountId32,
        value: u128,
    },
    #[serde(rename_all = "camelCase")]
    BalanceSet {
        who: AccountId32,
        free: u128,
        reserved: u128,
    },
    #[serde(rename_all = "camelCase")]
    Reserved {
        who: AccountId32,
        amount: u128,
    },
    #[serde(rename_all = "camelCase")]
    Unreserved {
        who: AccountId32,
        amount: u128,
    },
    #[serde(rename_all = "camelCase")]
    ReserveRepatriated {
        from: AccountId32,
        to: AccountId32,
        amount: u128,
        destination_status: Status,
    },
    #[serde(rename_all = "camelCase")]
    Deposit {
        who: AccountId32,
        amount: u128,
    },
    Withdraw {
        who: AccountId32,
        amount: u128,
    },
    #[serde(rename_all = "camelCase")]
    Slashed {
        who: AccountId32,
        amount: u128,
    },
}

pub fn balance_index_event(trees: Trees, block_number: u32, event_index: u32, event: subxt::events::EventDetails) -> Result<(), subxt::Error> {
    match event.variant_name() {
        "Endowed" => {
            let endowed_event = event.as_event::<polkadot::balances::events::Endowed>()?.unwrap();
            let event = Event::Balances(
                Balances::Endowed {
                    account: endowed_event.account.clone(),
                    free_balance: endowed_event.free_balance.clone(),
                }
            );
            let value = Event::encode(&event);
            index_event_account_id(trees.clone(), endowed_event.account, block_number, event_index, &value);
            Ok(())
        },
        "DustLost" => {
            let dustlost_event = event.as_event::<polkadot::balances::events::DustLost>()?.unwrap();
            let event = Event::Balances(
                Balances::DustLost {
                    account: dustlost_event.account.clone(),
                    amount: dustlost_event.amount,
                }
            );
            let value = Event::encode(&event);
            index_event_account_id(trees.clone(), dustlost_event.account, block_number, event_index, &value);
            Ok(())
        },
        "Transfer" => {
            let transfer_event = event.as_event::<polkadot::balances::events::Transfer>()?.unwrap();
            let event = Event::Balances(
                Balances::Transfer {
                    from: transfer_event.from.clone(),
                    to: transfer_event.to.clone(),
                    value: transfer_event.amount,
                }
            );
            let value = Event::encode(&event);
            index_event_account_id(trees.clone(), transfer_event.from, block_number, event_index, &value);
            index_event_account_id(trees.clone(), transfer_event.to, block_number, event_index, &value);
            Ok(())
        },
        "BalanceSet" => {
            let balance_set_event = event.as_event::<polkadot::balances::events::BalanceSet>()?.unwrap();
            let event = Event::Balances(
                Balances::BalanceSet {
                    who: balance_set_event.who.clone(),
                    free: balance_set_event.free,
                    reserved: balance_set_event.reserved,
                }
            );
            let value = Event::encode(&event);
            index_event_account_id(trees.clone(), balance_set_event.who, block_number, event_index, &value);
            Ok(())
        },
        "Reserved" => {
            let reserved_event = event.as_event::<polkadot::balances::events::Reserved>()?.unwrap();
            let event = Event::Balances(
                Balances::Reserved {
                    who: reserved_event.who.clone(),
                    amount: reserved_event.amount,
                }
            );
            let value = Event::encode(&event);
            index_event_account_id(trees.clone(), reserved_event.who, block_number, event_index, &value);
            Ok(())
        },
        "Unreserved" => {
            let unreserved_event = event.as_event::<polkadot::balances::events::Unreserved>()?.unwrap();
            let event = Event::Balances(
                Balances::Unreserved {
                    who: unreserved_event.who.clone(),
                    amount: unreserved_event.amount,
                }
            );
            let value = Event::encode(&event);
            index_event_account_id(trees.clone(), unreserved_event.who, block_number, event_index, &value);
            Ok(())
        },
        "ReserveRepatriated" => {
            let reserve_repatriated_event = event.as_event::<polkadot::balances::events::ReserveRepatriated>()?.unwrap();
            let event = Event::Balances(
                Balances::ReserveRepatriated {
                    from: reserve_repatriated_event.from.clone(),
                    to: reserve_repatriated_event.to.clone(),
                    amount: reserve_repatriated_event.amount,
                    destination_status: Status::from(&reserve_repatriated_event.destination_status),
                }
            );
            let value = Event::encode(&event);
            index_event_account_id(trees.clone(), reserve_repatriated_event.from, block_number, event_index, &value);
            index_event_account_id(trees.clone(), reserve_repatriated_event.to, block_number, event_index, &value);
            Ok(())
        },
        "Deposit" => {
            let deposit_event = event.as_event::<polkadot::balances::events::Deposit>()?.unwrap();
            let event = Event::Balances(
                Balances::Deposit {
                    who: deposit_event.who.clone(),
                    amount: deposit_event.amount,
                }
            );
            let value = Event::encode(&event);
            index_event_account_id(trees.clone(), deposit_event.who, block_number, event_index, &value);
            Ok(())
        },
        "Withdraw" => {
            let withdraw_event = event.as_event::<polkadot::balances::events::Withdraw>()?.unwrap();
            let event = Event::Balances(
                Balances::Withdraw {
                    who: withdraw_event.who.clone(),
                    amount: withdraw_event.amount,
                }
            );
            let value = Event::encode(&event);
            index_event_account_id(trees.clone(), withdraw_event.who, block_number, event_index, &value);
            Ok(())
        },
        "Slashed" => {
            let slashed_event = event.as_event::<polkadot::balances::events::Slashed>()?.unwrap();
            let event = Event::Balances(
                Balances::Slashed {
                    who: slashed_event.who.clone(),
                    amount: slashed_event.amount,
                }
            );
            let value = Event::encode(&event);
            index_event_account_id(trees.clone(), slashed_event.who, block_number, event_index, &value);
            Ok(())
        },
        _ => Ok(()),
    }
}
