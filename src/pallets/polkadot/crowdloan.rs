use crate::shared::*;
use crate::substrate::*;

pub fn crowdloan_index_event(trees: Trees, block_number: u32, event_index: u32, event: subxt::events::EventDetails) -> Result<(), subxt::Error> {
    match event.variant_name() {
        "Created" => {
            let event = event.as_event::<polkadot::crowdloan::events::Created>()?.unwrap();
            index_event_para_id(trees, event.para_id.0, block_number, event_index);
            Ok(())
        },
        "Contributed" => {
            let event = event.as_event::<polkadot::crowdloan::events::Contributed>()?.unwrap();
            index_event_account_id(trees.clone(), event.who, block_number, event_index);
            index_event_para_id(trees, event.fund_index.0, block_number, event_index);
            Ok(())
        },
        "Withdrew" => {
            let event = event.as_event::<polkadot::crowdloan::events::Withdrew>()?.unwrap();
            index_event_account_id(trees.clone(), event.who, block_number, event_index);
            index_event_para_id(trees, event.fund_index.0, block_number, event_index);
            Ok(())
        },
        "PartiallyRefunded" => {
            let event = event.as_event::<polkadot::crowdloan::events::PartiallyRefunded>()?.unwrap();
            index_event_para_id(trees, event.para_id.0, block_number, event_index);
            Ok(())
        },
        "AllRefunded" => {
            let event = event.as_event::<polkadot::crowdloan::events::AllRefunded>()?.unwrap();
            index_event_para_id(trees, event.para_id.0, block_number, event_index);
            Ok(())
        },
        "Dissolved" => {
            let event = event.as_event::<polkadot::crowdloan::events::Dissolved>()?.unwrap();
            index_event_para_id(trees, event.para_id.0, block_number, event_index);
            Ok(())
        },
        "HandleBidResult" => {
            let event = event.as_event::<polkadot::crowdloan::events::HandleBidResult>()?.unwrap();
            index_event_para_id(trees, event.para_id.0, block_number, event_index);
            Ok(())
        },
        "Edited" => {
            let event = event.as_event::<polkadot::crowdloan::events::Edited>()?.unwrap();
            index_event_para_id(trees, event.para_id.0, block_number, event_index);
            Ok(())
        },
        "MemoUpdated" => {
            let event = event.as_event::<polkadot::crowdloan::events::MemoUpdated>()?.unwrap();
            index_event_account_id(trees.clone(), event.who, block_number, event_index);
            index_event_para_id(trees, event.para_id.0, block_number, event_index);
            Ok(())
        },
        "AddedToNewRaise" => {
            let event = event.as_event::<polkadot::crowdloan::events::AddedToNewRaise>()?.unwrap();
            index_event_para_id(trees, event.para_id.0, block_number, event_index);
            Ok(())
        },
        _ => Ok(()),
    }
}
