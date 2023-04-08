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
pub enum Bounties {
    #[serde(rename_all = "camelCase")]
	BountyProposed {
	    index: u32,
	},
    #[serde(rename_all = "camelCase")]
	BountyRejected {
	    index: u32,
	    bond: u128,
	},
    #[serde(rename_all = "camelCase")]
	BountyBecameActive {
	    index: u32,
	},
    #[serde(rename_all = "camelCase")]
	BountyAwarded {
	    index: u32,
	    beneficiary: AccountId32,
	},
    #[serde(rename_all = "camelCase")]
	BountyClaimed {
	    index: u32,
	    payout: u128,
	    beneficiary: AccountId32,
	},
    #[serde(rename_all = "camelCase")]
	BountyCanceled {
	    index: u32,
	},
    #[serde(rename_all = "camelCase")]
	BountyExtended {
	    index: u32,
	},
}

pub fn bounties_index_event(trees: Trees, block_number: u32, event_index: u32, event: subxt::events::EventDetails) -> Result<(), subxt::Error> {
    match event.variant_name() {
        "BountyProposed" => {
            let event = event.as_event::<polkadot::bounties::events::BountyProposed>()?.unwrap();
            let event_db = Event::Bounties(
                Bounties::BountyProposed {
	                index: event.index,
                }
            );
            let value = Event::encode(&event_db);
            index_event_bounty_index(trees.clone(), event.index, block_number, event_index, &value);
            Ok(())
        },
        "BountyRejected" => {
            let event = event.as_event::<polkadot::bounties::events::BountyRejected>()?.unwrap();
            let event_db = Event::Bounties(
                Bounties::BountyRejected {
	                index: event.index,
	                bond: event.bond,
                }
            );
            let value = Event::encode(&event_db);
            index_event_bounty_index(trees.clone(), event.index, block_number, event_index, &value);
            Ok(())
        },
        "BountyBecameActive" => {
            let event = event.as_event::<polkadot::bounties::events::BountyBecameActive>()?.unwrap();
            let event_db = Event::Bounties(
                Bounties::BountyBecameActive {
	                index: event.index,
                }
            );
            let value = Event::encode(&event_db);
            index_event_bounty_index(trees.clone(), event.index, block_number, event_index, &value);
            Ok(())
        },
        "BountyAwarded" => {
            let event = event.as_event::<polkadot::bounties::events::BountyAwarded>()?.unwrap();
            let event_db = Event::Bounties(
                Bounties::BountyAwarded {
	                index: event.index,
	                beneficiary: event.beneficiary.clone(),
                }
            );
            let value = Event::encode(&event_db);
            index_event_bounty_index(trees.clone(), event.index, block_number, event_index, &value);
            index_event_account_id(trees.clone(), event.beneficiary, block_number, event_index, &value);
            Ok(())
        },
        "BountyClaimed" => {
            let event = event.as_event::<polkadot::bounties::events::BountyClaimed>()?.unwrap();
            let event_db = Event::Bounties(
                Bounties::BountyClaimed {
	                index: event.index,
	                payout: event.payout,
	                beneficiary: event.beneficiary.clone(),
                }
            );
            let value = Event::encode(&event_db);
            index_event_bounty_index(trees.clone(), event.index, block_number, event_index, &value);
            index_event_account_id(trees.clone(), event.beneficiary, block_number, event_index, &value);
            Ok(())
        },
        "BountyCanceled" => {
            let event = event.as_event::<polkadot::bounties::events::BountyCanceled>()?.unwrap();
            let event_db = Event::Bounties(
                Bounties::BountyCanceled {
	                index: event.index,
                }
            );
            let value = Event::encode(&event_db);
            index_event_bounty_index(trees.clone(), event.index, block_number, event_index, &value);
            Ok(())
        },
        "BountyExtended" => {
            let event = event.as_event::<polkadot::bounties::events::BountyExtended>()?.unwrap();
            let event_db = Event::Bounties(
                Bounties::BountyExtended {
	                index: event.index,
                }
            );
            let value = Event::encode(&event_db);
            index_event_bounty_index(trees.clone(), event.index, block_number, event_index, &value);
            Ok(())
        },
        _ => Ok(()),
    }
}
