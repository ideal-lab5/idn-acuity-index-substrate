use crate::shared::*;
use crate::substrate::*;
use subxt::PolkadotConfig;

pub fn parachains_hrmp_index_event(
    indexer: &Indexer,
    block_number: u32,
    event_index: u32,
    event: subxt::events::EventDetails<PolkadotConfig>,
) -> Result<(), subxt::Error> {
    match event.variant_name() {
        "OpenChannelRequested" => {
            let event = event
                .as_event::<polkadot::hrmp::events::OpenChannelRequested>()?
                .unwrap();
            indexer.index_event_para_id(event.0 .0, block_number, event_index);
            indexer.index_event_para_id(event.1 .0, block_number, event_index);
            Ok(())
        }
        "OpenChannelCanceled" => {
            let event = event
                .as_event::<polkadot::hrmp::events::OpenChannelCanceled>()?
                .unwrap();
            indexer.index_event_para_id(event.0 .0, block_number, event_index);
            Ok(())
        }
        "OpenChannelAccepted" => {
            let event = event
                .as_event::<polkadot::hrmp::events::OpenChannelAccepted>()?
                .unwrap();
            indexer.index_event_para_id(event.0 .0, block_number, event_index);
            indexer.index_event_para_id(event.1 .0, block_number, event_index);
            Ok(())
        }
        "ChannelClosed" => {
            let event = event
                .as_event::<polkadot::hrmp::events::ChannelClosed>()?
                .unwrap();
            indexer.index_event_para_id(event.0 .0, block_number, event_index);
            Ok(())
        }
        "HrmpChannelForceOpened" => {
            let event = event
                .as_event::<polkadot::hrmp::events::HrmpChannelForceOpened>()?
                .unwrap();
            indexer.index_event_para_id(event.0 .0, block_number, event_index);
            indexer.index_event_para_id(event.1 .0, block_number, event_index);
            Ok(())
        }
        _ => Ok(()),
    }
}
