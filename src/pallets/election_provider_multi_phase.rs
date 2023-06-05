use subxt::PolkadotConfig;
use crate::shared::*;
use crate::substrate::*;

pub fn election_provider_multi_phase_index_event(indexer: &Indexer, block_number: u32, event_index: u32, event: subxt::events::EventDetails<PolkadotConfig>) -> Result<(), subxt::Error> {
    match event.variant_name() {
        "SolutionStored" => {
            let event = event.as_event::<polkadot::election_provider_multi_phase::events::SolutionStored>()?.unwrap();
            if let Some(account) = event.origin {
                indexer.index_event_account_id(account, block_number, event_index);
            }
            Ok(())
        },
        "Rewarded" => {
            let event = event.as_event::<polkadot::election_provider_multi_phase::events::Rewarded>()?.unwrap();
            indexer.index_event_account_id(event.account, block_number, event_index);
            Ok(())
        },
        "Slashed" => {
            let event = event.as_event::<polkadot::election_provider_multi_phase::events::Slashed>()?.unwrap();
            indexer.index_event_account_id(event.account, block_number, event_index);
            Ok(())
        },
        _ => Ok(()),
    }
}
