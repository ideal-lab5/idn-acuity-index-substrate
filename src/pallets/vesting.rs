use subxt::PolkadotConfig;
use crate::shared::*;
use crate::substrate::*;

pub fn vesting_index_event(trees: Trees, block_number: u32, event_index: u32, event: subxt::events::EventDetails<PolkadotConfig>) -> Result<(), subxt::Error> {
    match event.variant_name() {
        "VestingUpdated" => {
            let event = event.as_event::<polkadot::vesting::events::VestingUpdated>().unwrap().unwrap();
            index_event_account_id(trees, event.account, block_number, event_index);
            Ok(())
        },
        "VestingCompleted" => {
            let event = event.as_event::<polkadot::vesting::events::VestingCompleted>().unwrap().unwrap();
            index_event_account_id(trees, event.account, block_number, event_index);
            Ok(())
        },
        _ => Ok(()),
    }
}
