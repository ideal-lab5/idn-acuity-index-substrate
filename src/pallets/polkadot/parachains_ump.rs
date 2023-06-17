use crate::shared::*;
use crate::substrate::*;
use subxt::PolkadotConfig;

pub fn parachains_ump_index_event(
    indexer: &Indexer,
    block_number: u32,
    event_index: u32,
    event: subxt::events::EventDetails<PolkadotConfig>,
) -> Result<(), subxt::Error> {
    match event.variant_name() {
        "InvalidFormat" => {
            let event = event
                .as_event::<polkadot::ump::events::InvalidFormat>()?
                .unwrap();
            indexer.index_event_message_id(event.0, block_number, event_index);
            Ok(())
        }
        "UnsupportedVersion" => {
            let event = event
                .as_event::<polkadot::ump::events::UnsupportedVersion>()?
                .unwrap();
            indexer.index_event_message_id(event.0, block_number, event_index);
            Ok(())
        }
        "ExecutedUpward" => {
            let event = event
                .as_event::<polkadot::ump::events::ExecutedUpward>()?
                .unwrap();
            indexer.index_event_message_id(event.0, block_number, event_index);
            Ok(())
        }
        "WeightExhausted" => {
            let event = event
                .as_event::<polkadot::ump::events::WeightExhausted>()?
                .unwrap();
            indexer.index_event_message_id(event.0, block_number, event_index);
            Ok(())
        }
        "UpwardMessagesReceived" => {
            let event = event
                .as_event::<polkadot::ump::events::UpwardMessagesReceived>()?
                .unwrap();
            indexer.index_event_para_id(event.0 .0, block_number, event_index);
            Ok(())
        }
        "OverweightEnqueued" => {
            let event = event
                .as_event::<polkadot::ump::events::OverweightEnqueued>()?
                .unwrap();
            indexer.index_event_para_id(event.0 .0, block_number, event_index);
            indexer.index_event_message_id(event.1, block_number, event_index);
            Ok(())
        }
        _ => Ok(()),
    }
}
