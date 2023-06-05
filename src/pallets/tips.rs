use subxt::PolkadotConfig;
use crate::shared::*;
use crate::substrate::*;

pub fn tips_index_event(indexer: &Indexer, block_number: u32, event_index: u32, event: subxt::events::EventDetails<PolkadotConfig>) -> Result<(), subxt::Error> {
    match event.variant_name() {
        "NewTip" => {
            let event = event.as_event::<polkadot::tips::events::NewTip>()?.unwrap();
            indexer.index_event_tip_hash(event.tip_hash.into(), block_number, event_index);
            Ok(())
        },
        "TipClosing" => {
            let event = event.as_event::<polkadot::tips::events::TipClosing>()?.unwrap();
            indexer.index_event_tip_hash(event.tip_hash.into(), block_number, event_index);
            Ok(())
        },
        "TipClosed" => {
            let event = event.as_event::<polkadot::tips::events::TipClosed>()?.unwrap();
            indexer.index_event_tip_hash(event.tip_hash.into(), block_number, event_index);
            indexer.index_event_account_id(event.who, block_number, event_index);
            Ok(())
        },
        "TipRetracted" => {
            let event = event.as_event::<polkadot::tips::events::TipRetracted>()?.unwrap();
            indexer.index_event_tip_hash(event.tip_hash.into(), block_number, event_index);
            Ok(())
        },
        "TipSlashed" => {
            let event = event.as_event::<polkadot::tips::events::TipSlashed>()?.unwrap();
            indexer.index_event_tip_hash(event.tip_hash.into(), block_number, event_index);
            indexer.index_event_account_id(event.finder, block_number, event_index);
            Ok(())
        },
        _ => Ok(()),
    }
}
