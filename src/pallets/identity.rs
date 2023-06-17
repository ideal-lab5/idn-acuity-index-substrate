use crate::shared::*;
use crate::substrate::*;
use subxt::PolkadotConfig;

pub fn identity_index_event(
    indexer: &Indexer,
    block_number: u32,
    event_index: u32,
    event: subxt::events::EventDetails<PolkadotConfig>,
) -> Result<(), subxt::Error> {
    match event.variant_name() {
        "IdentitySet" => {
            let event = event
                .as_event::<polkadot::identity::events::IdentitySet>()?
                .unwrap();
            indexer.index_event_account_id(event.who, block_number, event_index);
            Ok(())
        }
        "IdentityCleared" => {
            let event = event
                .as_event::<polkadot::identity::events::IdentityCleared>()?
                .unwrap();
            indexer.index_event_account_id(event.who, block_number, event_index);
            Ok(())
        }
        "IdentityKilled" => {
            let event = event
                .as_event::<polkadot::identity::events::IdentityKilled>()?
                .unwrap();
            indexer.index_event_account_id(event.who, block_number, event_index);
            Ok(())
        }
        "JudgementRequested" => {
            let event = event
                .as_event::<polkadot::identity::events::JudgementRequested>()?
                .unwrap();
            indexer.index_event_account_id(event.who, block_number, event_index);
            indexer.index_event_registrar_index(event.registrar_index, block_number, event_index);
            Ok(())
        }
        "JudgementUnrequested" => {
            let event = event
                .as_event::<polkadot::identity::events::JudgementUnrequested>()?
                .unwrap();
            indexer.index_event_account_id(event.who, block_number, event_index);
            indexer.index_event_registrar_index(event.registrar_index, block_number, event_index);
            Ok(())
        }
        "JudgementGiven" => {
            let event = event
                .as_event::<polkadot::identity::events::JudgementGiven>()?
                .unwrap();
            indexer.index_event_account_id(event.target, block_number, event_index);
            indexer.index_event_registrar_index(event.registrar_index, block_number, event_index);
            Ok(())
        }
        "RegistrarAdded" => {
            let event = event
                .as_event::<polkadot::identity::events::RegistrarAdded>()?
                .unwrap();
            indexer.index_event_registrar_index(event.registrar_index, block_number, event_index);
            Ok(())
        }
        "SubIdentityAdded" => {
            let event = event
                .as_event::<polkadot::identity::events::SubIdentityAdded>()?
                .unwrap();
            indexer.index_event_account_id(event.sub, block_number, event_index);
            indexer.index_event_account_id(event.main, block_number, event_index);
            Ok(())
        }
        "SubIdentityRemoved" => {
            let event = event
                .as_event::<polkadot::identity::events::SubIdentityRemoved>()?
                .unwrap();
            indexer.index_event_account_id(event.sub, block_number, event_index);
            indexer.index_event_account_id(event.main, block_number, event_index);
            Ok(())
        }
        "SubIdentityRevoked" => {
            let event = event
                .as_event::<polkadot::identity::events::SubIdentityRevoked>()?
                .unwrap();
            indexer.index_event_account_id(event.sub, block_number, event_index);
            indexer.index_event_account_id(event.main, block_number, event_index);
            Ok(())
        }
        _ => Ok(()),
    }
}
