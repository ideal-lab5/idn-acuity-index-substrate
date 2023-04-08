use subxt::{
    utils::AccountId32,
};

use parity_scale_codec::{Encode, Decode};
use serde::{Serialize, Deserialize};

use crate::shared::*;
use crate::substrate::*;

#[derive(Encode, Decode, Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "variant", content = "details")]
pub enum Registrar {
    #[serde(rename_all = "camelCase")]
	Registered {
	    para_id: ParaId,
	    manager: AccountId32,
	},
    #[serde(rename_all = "camelCase")]
	Deregistered {
	    para_id: ParaId,
	},
    #[serde(rename_all = "camelCase")]
	Reserved {
	    para_id: ParaId,
	    who: AccountId32,
	},
}

pub fn paras_registrar_index_event(trees: Trees, block_number: u32, event_index: u32, event: subxt::events::EventDetails) -> Result<(), subxt::Error> {
    match event.variant_name() {
        "Registered" => {
            let event = event.as_event::<polkadot::registrar::events::Registered>()?.unwrap();
            let event_db = Event::Registrar(
                Registrar::Registered {
	                para_id: ParaId(event.para_id.0),
	                manager: event.manager.clone(),
                }
            );
            let value = Event::encode(&event_db);
            index_event_para_id(trees.clone(), event.para_id.0, block_number, event_index, &value);
            index_event_account_id(trees.clone(), event.manager, block_number, event_index, &value);
            Ok(())
        },
        "Deregistered" => {
            let event = event.as_event::<polkadot::registrar::events::Deregistered>()?.unwrap();
            let event_db = Event::Registrar(
                Registrar::Deregistered {
	                para_id: ParaId(event.para_id.0),
                }
            );
            let value = Event::encode(&event_db);
            index_event_para_id(trees.clone(), event.para_id.0, block_number, event_index, &value);
            Ok(())
        },
        "Reserved" => {
            let event = event.as_event::<polkadot::registrar::events::Reserved>()?.unwrap();
            let event_db = Event::Registrar(
                Registrar::Reserved {
	                para_id: ParaId(event.para_id.0),
	                who: event.who.clone(),
                }
            );
            let value = Event::encode(&event_db);
            index_event_para_id(trees.clone(), event.para_id.0, block_number, event_index, &value);
            index_event_account_id(trees.clone(), event.who, block_number, event_index, &value);
            Ok(())
        },
        _ => Ok(()),
    }
}
