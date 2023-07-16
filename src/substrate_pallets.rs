#[macro_export]
macro_rules! index_system_event {
    ($event_enum: ty, $event: ident, $indexer: ident, $block_number: ident, $event_index: ident) => {
        match $event {
            <$event_enum>::NewAccount { account } => {
                $indexer.index_event_account_id(account, $block_number, $event_index);
            }
            <$event_enum>::KilledAccount { account } => {
                $indexer.index_event_account_id(account, $block_number, $event_index);
            }
            _ => {}
        }
    };
}

#[macro_export]
macro_rules! index_preimage_event {
    ($event_enum: ty, $event: ident, $indexer: ident, $block_number: ident, $event_index: ident) => {
        match $event {
            <$event_enum>::Noted { hash } => {
                $indexer.index_event_preimage_hash(hash.into(), $block_number, $event_index);
            }
            <$event_enum>::Requested { hash } => {
                $indexer.index_event_preimage_hash(hash.into(), $block_number, $event_index);
            }
            <$event_enum>::Cleared { hash } => {
                $indexer.index_event_preimage_hash(hash.into(), $block_number, $event_index);
            }
            _ => {}
        }
    };
}

#[macro_export]
macro_rules! index_indicies_event {
    ($event_enum: ty, $event: ident, $indexer: ident, $block_number: ident, $event_index: ident) => {
        match $event {
            <$event_enum>::IndexAssigned { who, index } => {
                $indexer.index_event_account_id(who, $block_number, $event_index);
                $indexer.index_event_account_index(index, $block_number, $event_index);
            }
            <$event_enum>::IndexFreed { index } => {
                $indexer.index_event_account_index(index, $block_number, $event_index);
            }
            <$event_enum>::IndexFrozen { index, who } => {
                $indexer.index_event_account_index(index, $block_number, $event_index);
                $indexer.index_event_account_id(who, $block_number, $event_index);
            }
            _ => {}
        }
    };
}
