use crate::shared::*;
use crate::substrate::*;
use subxt::PolkadotConfig;

pub fn parachains_disputes_index_event(
    indexer: &Indexer,
    block_number: u32,
    event_index: u32,
    event: subxt::events::EventDetails<PolkadotConfig>,
) -> Result<(), subxt::Error> {
    match event.variant_name() {
        "DisputeInitiated" => {
            let event = event
                .as_event::<polkadot::paras_disputes::events::DisputeInitiated>()?
                .unwrap();
            indexer.index_event_candidate_hash(event.0 .0 .0, block_number, event_index);
            Ok(())
        }
        "DisputeConcluded" => {
            let event = event
                .as_event::<polkadot::paras_disputes::events::DisputeConcluded>()?
                .unwrap();
            indexer.index_event_candidate_hash(event.0 .0 .0, block_number, event_index);
            Ok(())
        }
        "DisputeTimedOut" => {
            let event = event
                .as_event::<polkadot::paras_disputes::events::DisputeTimedOut>()?
                .unwrap();
            indexer.index_event_candidate_hash(event.0 .0 .0, block_number, event_index);
            Ok(())
        }
        _ => Ok(()),
    }
}
