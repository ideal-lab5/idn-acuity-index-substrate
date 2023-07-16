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
macro_rules! index_indices_event {
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

#[macro_export]
macro_rules! index_balances_event {
    ($event_enum: ty, $event: ident, $indexer: ident, $block_number: ident, $event_index: ident) => {
        match $event {
            <$event_enum>::Endowed { account, .. } => {
                $indexer.index_event_account_id(account, $block_number, $event_index);
            }
            <$event_enum>::DustLost { account, .. } => {
                $indexer.index_event_account_id(account, $block_number, $event_index);
            }
            <$event_enum>::Transfer { from, to, .. } => {
                $indexer.index_event_account_id(from, $block_number, $event_index);
                $indexer.index_event_account_id(to, $block_number, $event_index);
            }
            <$event_enum>::BalanceSet { who, .. } => {
                $indexer.index_event_account_id(who, $block_number, $event_index);
            }
            <$event_enum>::Reserved { who, .. } => {
                $indexer.index_event_account_id(who, $block_number, $event_index);
            }
            <$event_enum>::Unreserved { who, .. } => {
                $indexer.index_event_account_id(who, $block_number, $event_index);
            }
            <$event_enum>::ReserveRepatriated { from, to, .. } => {
                $indexer.index_event_account_id(from, $block_number, $event_index);
                $indexer.index_event_account_id(to, $block_number, $event_index);
            }
            <$event_enum>::Deposit { who, .. } => {
                $indexer.index_event_account_id(who, $block_number, $event_index);
            }
            <$event_enum>::Withdraw { who, .. } => {
                $indexer.index_event_account_id(who, $block_number, $event_index);
            }
            <$event_enum>::Slashed { who, .. } => {
                $indexer.index_event_account_id(who, $block_number, $event_index);
            }
            <$event_enum>::Minted { who, .. } => {
                $indexer.index_event_account_id(who, $block_number, $event_index);
            }
            <$event_enum>::Burned { who, .. } => {
                $indexer.index_event_account_id(who, $block_number, $event_index);
            }
            <$event_enum>::Suspended { who, .. } => {
                $indexer.index_event_account_id(who, $block_number, $event_index);
            }
            <$event_enum>::Restored { who, .. } => {
                $indexer.index_event_account_id(who, $block_number, $event_index);
            }
            <$event_enum>::Upgraded { who, .. } => {
                $indexer.index_event_account_id(who, $block_number, $event_index);
            }
            <$event_enum>::Locked { who, .. } => {
                $indexer.index_event_account_id(who, $block_number, $event_index);
            }
            <$event_enum>::Unlocked { who, .. } => {
                $indexer.index_event_account_id(who, $block_number, $event_index);
            }
            <$event_enum>::Frozen { who, .. } => {
                $indexer.index_event_account_id(who, $block_number, $event_index);
            }
            <$event_enum>::Thawed { who, .. } => {
                $indexer.index_event_account_id(who, $block_number, $event_index);
            }
            _ => {}
        }
    };
}
