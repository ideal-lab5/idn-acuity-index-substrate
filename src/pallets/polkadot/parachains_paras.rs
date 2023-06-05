use subxt::PolkadotConfig;
use crate::shared::*;
use crate::substrate::*;

pub fn parachains_paras_index_event(indexer: &Indexer, block_number: u32, event_index: u32, event: subxt::events::EventDetails<PolkadotConfig>) -> Result<(), subxt::Error> {
    match event.variant_name() {
        "CurrentCodeUpdated" => {
            let event = event.as_event::<polkadot::paras::events::CurrentCodeUpdated>()?.unwrap();
            indexer.index_event_para_id(event.0.0, block_number, event_index);
            Ok(())
        },
        "CurrentHeadUpdated" => {
            let event = event.as_event::<polkadot::paras::events::CurrentHeadUpdated>()?.unwrap();
            indexer.index_event_para_id(event.0.0, block_number, event_index);
            Ok(())
        },
        "CodeUpgradeScheduled" => {
            let event = event.as_event::<polkadot::paras::events::CodeUpgradeScheduled>()?.unwrap();
            indexer.index_event_para_id(event.0.0, block_number, event_index);
            Ok(())
        },
        "NewHeadNoted" => {
            let event = event.as_event::<polkadot::paras::events::NewHeadNoted>()?.unwrap();
            indexer.index_event_para_id(event.0.0, block_number, event_index);
            Ok(())
        },
        "ActionQueued" => {
            let event = event.as_event::<polkadot::paras::events::ActionQueued>()?.unwrap();
            indexer.index_event_para_id(event.0.0, block_number, event_index);
            indexer.index_event_session_index(event.1, block_number, event_index);
            Ok(())
        },
        "PvfCheckStarted" => {
            let event = event.as_event::<polkadot::paras::events::PvfCheckStarted>()?.unwrap();
            indexer.index_event_para_id(event.1.0, block_number, event_index);
            Ok(())
        },
        "PvfCheckAccepted" => {
            let event = event.as_event::<polkadot::paras::events::PvfCheckAccepted>()?.unwrap();
            indexer.index_event_para_id(event.1.0, block_number, event_index);
            Ok(())
        },
        "PvfCheckRejected" => {
            let event = event.as_event::<polkadot::paras::events::PvfCheckRejected>()?.unwrap();
            indexer.index_event_para_id(event.1.0, block_number, event_index);
            Ok(())
        },
        _ => Ok(()),
    }
}
