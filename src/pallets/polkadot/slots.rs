use subxt::PolkadotConfig;
use crate::shared::*;
use crate::substrate::*;

pub fn slots_index_event(indexer: &Indexer, block_number: u32, event_index: u32, event: subxt::events::EventDetails<PolkadotConfig>) -> Result<(), subxt::Error> {
    match event.variant_name() {
        "Leased" => {
            let event = event.as_event::<polkadot::slots::events::Leased>()?.unwrap();
            indexer.index_event_para_id(event.para_id.0, block_number, event_index);
            indexer.index_event_account_id(event.leaser, block_number, event_index);
            Ok(())
        },
        _ => Ok(()),
    }
}
