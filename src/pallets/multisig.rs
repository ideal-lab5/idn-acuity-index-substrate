use crate::shared::*;
use crate::substrate::*;
use subxt::PolkadotConfig;

pub fn multisig_index_event(
    indexer: &Indexer,
    block_number: u32,
    event_index: u32,
    event: subxt::events::EventDetails<PolkadotConfig>,
) -> Result<(), subxt::Error> {
    match event.variant_name() {
        "NewMultisig" => {
            let event = event
                .as_event::<polkadot::multisig::events::NewMultisig>()?
                .unwrap();
            indexer.index_event_account_id(event.approving, block_number, event_index);
            indexer.index_event_account_id(event.multisig, block_number, event_index);
            Ok(())
        }
        "MultisigApproval" => {
            let event = event
                .as_event::<polkadot::multisig::events::MultisigApproval>()?
                .unwrap();
            indexer.index_event_account_id(event.approving, block_number, event_index);
            indexer.index_event_account_id(event.multisig, block_number, event_index);
            Ok(())
        }
        "MultisigExecuted" => {
            let event = event
                .as_event::<polkadot::multisig::events::MultisigExecuted>()?
                .unwrap();
            indexer.index_event_account_id(event.approving, block_number, event_index);
            indexer.index_event_account_id(event.multisig, block_number, event_index);
            Ok(())
        }
        "MultisigCancelled" => {
            let event = event
                .as_event::<polkadot::multisig::events::MultisigCancelled>()?
                .unwrap();
            indexer.index_event_account_id(event.cancelling, block_number, event_index);
            indexer.index_event_account_id(event.multisig, block_number, event_index);
            Ok(())
        }
        _ => Ok(()),
    }
}
