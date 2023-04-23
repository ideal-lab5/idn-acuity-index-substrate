use crate::shared::*;
use crate::substrate::*;

pub fn tips_index_event(trees: Trees, block_number: u32, event_index: u32, event: subxt::events::EventDetails) -> Result<(), subxt::Error> {
    match event.variant_name() {
        "NewTip" => {
            let event = event.as_event::<polkadot::tips::events::NewTip>()?.unwrap();
            index_event_tip_hash(trees, event.tip_hash.into(), block_number, event_index);
            Ok(())
        },
        "TipClosing" => {
            let event = event.as_event::<polkadot::tips::events::TipClosing>()?.unwrap();
            index_event_tip_hash(trees, event.tip_hash.into(), block_number, event_index);
            Ok(())
        },
        "TipClosed" => {
            let event = event.as_event::<polkadot::tips::events::TipClosed>()?.unwrap();
            index_event_tip_hash(trees.clone(), event.tip_hash.into(), block_number, event_index);
            index_event_account_id(trees, event.who, block_number, event_index);
            Ok(())
        },
        "TipRetracted" => {
            let event = event.as_event::<polkadot::tips::events::TipRetracted>()?.unwrap();
            index_event_tip_hash(trees, event.tip_hash.into(), block_number, event_index);
            Ok(())
        },
        "TipSlashed" => {
            let event = event.as_event::<polkadot::tips::events::TipSlashed>()?.unwrap();
            index_event_tip_hash(trees.clone(), event.tip_hash.into(), block_number, event_index);
            index_event_account_id(trees, event.finder, block_number, event_index);
            Ok(())
        },
        _ => Ok(()),
    }
}
