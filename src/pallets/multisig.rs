use crate::shared::*;
use crate::substrate::*;

pub fn multisig_index_event(trees: Trees, block_number: u32, event_index: u32, event: subxt::events::EventDetails) -> Result<(), subxt::Error> {
    match event.variant_name() {
        "NewMultisig" => {
            let event = event.as_event::<polkadot::multisig::events::NewMultisig>()?.unwrap();
            index_event_account_id(trees.clone(), event.approving, block_number, event_index);
            index_event_account_id(trees, event.multisig, block_number, event_index);
            Ok(())
        },
        "MultisigApproval" => {
            let event = event.as_event::<polkadot::multisig::events::MultisigApproval>()?.unwrap();
            index_event_account_id(trees.clone(), event.approving, block_number, event_index);
            index_event_account_id(trees, event.multisig, block_number, event_index);
            Ok(())
        },
        "MultisigExecuted" => {
            let event = event.as_event::<polkadot::multisig::events::MultisigExecuted>()?.unwrap();
            index_event_account_id(trees.clone(), event.approving, block_number, event_index);
            index_event_account_id(trees, event.multisig, block_number, event_index);
            Ok(())
        },
        "MultisigCancelled" => {
            let event = event.as_event::<polkadot::multisig::events::MultisigCancelled>()?.unwrap();
            index_event_account_id(trees.clone(), event.cancelling, block_number, event_index);
            index_event_account_id(trees, event.multisig, block_number, event_index);
            Ok(())
        },
        _ => Ok(()),
    }
}
