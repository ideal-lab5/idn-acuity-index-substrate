use subxt::PolkadotConfig;
use crate::shared::*;
use crate::substrate::*;

pub fn staking_index_event(trees: Trees, block_number: u32, event_index: u32, event: subxt::events::EventDetails<PolkadotConfig>) -> Result<(), subxt::Error> {
    match event.variant_name() {
        "EraPaid" => {
            let event = event.as_event::<polkadot::staking::events::EraPaid>()?.unwrap();
            index_event_era_index(trees, event.era_index, block_number, event_index);
            Ok(())
        },
        "Rewarded" => {
            let event = event.as_event::<polkadot::staking::events::Rewarded>()?.unwrap();
            index_event_account_id(trees, event.stash, block_number, event_index);
            Ok(())
        },
        "Slashed" => {
            let event = event.as_event::<polkadot::staking::events::Slashed>()?.unwrap();
            index_event_account_id(trees, event.staker, block_number, event_index);
            Ok(())
        },
        "SlashReported" => {
            let event = event.as_event::<polkadot::staking::events::SlashReported>()?.unwrap();
            index_event_account_id(trees.clone(), event.validator, block_number, event_index);
            index_event_era_index(trees, event.slash_era, block_number, event_index);
            Ok(())
        },
        "OldSlashingReportDiscarded" => {
            let event = event.as_event::<polkadot::staking::events::OldSlashingReportDiscarded>()?.unwrap();
            index_event_session_index(trees, event.session_index, block_number, event_index);
            Ok(())
        },
        "Bonded" => {
            let event = event.as_event::<polkadot::staking::events::Bonded>()?.unwrap();
            index_event_account_id(trees, event.stash, block_number, event_index);
            Ok(())
        },
        "Unbonded" => {
            let event = event.as_event::<polkadot::staking::events::Unbonded>()?.unwrap();
            index_event_account_id(trees, event.stash, block_number, event_index);
            Ok(())
        },
        "Withdrawn" => {
            let event = event.as_event::<polkadot::staking::events::Withdrawn>()?.unwrap();
            index_event_account_id(trees, event.stash, block_number, event_index);
            Ok(())
        },
        "Kicked" => {
            let event = event.as_event::<polkadot::staking::events::Kicked>()?.unwrap();
            index_event_account_id(trees.clone(), event.nominator, block_number, event_index);
            index_event_account_id(trees, event.stash, block_number, event_index);
            Ok(())
        },
        "Chilled" => {
            let event = event.as_event::<polkadot::staking::events::Chilled>()?.unwrap();
            index_event_account_id(trees, event.stash, block_number, event_index);
            Ok(())
        },
        "PayoutStarted" => {
            let event = event.as_event::<polkadot::staking::events::PayoutStarted>()?.unwrap();
            index_event_era_index(trees.clone(), event.era_index, block_number, event_index);
            index_event_account_id(trees, event.validator_stash, block_number, event_index);
            Ok(())
        },
        "ValidatorPrefsSet" => {
            let event = event.as_event::<polkadot::staking::events::ValidatorPrefsSet>()?.unwrap();
            index_event_account_id(trees, event.stash, block_number, event_index);
            Ok(())
        },
        _ => Ok(()),
    }
}
