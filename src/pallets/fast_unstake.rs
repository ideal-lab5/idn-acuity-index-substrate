use crate::shared::*;
use crate::substrate::*;
use subxt::PolkadotConfig;

pub fn fast_unstake_index_event(
    indexer: &Indexer,
    block_number: u32,
    event_index: u32,
    event: subxt::events::EventDetails<PolkadotConfig>,
) -> Result<(), subxt::Error> {
    match event.variant_name() {
        "Unstaked" => {
            let event = event
                .as_event::<polkadot::fast_unstake::events::Unstaked>()?
                .unwrap();
            indexer.index_event_account_id(event.stash, block_number, event_index);
            Ok(())
        }
        "Slashed" => {
            let event = event
                .as_event::<polkadot::fast_unstake::events::Slashed>()?
                .unwrap();
            indexer.index_event_account_id(event.stash, block_number, event_index);
            Ok(())
        }
        "BatchChecked" => {
            let event = event
                .as_event::<polkadot::fast_unstake::events::BatchChecked>()?
                .unwrap();
            for era in event.eras {
                indexer.index_event_era_index(era, block_number, event_index);
            }
            Ok(())
        }
        _ => Ok(()),
    }
}
