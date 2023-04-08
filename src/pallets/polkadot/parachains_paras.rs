use parity_scale_codec::{Encode, Decode};
use serde::{Serialize, Deserialize};

use crate::shared::*;
use crate::substrate::*;

#[derive(Encode, Decode, Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ValidationCodeHash([u8; 32]);

pub type SessionIndex = u32;

#[derive(Encode, Decode, Serialize, Deserialize, Debug, Clone)]
pub enum Paras {
    CurrentCodeUpdated(ParaId),
	CurrentHeadUpdated(ParaId),
	CodeUpgradeScheduled(ParaId),
	NewHeadNoted(ParaId),
	ActionQueued(ParaId, SessionIndex),
	PvfCheckStarted(ValidationCodeHash, ParaId),
	PvfCheckAccepted(ValidationCodeHash, ParaId),
	PvfCheckRejected(ValidationCodeHash, ParaId),
}

pub fn parachains_paras_index_event(trees: Trees, block_number: u32, event_index: u32, event: subxt::events::EventDetails) -> Result<(), subxt::Error> {
    match event.variant_name() {
        "CurrentCodeUpdated" => {
            let event = event.as_event::<polkadot::paras::events::CurrentCodeUpdated>()?.unwrap();
            let event_db = Event::Paras(
                Paras::CurrentCodeUpdated(ParaId(event.0.0))
            );
            let value = Event::encode(&event_db);
            index_event_para_id(trees.clone(), event.0.0, block_number, event_index, &value);
            Ok(())
        },
        "CurrentHeadUpdated" => {
            let event = event.as_event::<polkadot::paras::events::CurrentHeadUpdated>()?.unwrap();
            let event_db = Event::Paras(
                Paras::CurrentHeadUpdated(ParaId(event.0.0))
            );
            let value = Event::encode(&event_db);
            index_event_para_id(trees.clone(), event.0.0, block_number, event_index, &value);
            Ok(())
        },
        "CodeUpgradeScheduled" => {
            let event = event.as_event::<polkadot::paras::events::CodeUpgradeScheduled>()?.unwrap();
            let event_db = Event::Paras(
                Paras::CodeUpgradeScheduled(ParaId(event.0.0))
            );
            let value = Event::encode(&event_db);
            index_event_para_id(trees.clone(), event.0.0, block_number, event_index, &value);
            Ok(())
        },
        "NewHeadNoted" => {
            let event = event.as_event::<polkadot::paras::events::NewHeadNoted>()?.unwrap();
            let event_db = Event::Paras(
                Paras::NewHeadNoted(ParaId(event.0.0))
            );
            let value = Event::encode(&event_db);
            index_event_para_id(trees.clone(), event.0.0, block_number, event_index, &value);
            Ok(())
        },
        "ActionQueued" => {
            let event = event.as_event::<polkadot::paras::events::ActionQueued>()?.unwrap();
            let event_db = Event::Paras(
                Paras::ActionQueued(ParaId(event.0.0), event.1)
            );
            let value = Event::encode(&event_db);
            index_event_para_id(trees.clone(), event.0.0, block_number, event_index, &value);
            Ok(())
        },
        "PvfCheckStarted" => {
            let event = event.as_event::<polkadot::paras::events::PvfCheckStarted>()?.unwrap();
            let event_db = Event::Paras(
                Paras::PvfCheckStarted(ValidationCodeHash(event.0.0.0), ParaId(event.1.0))
            );
            let value = Event::encode(&event_db);
            index_event_para_id(trees.clone(), event.1.0, block_number, event_index, &value);
            Ok(())
        },
        "PvfCheckAccepted" => {
            let event = event.as_event::<polkadot::paras::events::PvfCheckAccepted>()?.unwrap();
            let event_db = Event::Paras(
                Paras::PvfCheckAccepted(ValidationCodeHash(event.0.0.0), ParaId(event.1.0))
            );
            let value = Event::encode(&event_db);
            index_event_para_id(trees.clone(), event.1.0, block_number, event_index, &value);
            Ok(())
        },
        "PvfCheckRejected" => {
            let event = event.as_event::<polkadot::paras::events::PvfCheckRejected>()?.unwrap();
            let event_db = Event::Paras(
                Paras::PvfCheckRejected(ValidationCodeHash(event.0.0.0), ParaId(event.1.0))
            );
            let value = Event::encode(&event_db);
            index_event_para_id(trees.clone(), event.1.0, block_number, event_index, &value);
            Ok(())
        },
        _ => Ok(()),
    }
}
