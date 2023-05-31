use subxt::PolkadotConfig;
use crate::shared::*;
use crate::substrate::*;

pub fn auctions_index_event(trees: Trees, block_number: u32, event_index: u32, event: subxt::events::EventDetails<PolkadotConfig>) -> Result<(), subxt::Error> {
    match event.variant_name() {
        "AuctionStarted" => {
            let event = event.as_event::<polkadot::auctions::events::AuctionStarted>()?.unwrap();
            index_event_auction_index(trees, event.auction_index, block_number, event_index);
            Ok(())
        },
        "AuctionClosed" => {
            let event = event.as_event::<polkadot::auctions::events::AuctionClosed>()?.unwrap();
            index_event_auction_index(trees, event.auction_index, block_number, event_index);
            Ok(())
        },
        "Reserved" => {
            let event = event.as_event::<polkadot::auctions::events::Reserved>()?.unwrap();
            index_event_account_id(trees, event.bidder, block_number, event_index);
            Ok(())
        },
        "Unreserved" => {
            let event = event.as_event::<polkadot::auctions::events::Unreserved>()?.unwrap();
            index_event_account_id(trees, event.bidder, block_number, event_index);
            Ok(())
        },
        "ReserveConfiscated" => {
            let event = event.as_event::<polkadot::auctions::events::ReserveConfiscated>()?.unwrap();
            index_event_para_id(trees.clone(), event.para_id.0, block_number, event_index);
            index_event_account_id(trees, event.leaser, block_number, event_index);
            Ok(())
        },
        "BidAccepted" => {
            let event = event.as_event::<polkadot::auctions::events::BidAccepted>()?.unwrap();
            index_event_account_id(trees.clone(), event.bidder, block_number, event_index);
            index_event_para_id(trees, event.para_id.0, block_number, event_index);
            Ok(())
        },
        "WinningOffset" => {
            let event = event.as_event::<polkadot::auctions::events::WinningOffset>()?.unwrap();
            index_event_auction_index(trees, event.auction_index, block_number, event_index);
            Ok(())
        },
        _ => Ok(()),
    }
}
