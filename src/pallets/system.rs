use crate::shared::*;
use crate::substrate::*;
use subxt::PolkadotConfig;

pub fn system_index_event(
    indexer: &Indexer,
    block_number: u32,
    event_index: u32,
    event: subxt::events::EventDetails<PolkadotConfig>,
) -> Result<(), subxt::Error> {
    match event.variant_name() {
        "NewAccount" => {
            let event = event
                .as_event::<polkadot::system::events::NewAccount>()?
                .unwrap();
            indexer.index_event_account_id(event.account, block_number, event_index);
            Ok(())
        }
        "KilledAccount" => {
            let event = event
                .as_event::<polkadot::system::events::KilledAccount>()?
                .unwrap();
            indexer.index_event_account_id(event.account, block_number, event_index);
            Ok(())
        }
        "Remarked" => {
            let event = event
                .as_event::<polkadot::system::events::Remarked>()?
                .unwrap();
            indexer.index_event_account_id(event.sender, block_number, event_index);
            Ok(())
        }
        _ => Ok(()),
    }
}
