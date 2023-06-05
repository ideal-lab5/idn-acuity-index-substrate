use subxt::PolkadotConfig;
use crate::shared::*;
use crate::substrate::*;

pub fn paras_registrar_index_event(indexer: &Indexer, block_number: u32, event_index: u32, event: subxt::events::EventDetails<PolkadotConfig>) -> Result<(), subxt::Error> {
    match event.variant_name() {
        "Registered" => {
            let event = event.as_event::<polkadot::registrar::events::Registered>()?.unwrap();
            indexer.index_event_para_id(event.para_id.0, block_number, event_index);
            indexer.index_event_account_id(event.manager, block_number, event_index);
            Ok(())
        },
        "Deregistered" => {
            let event = event.as_event::<polkadot::registrar::events::Deregistered>()?.unwrap();
            indexer.index_event_para_id(event.para_id.0, block_number, event_index);
            Ok(())
        },
        "Reserved" => {
            let event = event.as_event::<polkadot::registrar::events::Reserved>()?.unwrap();
            indexer.index_event_para_id(event.para_id.0, block_number, event_index);
            indexer.index_event_account_id(event.who, block_number, event_index);
            Ok(())
        },
        _ => Ok(()),
    }
}
