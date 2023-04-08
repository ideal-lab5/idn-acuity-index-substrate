use subxt::{
    utils::AccountId32,
};

use parity_scale_codec::{Encode, Decode};
use serde::{Serialize, Deserialize};

use crate::shared::*;
use crate::substrate::*;

#[derive(Encode, Decode, Serialize, Deserialize, Debug, Clone)]
pub enum ElectionCompute {
	OnChain,
	Signed,
	Unsigned,
	Fallback,
	Emergency,
}

use crate::shared::polkadot::runtime_types::pallet_election_provider_multi_phase::ElectionCompute as SubElectionCompute;

impl From<SubElectionCompute> for ElectionCompute {
    fn from(x: SubElectionCompute) -> Self {
        match x {
            SubElectionCompute::OnChain => ElectionCompute::OnChain,
            SubElectionCompute::Signed => ElectionCompute::Signed,
            SubElectionCompute::Unsigned => ElectionCompute::Unsigned,
            SubElectionCompute::Fallback => ElectionCompute::Fallback,
            SubElectionCompute::Emergency => ElectionCompute::Emergency,
        }
    }
}

#[derive(Encode, Decode, Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "variant", content = "details")]
pub enum ElectionProviderMultiPhase {
    #[serde(rename_all = "camelCase")]
	SolutionStored {
		compute: ElectionCompute,
		origin: Option<AccountId32>,
		prev_ejected: bool,
	},
    #[serde(rename_all = "camelCase")]
	Rewarded {
	    account: AccountId32,
	    value: u128,
	},
    #[serde(rename_all = "camelCase")]
	Slashed {
	    account: AccountId32,
	    value: u128,
	},
}

pub fn election_provider_multi_phase_index_event(trees: Trees, block_number: u32, event_index: u32, event: subxt::events::EventDetails) -> Result<(), subxt::Error> {
    match event.variant_name() {
        "SolutionStored" => {
            let event = event.as_event::<polkadot::election_provider_multi_phase::events::SolutionStored>()?.unwrap();
            let event_db = Event::ElectionProviderMultiPhase(
                ElectionProviderMultiPhase::SolutionStored {
		            compute: event.compute.into(),
		            origin: event.origin.clone(),
		            prev_ejected: event.prev_ejected,
                }
            );
            let value = Event::encode(&event_db);
            match event.origin {
                Some(account) => index_event_account_id(trees.clone(), account, block_number, event_index, &value),
                None => {},
            }
            Ok(())
        },
        "Rewarded" => {
            let event = event.as_event::<polkadot::election_provider_multi_phase::events::Rewarded>()?.unwrap();
            let event_db = Event::ElectionProviderMultiPhase(
                ElectionProviderMultiPhase::Rewarded {
	                account: event.account.clone(),
	                value: event.value,
                }
            );
            let value = Event::encode(&event_db);
            index_event_account_id(trees.clone(), event.account, block_number, event_index, &value);
            Ok(())
        },
        "Slashed" => {
            let event = event.as_event::<polkadot::election_provider_multi_phase::events::Slashed>()?.unwrap();
            let event_db = Event::ElectionProviderMultiPhase(
                ElectionProviderMultiPhase::Slashed {
	                account: event.account.clone(),
	                value: event.value,
                }
            );
            let value = Event::encode(&event_db);
            index_event_account_id(trees.clone(), event.account, block_number, event_index, &value);
            Ok(())
        },
        _ => Ok(()),
    }
}
