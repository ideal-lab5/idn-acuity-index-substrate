use crate::shared::*;
use crate::substrate::*;

pub fn bags_list_index_event<R: RuntimeIndexer, E: std::fmt::Debug>(
    indexer: &Indexer<R>,
    block_number: u32,
    event_index: u32,
    event: E,
) -> Result<(), subxt::Error> {
    println!("Event: {:?}", event);

    /*
        match event {
            BagsList::Rebagged { who, from, to } => {
                indexer.index_event_account_id(who, block_number, event_index);
            }
              E::ScoreUpdated { who, new_score } => {
                  indexer.index_event_account_id(who, block_number, event_index);
              }
        }
    */

    Ok(())
}
