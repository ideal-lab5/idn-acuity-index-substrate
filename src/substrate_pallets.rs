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
