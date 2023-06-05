use subxt::PolkadotConfig;
use crate::shared::*;
use crate::substrate::*;

pub fn pallet_xcm_index_event(indexer: &Indexer, block_number: u32, event_index: u32, event: subxt::events::EventDetails<PolkadotConfig>) -> Result<(), subxt::Error> {
    match event.variant_name() {
        "Claimed" => {
            let event = event.as_event::<polkadot::pallet_xcm::events::Claimed>()?.unwrap();
            indexer.index_event_account_id(event.who, block_number, event_index);
            Ok(())
        },
        _ => Ok(()),
    }
}
