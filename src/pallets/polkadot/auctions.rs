use subxt::{
    utils::AccountId32,
};

use parity_scale_codec::{Encode, Decode};
use serde::{Serialize, Deserialize};

use crate::shared::*;
use crate::substrate::*;

#[derive(Encode, Decode, Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "variant", content = "details")]
pub enum Auctions {
    #[serde(rename_all = "camelCase")]
	AuctionStarted {
		auction_index: u32,
		lease_period: u32,
		ending: u32,
	},
    #[serde(rename_all = "camelCase")]
	AuctionClosed {
	    auction_index: u32,
	},
    #[serde(rename_all = "camelCase")]
	Reserved {
	    bidder: AccountId32,
	    extra_reserved: u128,
	    total_amount: u128,
	},
    #[serde(rename_all = "camelCase")]
	Unreserved {
	    bidder: AccountId32,
	    amount: u128,
	},
    #[serde(rename_all = "camelCase")]
	ReserveConfiscated {
	    para_id: u32,
	    leaser: AccountId32,
	    amount: u128,
	},
    #[serde(rename_all = "camelCase")]
	BidAccepted {
		bidder: AccountId32,
		para_id: u32,
		amount: u128,
		first_slot: u32,
		last_slot: u32,
	},
    #[serde(rename_all = "camelCase")]
	WinningOffset {
	    auction_index: u32,
	    block_number: u32,
	},
}

pub fn auctions_index_event(trees: Trees, block_number: u32, event_index: u32, event: subxt::events::EventDetails) -> Result<(), subxt::Error> {
    match event.variant_name() {
        "AuctionStarted" => {
            let event = event.as_event::<polkadot::auctions::events::AuctionStarted>()?.unwrap();
            let event_db = Event::Auctions(
                Auctions::AuctionStarted {
		            auction_index: event.auction_index,
		            lease_period: event.lease_period,
		            ending: event.ending,
                }
            );
            let value = Event::encode(&event_db);
            index_event_auction_index(trees.clone(), event.auction_index, block_number, event_index, &value);
            Ok(())
        },
        "AuctionClosed" => {
            let event = event.as_event::<polkadot::auctions::events::AuctionClosed>()?.unwrap();
            let event_db = Event::Auctions(
                Auctions::AuctionClosed {
		            auction_index: event.auction_index,
                }
            );
            let value = Event::encode(&event_db);
            index_event_auction_index(trees.clone(), event.auction_index, block_number, event_index, &value);
            Ok(())
        },
        "Reserved" => {
            let event = event.as_event::<polkadot::auctions::events::Reserved>()?.unwrap();
            let event_db = Event::Auctions(
                Auctions::Reserved {
	                bidder: event.bidder.clone(),
	                extra_reserved: event.extra_reserved,
	                total_amount: event.total_amount,
                }
            );
            let value = Event::encode(&event_db);
            index_event_account_id(trees.clone(), event.bidder, block_number, event_index, &value);
            Ok(())
        },
        "Unreserved" => {
            let event = event.as_event::<polkadot::auctions::events::Unreserved>()?.unwrap();
            let event_db = Event::Auctions(
                Auctions::Unreserved {
	                bidder: event.bidder.clone(),
	                amount: event.amount,
                }
            );
            let value = Event::encode(&event_db);
            index_event_account_id(trees.clone(), event.bidder, block_number, event_index, &value);
            Ok(())
        },
        "ReserveConfiscated" => {
            let event = event.as_event::<polkadot::auctions::events::ReserveConfiscated>()?.unwrap();
            let event_db = Event::Auctions(
                Auctions::ReserveConfiscated {
	                para_id: event.para_id.0,
	                leaser: event.leaser.clone(),
	                amount: event.amount,
                }
            );
            let value = Event::encode(&event_db);
            index_event_para_id(trees.clone(), event.para_id.0, block_number, event_index, &value);
            index_event_account_id(trees.clone(), event.leaser, block_number, event_index, &value);
            Ok(())
        },
        "BidAccepted" => {
            let event = event.as_event::<polkadot::auctions::events::BidAccepted>()?.unwrap();
            let event_db = Event::Auctions(
                Auctions::BidAccepted {
		            bidder: event.bidder.clone(),
		            para_id: event.para_id.0,
		            amount: event.amount,
		            first_slot: event.first_slot,
		            last_slot: event.last_slot,
                }
            );
            let value = Event::encode(&event_db);
            index_event_account_id(trees.clone(), event.bidder, block_number, event_index, &value);
            index_event_para_id(trees.clone(), event.para_id.0, block_number, event_index, &value);
            Ok(())
        },
        "WinningOffset" => {
            let event = event.as_event::<polkadot::auctions::events::WinningOffset>()?.unwrap();
            let event_db = Event::Auctions(
                Auctions::WinningOffset {
	                auction_index: event.auction_index,
	                block_number: event.block_number,
                }
            );
            let value = Event::encode(&event_db);
            index_event_auction_index(trees.clone(), event.auction_index, block_number, event_index, &value);
            Ok(())
        },
        _ => Ok(()),
    }
}
