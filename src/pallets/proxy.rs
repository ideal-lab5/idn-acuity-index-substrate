use crate::shared::*;
use crate::substrate::*;

pub fn proxy_index_event(trees: Trees, block_number: u32, event_index: u32, event: subxt::events::EventDetails) -> Result<(), subxt::Error> {
    match event.variant_name() {
        "PureCreated" => {
            let event = event.as_event::<polkadot::proxy::events::PureCreated>()?.unwrap();
            index_event_account_id(trees.clone(), event.pure, block_number, event_index);
            index_event_account_id(trees.clone(), event.who, block_number, event_index);
            Ok(())
        },
        "Announced" => {
            let event = event.as_event::<polkadot::proxy::events::Announced>()?.unwrap();
            index_event_account_id(trees.clone(), event.real, block_number, event_index);
            index_event_account_id(trees.clone(), event.proxy, block_number, event_index);
            Ok(())
        },
        "ProxyAdded" => {
            let event = event.as_event::<polkadot::proxy::events::ProxyAdded>()?.unwrap();
            index_event_account_id(trees.clone(), event.delegator, block_number, event_index);
            index_event_account_id(trees.clone(), event.delegatee, block_number, event_index);
            Ok(())
        },
        "ProxyRemoved" => {
            let event = event.as_event::<polkadot::proxy::events::ProxyRemoved>()?.unwrap();
            index_event_account_id(trees.clone(), event.delegator, block_number, event_index);
            index_event_account_id(trees.clone(), event.delegatee, block_number, event_index);
            Ok(())
        },
        _ => Ok(()),
    }
}
