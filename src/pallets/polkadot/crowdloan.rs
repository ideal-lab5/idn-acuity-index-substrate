use subxt::PolkadotConfig;
use crate::shared::*;
use crate::substrate::*;

pub fn crowdloan_index_event(indexer: &Indexer, block_number: u32, event_index: u32, event: subxt::events::EventDetails<PolkadotConfig>) -> Result<(), subxt::Error> {
    match event.variant_name() {
        "Created" => {
            let event = event.as_event::<polkadot::crowdloan::events::Created>()?.unwrap();
            indexer.index_event_para_id(event.para_id.0, block_number, event_index);
            Ok(())
        },
        "Contributed" => {
            let event = event.as_event::<polkadot::crowdloan::events::Contributed>()?.unwrap();
            indexer.index_event_account_id(event.who, block_number, event_index);
            indexer.index_event_para_id(event.fund_index.0, block_number, event_index);
            Ok(())
        },
        "Withdrew" => {
            let event = event.as_event::<polkadot::crowdloan::events::Withdrew>()?.unwrap();
            indexer.index_event_account_id(event.who, block_number, event_index);
            indexer.index_event_para_id(event.fund_index.0, block_number, event_index);
            Ok(())
        },
        "PartiallyRefunded" => {
            let event = event.as_event::<polkadot::crowdloan::events::PartiallyRefunded>()?.unwrap();
            indexer.index_event_para_id(event.para_id.0, block_number, event_index);
            Ok(())
        },
        "AllRefunded" => {
            let event = event.as_event::<polkadot::crowdloan::events::AllRefunded>()?.unwrap();
            indexer.index_event_para_id(event.para_id.0, block_number, event_index);
            Ok(())
        },
        "Dissolved" => {
            let event = event.as_event::<polkadot::crowdloan::events::Dissolved>()?.unwrap();
            indexer.index_event_para_id(event.para_id.0, block_number, event_index);
            Ok(())
        },
        "HandleBidResult" => {
            let event = event.as_event::<polkadot::crowdloan::events::HandleBidResult>()?.unwrap();
            indexer.index_event_para_id(event.para_id.0, block_number, event_index);
            Ok(())
        },
        "Edited" => {
            let event = event.as_event::<polkadot::crowdloan::events::Edited>()?.unwrap();
            indexer.index_event_para_id(event.para_id.0, block_number, event_index);
            Ok(())
        },
        "MemoUpdated" => {
            let event = event.as_event::<polkadot::crowdloan::events::MemoUpdated>()?.unwrap();
            indexer.index_event_account_id(event.who, block_number, event_index);
            indexer.index_event_para_id(event.para_id.0, block_number, event_index);
            Ok(())
        },
        "AddedToNewRaise" => {
            let event = event.as_event::<polkadot::crowdloan::events::AddedToNewRaise>()?.unwrap();
            indexer.index_event_para_id(event.para_id.0, block_number, event_index);
            Ok(())
        },
        _ => Ok(()),
    }
}
