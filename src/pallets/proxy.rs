use subxt::{
    utils::AccountId32,
};

use parity_scale_codec::{Encode, Decode};
use serde::Serialize;

use crate::shared::*;
use crate::substrate::*;

#[derive(Encode, Decode, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[allow(clippy::unnecessary_cast)]
pub enum ProxyType {
    Any = 0,
    NonTransfer = 1,
    Governance = 2,
    Staking = 3,
    // Skip 4 as it is now removed (was SudoBalances)
    IdentityJudgement = 5,
    CancelProxy = 6,
    Auction = 7,
}

use crate::shared::polkadot::runtime_types::polkadot_runtime::ProxyType as SubProxyType;

impl From<SubProxyType> for ProxyType {
    fn from(x: SubProxyType) -> Self {
        match x {
            SubProxyType::Any => ProxyType::Any,
            SubProxyType::NonTransfer => ProxyType::NonTransfer,
            SubProxyType::Governance => ProxyType::Governance,
            SubProxyType::Staking => ProxyType::Staking,
            SubProxyType::IdentityJudgement => ProxyType::IdentityJudgement,
            SubProxyType::CancelProxy => ProxyType::CancelProxy,
            SubProxyType::Auction => ProxyType::Auction,
        }
    }
}

#[derive(Encode, Decode, Serialize, Debug, Clone)]
#[serde(tag = "variant", content = "details")]
pub enum Proxy {
    #[serde(rename_all = "camelCase")]
    PureCreated {
        pure: AccountId32,
        who: AccountId32,
        proxy_type: ProxyType,
        disambiguation_index: u16,
    },
    #[serde(rename_all = "camelCase")]
    Announced {
        real: AccountId32,
        proxy: AccountId32,
        call_hash: [u8; 32],
    },
    #[serde(rename_all = "camelCase")]
    ProxyAdded {
        delegator: AccountId32,
        delegatee: AccountId32,
        proxy_type: ProxyType,
        delay: u32,
    },
    #[serde(rename_all = "camelCase")]
    ProxyRemoved {
        delegator: AccountId32,
        delegatee: AccountId32,
        proxy_type: ProxyType,
        delay: u32,
    },
}

pub fn proxy_index_event(trees: Trees, block_number: u32, event_index: u32, event: subxt::events::EventDetails) -> Result<(), subxt::Error> {
    match event.variant_name() {
        "PureCreated" => {
            let event = event.as_event::<polkadot::proxy::events::PureCreated>()?.unwrap();
            let event_db = Event::Proxy(
                Proxy::PureCreated {
                    pure: event.pure.clone(),
                    who: event.who.clone(),
                    proxy_type: event.proxy_type.into(),
                    disambiguation_index: event.disambiguation_index,
                }
            );
            let value = Event::encode(&event_db);
            index_event_account_id(trees.clone(), event.pure, block_number, event_index, &value);
            index_event_account_id(trees, event.who, block_number, event_index, &value);
            Ok(())
        },
        "Announced" => {
            let event = event.as_event::<polkadot::proxy::events::Announced>()?.unwrap();
            let event_db = Event::Proxy(
                Proxy::Announced {
                    real: event.real.clone(),
                    proxy: event.proxy.clone(),
                    call_hash: event.call_hash.into(),
                }
            );
            let value = Event::encode(&event_db);
            index_event_account_id(trees.clone(), event.real, block_number, event_index, &value);
            index_event_account_id(trees, event.proxy, block_number, event_index, &value);
            Ok(())
        },
        "ProxyAdded" => {
            let event = event.as_event::<polkadot::proxy::events::ProxyAdded>()?.unwrap();
            let event_db = Event::Proxy(
                Proxy::ProxyAdded {
                    delegator: event.delegator.clone(),
                    delegatee: event.delegatee.clone(),
                    proxy_type: event.proxy_type.into(),
                    delay: event.delay,
                }
            );
            let value = Event::encode(&event_db);
            index_event_account_id(trees.clone(), event.delegator, block_number, event_index, &value);
            index_event_account_id(trees, event.delegatee, block_number, event_index, &value);
            Ok(())
        },
        "ProxyRemoved" => {
            let event = event.as_event::<polkadot::proxy::events::ProxyRemoved>()?.unwrap();
            let event_db = Event::Proxy(
                Proxy::ProxyRemoved {
                    delegator: event.delegator.clone(),
                    delegatee: event.delegatee.clone(),
                    proxy_type: event.proxy_type.into(),
                    delay: event.delay,
                }
            );
            let value = Event::encode(&event_db);
            index_event_account_id(trees.clone(), event.delegator, block_number, event_index, &value);
            index_event_account_id(trees, event.delegatee, block_number, event_index, &value);
            Ok(())
        },
        _ => Ok(()),
    }
}
