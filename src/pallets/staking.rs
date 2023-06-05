use subxt::PolkadotConfig;
use crate::shared::*;
use crate::substrate::*;

pub fn staking_index_event(indexer: &Indexer, block_number: u32, event_index: u32, event: subxt::events::EventDetails<PolkadotConfig>) -> Result<(), subxt::Error> {
    match event.variant_name() {
        "EraPaid" => {
            let event = event.as_event::<polkadot::staking::events::EraPaid>()?.unwrap();
            indexer.index_event_era_index(event.era_index, block_number, event_index);
            Ok(())
        },
        "Rewarded" => {
            let event = event.as_event::<polkadot::staking::events::Rewarded>()?.unwrap();
            indexer.index_event_account_id(event.stash, block_number, event_index);
            Ok(())
        },
        "Slashed" => {
            let event = event.as_event::<polkadot::staking::events::Slashed>()?.unwrap();
            indexer.index_event_account_id(event.staker, block_number, event_index);
            Ok(())
        },
        "SlashReported" => {
            let event = event.as_event::<polkadot::staking::events::SlashReported>()?.unwrap();
            indexer.index_event_account_id(event.validator, block_number, event_index);
            indexer.index_event_era_index(event.slash_era, block_number, event_index);
            Ok(())
        },
        "OldSlashingReportDiscarded" => {
            let event = event.as_event::<polkadot::staking::events::OldSlashingReportDiscarded>()?.unwrap();
            indexer.index_event_session_index(event.session_index, block_number, event_index);
            Ok(())
        },
        "Bonded" => {
            let event = event.as_event::<polkadot::staking::events::Bonded>()?.unwrap();
            indexer.index_event_account_id(event.stash, block_number, event_index);
            Ok(())
        },
        "Unbonded" => {
            let event = event.as_event::<polkadot::staking::events::Unbonded>()?.unwrap();
            indexer.index_event_account_id(event.stash, block_number, event_index);
            Ok(())
        },
        "Withdrawn" => {
            let event = event.as_event::<polkadot::staking::events::Withdrawn>()?.unwrap();
            indexer.index_event_account_id(event.stash, block_number, event_index);
            Ok(())
        },
        "Kicked" => {
            let event = event.as_event::<polkadot::staking::events::Kicked>()?.unwrap();
            indexer.index_event_account_id(event.nominator, block_number, event_index);
            indexer.index_event_account_id(event.stash, block_number, event_index);
            Ok(())
        },
        "Chilled" => {
            let event = event.as_event::<polkadot::staking::events::Chilled>()?.unwrap();
            indexer.index_event_account_id(event.stash, block_number, event_index);
            Ok(())
        },
        "PayoutStarted" => {
            let event = event.as_event::<polkadot::staking::events::PayoutStarted>()?.unwrap();
            indexer.index_event_era_index(event.era_index, block_number, event_index);
            indexer.index_event_account_id(event.validator_stash, block_number, event_index);
            Ok(())
        },
        "ValidatorPrefsSet" => {
            let event = event.as_event::<polkadot::staking::events::ValidatorPrefsSet>()?.unwrap();
            indexer.index_event_account_id(event.stash, block_number, event_index);
            Ok(())
        },
        _ => Ok(()),
    }
}
