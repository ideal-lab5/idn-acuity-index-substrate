use crate::shared::*;
use crate::substrate::*;

pub fn paras_registrar_index_event(trees: Trees, block_number: u32, event_index: u32, event: subxt::events::EventDetails) -> Result<(), subxt::Error> {
    match event.variant_name() {
        "Registered" => {
            let event = event.as_event::<polkadot::registrar::events::Registered>()?.unwrap();
            index_event_para_id(trees.clone(), event.para_id.0, block_number, event_index);
            index_event_account_id(trees, event.manager, block_number, event_index);
            Ok(())
        },
        "Deregistered" => {
            let event = event.as_event::<polkadot::registrar::events::Deregistered>()?.unwrap();
            index_event_para_id(trees, event.para_id.0, block_number, event_index);
            Ok(())
        },
        "Reserved" => {
            let event = event.as_event::<polkadot::registrar::events::Reserved>()?.unwrap();
            index_event_para_id(trees.clone(), event.para_id.0, block_number, event_index);
            index_event_account_id(trees, event.who, block_number, event_index);
            Ok(())
        },
        _ => Ok(()),
    }
}
