use crate::shared::*;
use crate::substrate::*;

pub fn parachains_disputes_index_event(trees: Trees, block_number: u32, event_index: u32, event: subxt::events::EventDetails) -> Result<(), subxt::Error> {
    match event.variant_name() {
        "DisputeInitiated" => {
            let event = event.as_event::<polkadot::paras_disputes::events::DisputeInitiated>()?.unwrap();
            index_event_candidate_hash(trees.clone(), event.0.0.0, block_number, event_index);
            Ok(())
        },
        "DisputeConcluded" => {
            let event = event.as_event::<polkadot::paras_disputes::events::DisputeConcluded>()?.unwrap();
            index_event_candidate_hash(trees.clone(), event.0.0.0, block_number, event_index);
            Ok(())
        },
        "DisputeTimedOut" => {
            let event = event.as_event::<polkadot::paras_disputes::events::DisputeTimedOut>()?.unwrap();
            index_event_candidate_hash(trees.clone(), event.0.0.0, block_number, event_index);
            Ok(())
        },
        _ => Ok(()),
    }
}
