use subxt::PolkadotConfig;
use crate::shared::*;
use crate::substrate::*;

pub fn parachains_paras_index_event(trees: Trees, block_number: u32, event_index: u32, event: subxt::events::EventDetails<PolkadotConfig>) -> Result<(), subxt::Error> {
    match event.variant_name() {
        "CurrentCodeUpdated" => {
            let event = event.as_event::<polkadot::paras::events::CurrentCodeUpdated>()?.unwrap();
            index_event_para_id(trees, event.0.0, block_number, event_index);
            Ok(())
        },
        "CurrentHeadUpdated" => {
            let event = event.as_event::<polkadot::paras::events::CurrentHeadUpdated>()?.unwrap();
            index_event_para_id(trees, event.0.0, block_number, event_index);
            Ok(())
        },
        "CodeUpgradeScheduled" => {
            let event = event.as_event::<polkadot::paras::events::CodeUpgradeScheduled>()?.unwrap();
            index_event_para_id(trees, event.0.0, block_number, event_index);
            Ok(())
        },
        "NewHeadNoted" => {
            let event = event.as_event::<polkadot::paras::events::NewHeadNoted>()?.unwrap();
            index_event_para_id(trees, event.0.0, block_number, event_index);
            Ok(())
        },
        "ActionQueued" => {
            let event = event.as_event::<polkadot::paras::events::ActionQueued>()?.unwrap();
            index_event_para_id(trees.clone(), event.0.0, block_number, event_index);
            index_event_session_index(trees, event.1, block_number, event_index);
            Ok(())
        },
        "PvfCheckStarted" => {
            let event = event.as_event::<polkadot::paras::events::PvfCheckStarted>()?.unwrap();
            index_event_para_id(trees, event.1.0, block_number, event_index);
            Ok(())
        },
        "PvfCheckAccepted" => {
            let event = event.as_event::<polkadot::paras::events::PvfCheckAccepted>()?.unwrap();
            index_event_para_id(trees, event.1.0, block_number, event_index);
            Ok(())
        },
        "PvfCheckRejected" => {
            let event = event.as_event::<polkadot::paras::events::PvfCheckRejected>()?.unwrap();
            index_event_para_id(trees, event.1.0, block_number, event_index);
            Ok(())
        },
        _ => Ok(()),
    }
}
