use crate::shared::*;
use crate::substrate::*;

pub fn identity_index_event(trees: Trees, block_number: u32, event_index: u32, event: subxt::events::EventDetails) -> Result<(), subxt::Error> {
    match event.variant_name() {
        "IdentitySet" => {
            let event = event.as_event::<polkadot::identity::events::IdentitySet>()?.unwrap();
            index_event_account_id(trees, event.who, block_number, event_index);
            Ok(())
        },
        "IdentityCleared" => {
            let event = event.as_event::<polkadot::identity::events::IdentityCleared>()?.unwrap();
            index_event_account_id(trees, event.who, block_number, event_index);
            Ok(())
        },
        "IdentityKilled" => {
            let event = event.as_event::<polkadot::identity::events::IdentityKilled>()?.unwrap();
            index_event_account_id(trees, event.who, block_number, event_index);
            Ok(())
        },
        "JudgementRequested" => {
            let event = event.as_event::<polkadot::identity::events::JudgementRequested>()?.unwrap();
            index_event_account_id(trees.clone(), event.who, block_number, event_index);
            index_event_registrar_index(trees, event.registrar_index, block_number, event_index);
            Ok(())
        },
        "JudgementUnrequested" => {
            let event = event.as_event::<polkadot::identity::events::JudgementUnrequested>()?.unwrap();
            index_event_account_id(trees.clone(), event.who, block_number, event_index);
            index_event_registrar_index(trees, event.registrar_index, block_number, event_index);
            Ok(())
        },
        "JudgementGiven" => {
            let event = event.as_event::<polkadot::identity::events::JudgementGiven>()?.unwrap();
            index_event_account_id(trees.clone(), event.target, block_number, event_index);
            index_event_registrar_index(trees, event.registrar_index, block_number, event_index);
            Ok(())
        },
        "RegistrarAdded" => {
            let event = event.as_event::<polkadot::identity::events::RegistrarAdded>()?.unwrap();
            index_event_registrar_index(trees, event.registrar_index, block_number, event_index);
            Ok(())
        },
        "SubIdentityAdded" => {
            let event = event.as_event::<polkadot::identity::events::SubIdentityAdded>()?.unwrap();
            index_event_account_id(trees.clone(), event.sub, block_number, event_index);
            index_event_account_id(trees, event.main, block_number, event_index);
            Ok(())
        },
        "SubIdentityRemoved" => {
            let event = event.as_event::<polkadot::identity::events::SubIdentityRemoved>()?.unwrap();
            index_event_account_id(trees.clone(), event.sub, block_number, event_index);
            index_event_account_id(trees, event.main, block_number, event_index);
            Ok(())
        },
        "SubIdentityRevoked" => {
            let event = event.as_event::<polkadot::identity::events::SubIdentityRevoked>()?.unwrap();
            index_event_account_id(trees.clone(), event.sub, block_number, event_index);
            index_event_account_id(trees, event.main, block_number, event_index);
            Ok(())
        },
        _ => Ok(()),
    }
}
