use subxt::PolkadotConfig;
use crate::shared::*;
use crate::substrate::*;

pub fn proxy_index_event(indexer: &Indexer, block_number: u32, event_index: u32, event: subxt::events::EventDetails<PolkadotConfig>) -> Result<(), subxt::Error> {
    match event.variant_name() {
        "PureCreated" => {
            let event = event.as_event::<polkadot::proxy::events::PureCreated>()?.unwrap();
            indexer.index_event_account_id(event.pure, block_number, event_index);
            indexer.index_event_account_id(event.who, block_number, event_index);
            Ok(())
        },
        "Announced" => {
            let event = event.as_event::<polkadot::proxy::events::Announced>()?.unwrap();
            indexer.index_event_account_id(event.real, block_number, event_index);
            indexer.index_event_account_id(event.proxy, block_number, event_index);
            Ok(())
        },
        "ProxyAdded" => {
            let event = event.as_event::<polkadot::proxy::events::ProxyAdded>()?.unwrap();
            indexer.index_event_account_id(event.delegator, block_number, event_index);
            indexer.index_event_account_id(event.delegatee, block_number, event_index);
            Ok(())
        },
        "ProxyRemoved" => {
            let event = event.as_event::<polkadot::proxy::events::ProxyRemoved>()?.unwrap();
            indexer.index_event_account_id(event.delegator, block_number, event_index);
            indexer.index_event_account_id(event.delegatee, block_number, event_index);
            Ok(())
        },
        _ => Ok(()),
    }
}
