use parity_scale_codec::{Encode, Decode};
use serde::{Serialize, Deserialize};

use crate::shared::*;
use crate::substrate::*;

#[derive(Encode, Decode, Serialize, Deserialize, Debug, Clone)]
pub struct HrmpChannelId {
    /// The para that acts as the sender in this channel.
    pub sender: ParaId,
    /// The para that acts as the recipient in this channel.
    pub recipient: ParaId,
}

use crate::shared::polkadot::runtime_types::polkadot_parachain::primitives::HrmpChannelId as SubHrmpChannelId;

impl From<SubHrmpChannelId> for HrmpChannelId {
    fn from(x: SubHrmpChannelId) -> Self {
        HrmpChannelId {
            sender: ParaId(x.sender.0),
            recipient: ParaId(x.recipient.0),
        }
    }
}

#[derive(Encode, Decode, Serialize, Deserialize, Debug, Clone)]
#[allow(clippy::enum_variant_names)]pub enum Hrmp {
    OpenChannelRequested(ParaId, ParaId, u32, u32),
    OpenChannelCanceled(ParaId, HrmpChannelId),
    OpenChannelAccepted(ParaId, ParaId),
    ChannelClosed(ParaId, HrmpChannelId),
    HrmpChannelForceOpened(ParaId, ParaId, u32, u32),
}

pub fn parachains_hrmp_index_event(trees: Trees, block_number: u32, event_index: u32, event: subxt::events::EventDetails) -> Result<(), subxt::Error> {
    match event.variant_name() {
        "OpenChannelRequested" => {
            let event = event.as_event::<polkadot::hrmp::events::OpenChannelRequested>()?.unwrap();
            let event_db = Event::Hrmp(
                Hrmp::OpenChannelRequested(ParaId(event.0.0), ParaId(event.1.0), event.2, event.3)
            );
            let value = Event::encode(&event_db);
            index_event_para_id(trees.clone(), event.0.0, block_number, event_index, &value);
            index_event_para_id(trees, event.1.0, block_number, event_index, &value);
            Ok(())
        },
        "OpenChannelCanceled" => {
            let event = event.as_event::<polkadot::hrmp::events::OpenChannelCanceled>()?.unwrap();
            let event_db = Event::Hrmp(
                Hrmp::OpenChannelCanceled(ParaId(event.0.0), event.1.into())
            );
            let value = Event::encode(&event_db);
            index_event_para_id(trees, event.0.0, block_number, event_index, &value);
            Ok(())
        },
        "OpenChannelAccepted" => {
            let event = event.as_event::<polkadot::hrmp::events::OpenChannelAccepted>()?.unwrap();
            let event_db = Event::Hrmp(
                Hrmp::OpenChannelAccepted(ParaId(event.0.0), ParaId(event.1.0))
            );
            let value = Event::encode(&event_db);
            index_event_para_id(trees.clone(), event.0.0, block_number, event_index, &value);
            index_event_para_id(trees, event.1.0, block_number, event_index, &value);
            Ok(())
        },
        "ChannelClosed" => {
            let event = event.as_event::<polkadot::hrmp::events::ChannelClosed>()?.unwrap();
            let event_db = Event::Hrmp(
                Hrmp::ChannelClosed(ParaId(event.0.0), event.1.into())
            );
            let value = Event::encode(&event_db);
            index_event_para_id(trees, event.0.0, block_number, event_index, &value);
            Ok(())
        },
        "HrmpChannelForceOpened" => {
            let event = event.as_event::<polkadot::hrmp::events::HrmpChannelForceOpened>()?.unwrap();
            let event_db = Event::Hrmp(
                Hrmp::HrmpChannelForceOpened(ParaId(event.0.0), ParaId(event.1.0), event.2, event.3)
            );
            let value = Event::encode(&event_db);
            index_event_para_id(trees.clone(), event.0.0, block_number, event_index, &value);
            index_event_para_id(trees, event.1.0, block_number, event_index, &value);
            Ok(())
        },
        _ => Ok(()),
    }
}
