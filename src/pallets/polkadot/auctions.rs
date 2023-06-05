use subxt::PolkadotConfig;
use crate::shared::*;
use crate::substrate::*;

pub fn auctions_index_event(indexer: &Indexer, block_number: u32, event_index: u32, event: subxt::events::EventDetails<PolkadotConfig>) -> Result<(), subxt::Error> {
    match event.variant_name() {
        "AuctionStarted" => {
            let event = event.as_event::<polkadot::auctions::events::AuctionStarted>()?.unwrap();
            indexer.index_event_auction_index(event.auction_index, block_number, event_index);
            Ok(())
        },
        "AuctionClosed" => {
            let event = event.as_event::<polkadot::auctions::events::AuctionClosed>()?.unwrap();
            indexer.index_event_auction_index(event.auction_index, block_number, event_index);
            Ok(())
        },
        "Reserved" => {
            let event = event.as_event::<polkadot::auctions::events::Reserved>()?.unwrap();
            indexer.index_event_account_id(event.bidder, block_number, event_index);
            Ok(())
        },
        "Unreserved" => {
            let event = event.as_event::<polkadot::auctions::events::Unreserved>()?.unwrap();
            indexer.index_event_account_id(event.bidder, block_number, event_index);
            Ok(())
        },
        "ReserveConfiscated" => {
            let event = event.as_event::<polkadot::auctions::events::ReserveConfiscated>()?.unwrap();
            indexer.index_event_para_id(event.para_id.0, block_number, event_index);
            indexer.index_event_account_id(event.leaser, block_number, event_index);
            Ok(())
        },
        "BidAccepted" => {
            let event = event.as_event::<polkadot::auctions::events::BidAccepted>()?.unwrap();
            indexer.index_event_account_id(event.bidder, block_number, event_index);
            indexer.index_event_para_id(event.para_id.0, block_number, event_index);
            Ok(())
        },
        "WinningOffset" => {
            let event = event.as_event::<polkadot::auctions::events::WinningOffset>()?.unwrap();
            indexer.index_event_auction_index(event.auction_index, block_number, event_index);
            Ok(())
        },
        _ => Ok(()),
    }
}
