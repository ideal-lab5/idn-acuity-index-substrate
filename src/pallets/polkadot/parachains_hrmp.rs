use crate::shared::*;
use crate::substrate::*;

pub fn parachains_hrmp_index_event(trees: Trees, block_number: u32, event_index: u32, event: subxt::events::EventDetails) -> Result<(), subxt::Error> {
    match event.variant_name() {
        "OpenChannelRequested" => {
            let event = event.as_event::<polkadot::hrmp::events::OpenChannelRequested>()?.unwrap();
            index_event_para_id(trees.clone(), event.0.0, block_number, event_index);
            index_event_para_id(trees, event.1.0, block_number, event_index);
            Ok(())
        },
        "OpenChannelCanceled" => {
            let event = event.as_event::<polkadot::hrmp::events::OpenChannelCanceled>()?.unwrap();
            index_event_para_id(trees, event.0.0, block_number, event_index);
            Ok(())
        },
        "OpenChannelAccepted" => {
            let event = event.as_event::<polkadot::hrmp::events::OpenChannelAccepted>()?.unwrap();
            index_event_para_id(trees.clone(), event.0.0, block_number, event_index);
            index_event_para_id(trees, event.1.0, block_number, event_index);
            Ok(())
        },
        "ChannelClosed" => {
            let event = event.as_event::<polkadot::hrmp::events::ChannelClosed>()?.unwrap();
            index_event_para_id(trees, event.0.0, block_number, event_index);
            Ok(())
        },
        "HrmpChannelForceOpened" => {
            let event = event.as_event::<polkadot::hrmp::events::HrmpChannelForceOpened>()?.unwrap();
            index_event_para_id(trees.clone(), event.0.0, block_number, event_index);
            index_event_para_id(trees, event.1.0, block_number, event_index);
            Ok(())
        },
        _ => Ok(()),
    }
}
