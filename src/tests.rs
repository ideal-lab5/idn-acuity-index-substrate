use crate::shared::*;
use crate::substrate::*;
use crate::websockets::*;
use crate::*;

use subxt::utils::AccountId32;

use hex_literal::hex;
use std::str::FromStr;
use zerocopy::{AsBytes, FromBytes};

pub struct TestIndexer;

impl RuntimeIndexer for TestIndexer {
    type RuntimeConfig = subxt::PolkadotConfig;

    fn get_name() -> &'static str {
        "test"
    }

    fn get_genesis_hash() -> <Self::RuntimeConfig as subxt::Config>::Hash {
        hex!["91b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3"].into()
    }

    fn get_versions() -> &'static [u32] {
        &[0]
    }

    fn get_default_url() -> &'static str {
        ""
    }

    fn process_event(
        _indexer: &Indexer<Self>,
        _block_number: u32,
        _event_index: u16,
        _event: subxt::events::EventDetails<Self::RuntimeConfig>,
    ) -> Result<u32, IndexError> {
        Ok(0)
    }
}

pub struct TestIndexer2;

impl RuntimeIndexer for TestIndexer2 {
    type RuntimeConfig = subxt::PolkadotConfig;

    fn get_name() -> &'static str {
        "test"
    }

    fn get_genesis_hash() -> <Self::RuntimeConfig as subxt::Config>::Hash {
        hex!["91b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3"].into()
    }

    fn get_versions() -> &'static [u32] {
        &[0, 500]
    }

    fn get_default_url() -> &'static str {
        ""
    }

    fn process_event(
        _indexer: &Indexer<Self>,
        _block_number: u32,
        _event_index: u16,
        _event: subxt::events::EventDetails<Self::RuntimeConfig>,
    ) -> Result<u32, IndexError> {
        Ok(0)
    }
}

#[test]
fn test_variant_key() {
    let key1 = VariantKey {
        pallet_index: 3,
        variant_index: 65,
        block_number: 4.into(),
        event_index: 5.into(),
    };

    let key2 = VariantKey::read_from(key1.as_bytes()).unwrap();
    assert_eq!(key1, key2);
}

#[test]
fn test_index_event_variant() {
    let trees = open_trees("target/debug/test_variant".into()).unwrap();
    let indexer = Indexer::<TestIndexer>::new_test(trees.clone());
    indexer.index_event_variant(3, 65, 4, 5).unwrap();

    let key1 = VariantKey {
        pallet_index: 3,
        variant_index: 65,
        block_number: 4.into(),
        event_index: 5.into(),
    };

    let k = trees.variant.scan_prefix([3, 65]).keys().next().unwrap();
    let key2 = VariantKey::read_from(&k.unwrap()).unwrap();
    assert_eq!(key1, key2);
}

#[tokio::test]
async fn test_process_msg_variant() {
    let trees = open_trees("target/debug/test_process_msg_variant".into()).unwrap();
    let indexer = Indexer::<TestIndexer>::new_test(trees.clone());
    indexer.index_event_variant(3, 65, 4, 5).unwrap();
    indexer.index_event_variant(3, 65, 8, 5).unwrap();
    indexer.index_event_variant(3, 65, 10, 5).unwrap();

    let response = process_msg_get_events(&trees, Key::Variant(3, 65));

    let ResponseMessage::Events {
        key: Key::Variant(pallet_id, variant_id),
        events,
    } = response
    else {
        panic!("Wrong response message.");
    };
    assert_eq!(pallet_id, 3);
    assert_eq!(variant_id, 65);
    assert_eq!(events.len(), 3);
    assert_eq!(events[0].block_number, 10);
    assert_eq!(events[1].block_number, 8);
    assert_eq!(events[2].block_number, 4);
}

#[test]
fn test_bytes32_key() {
    let key1 = Bytes32Key {
        key: AccountId32::from_str("5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY")
            .unwrap()
            .0,
        block_number: 4.into(),
        event_index: 5.into(),
    };

    let key2 = Bytes32Key::read_from(key1.as_bytes()).unwrap();
    assert_eq!(key1, key2);
}

#[test]
fn test_index_event_account_id() {
    let trees = open_trees("target/debug/test_account_id".into()).unwrap();
    let indexer = Indexer::<TestIndexer>::new_test(trees.clone());
    let account_id =
        AccountId32::from_str("5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY").unwrap();
    indexer
        .index_event_account_id(account_id.clone(), 4, 5)
        .unwrap();

    let key1 = Bytes32Key {
        key: account_id.0,
        block_number: 4.into(),
        event_index: 5.into(),
    };

    let k = trees
        .account_id
        .scan_prefix(account_id)
        .keys()
        .next()
        .unwrap();
    let key2 = Bytes32Key::read_from(&k.unwrap()).unwrap();
    assert_eq!(key1, key2);
}

#[tokio::test]
async fn test_process_msg_account_id() {
    let trees = open_trees("target/debug/test_process_msg_account_id".into()).unwrap();
    let indexer = Indexer::<TestIndexer>::new_test(trees.clone());
    let account_id =
        AccountId32::from_str("5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY").unwrap();
    indexer
        .index_event_account_id(account_id.clone(), 4, 5)
        .unwrap();
    indexer
        .index_event_account_id(account_id.clone(), 8, 5)
        .unwrap();
    indexer
        .index_event_account_id(account_id.clone(), 10, 5)
        .unwrap();

    let response = process_msg_get_events(&trees, Key::AccountId(Bytes32(account_id.0)));

    let ResponseMessage::Events {
        key: Key::AccountId(response_account_id),
        events,
    } = response
    else {
        panic!("Wrong response message.");
    };
    assert_eq!(Bytes32(account_id.0), response_account_id);
    assert_eq!(events.len(), 3);
    assert_eq!(events[0].block_number, 10);
    assert_eq!(events[1].block_number, 8);
    assert_eq!(events[2].block_number, 4);
}

#[test]
fn test_u32_key() {
    let key1 = U32Key {
        key: 8.into(),
        block_number: 4.into(),
        event_index: 5.into(),
    };

    let key2 = U32Key::read_from(key1.as_bytes()).unwrap();
    assert_eq!(key1, key2);
}

#[test]
fn test_index_event_account_index() {
    let trees = open_trees("target/debug/test_account_index".into()).unwrap();
    let indexer = Indexer::<TestIndexer>::new_test(trees.clone());
    indexer.index_event_account_index(8, 4, 5).unwrap();

    let key1 = U32Key {
        key: 8.into(),
        block_number: 4.into(),
        event_index: 5.into(),
    };

    let k = trees
        .account_index
        .scan_prefix(8_u32.to_be_bytes())
        .keys()
        .next()
        .unwrap();
    let key2 = U32Key::read_from(&k.unwrap()).unwrap();
    assert_eq!(key1, key2);
}

#[tokio::test]
async fn test_process_msg_account_index() {
    let trees = open_trees("target/debug/test_process_msg_account_index".into()).unwrap();
    let indexer = Indexer::<TestIndexer>::new_test(trees.clone());
    let account_index = 88;
    indexer
        .index_event_account_index(account_index, 4, 5)
        .unwrap();
    indexer
        .index_event_account_index(account_index, 8, 5)
        .unwrap();
    indexer
        .index_event_account_index(account_index, 10, 5)
        .unwrap();

    let response = process_msg_get_events(&trees, Key::AccountIndex(account_index));

    let ResponseMessage::Events {
        key: Key::AccountIndex(response_account_index),
        events,
    } = response
    else {
        panic!("Wrong response message.");
    };
    assert_eq!(account_index, response_account_index);
    assert_eq!(events.len(), 3);
    assert_eq!(events[0].block_number, 10);
    assert_eq!(events[1].block_number, 8);
    assert_eq!(events[2].block_number, 4);
}

#[test]
fn test_index_event_auction_index() {
    let trees = open_trees("target/debug/test_auction_index".into()).unwrap();
    let indexer = Indexer::<TestIndexer>::new_test(trees.clone());
    indexer.index_event_auction_index(8, 4, 5).unwrap();

    let key1 = U32Key {
        key: 8.into(),
        block_number: 4.into(),
        event_index: 5.into(),
    };

    let k = trees
        .auction_index
        .scan_prefix(8_u32.to_be_bytes())
        .keys()
        .next()
        .unwrap();
    let key2 = U32Key::read_from(&k.unwrap()).unwrap();
    assert_eq!(key1, key2);
}

#[tokio::test]
async fn test_process_msg_auction_index() {
    let trees = open_trees("target/debug/test_process_msg_auction_index".into()).unwrap();
    let indexer = Indexer::<TestIndexer>::new_test(trees.clone());
    let auction_index = 88;
    indexer
        .index_event_auction_index(auction_index, 4, 5)
        .unwrap();
    indexer
        .index_event_auction_index(auction_index, 8, 5)
        .unwrap();
    indexer
        .index_event_auction_index(auction_index, 10, 5)
        .unwrap();

    let response = process_msg_get_events(&trees, Key::AuctionIndex(auction_index));

    let ResponseMessage::Events {
        key: Key::AuctionIndex(response_auction_index),
        events,
    } = response
    else {
        panic!("Wrong response message.");
    };
    assert_eq!(auction_index, response_auction_index);
    assert_eq!(events.len(), 3);
    assert_eq!(events[0].block_number, 10);
    assert_eq!(events[1].block_number, 8);
    assert_eq!(events[2].block_number, 4);
}

#[test]
fn test_index_event_bounty_index() {
    let trees = open_trees("target/debug/test_bounty_index".into()).unwrap();
    let indexer = Indexer::<TestIndexer>::new_test(trees.clone());
    indexer.index_event_bounty_index(8, 4, 5).unwrap();

    let key1 = U32Key {
        key: 8.into(),
        block_number: 4.into(),
        event_index: 5.into(),
    };

    let k = trees
        .bounty_index
        .scan_prefix(8_u32.to_be_bytes())
        .keys()
        .next()
        .unwrap();
    let key2 = U32Key::read_from(&k.unwrap()).unwrap();
    assert_eq!(key1, key2);
}

#[tokio::test]
async fn test_process_msg_bounty_index() {
    let trees = open_trees("target/debug/test_process_msg_bounty_index".into()).unwrap();
    let indexer = Indexer::<TestIndexer>::new_test(trees.clone());
    let bounty_index = 88;
    indexer
        .index_event_bounty_index(bounty_index, 4, 5)
        .unwrap();
    indexer
        .index_event_bounty_index(bounty_index, 8, 5)
        .unwrap();
    indexer
        .index_event_bounty_index(bounty_index, 10, 5)
        .unwrap();

    let response = process_msg_get_events(&trees, Key::BountyIndex(bounty_index));

    let ResponseMessage::Events {
        key: Key::BountyIndex(response_bounty_index),
        events,
    } = response
    else {
        panic!("Wrong response message.");
    };
    assert_eq!(bounty_index, response_bounty_index);
    assert_eq!(events.len(), 3);
    assert_eq!(events[0].block_number, 10);
    assert_eq!(events[1].block_number, 8);
    assert_eq!(events[2].block_number, 4);
}

#[test]
fn test_index_event_candidate_hash() {
    let trees = open_trees("target/debug/test_candidate_hash".into()).unwrap();
    let indexer = Indexer::<TestIndexer>::new_test(trees.clone());
    indexer.index_event_candidate_hash([8; 32], 4, 5).unwrap();

    let key1 = Bytes32Key {
        key: [8; 32],
        block_number: 4.into(),
        event_index: 5.into(),
    };

    let k = trees
        .candidate_hash
        .scan_prefix([8; 32])
        .keys()
        .next()
        .unwrap();
    let key2 = Bytes32Key::read_from(&k.unwrap()).unwrap();
    assert_eq!(key1, key2);
}

#[tokio::test]
async fn test_process_msg_candidate_hash() {
    let trees = open_trees("target/debug/test_process_msg_candidate_hash".into()).unwrap();
    let indexer = Indexer::<TestIndexer>::new_test(trees.clone());
    let candidate_hash = Bytes32([8; 32]);
    indexer
        .index_event_candidate_hash(candidate_hash.0, 4, 5)
        .unwrap();
    indexer
        .index_event_candidate_hash(candidate_hash.0, 8, 5)
        .unwrap();
    indexer
        .index_event_candidate_hash(candidate_hash.0, 10, 5)
        .unwrap();

    let response = process_msg_get_events(&trees, Key::CandidateHash(candidate_hash));

    let ResponseMessage::Events {
        key: Key::CandidateHash(response_candidate_hash),
        events,
    } = response
    else {
        panic!("Wrong response message.");
    };
    assert_eq!(candidate_hash, response_candidate_hash);
    assert_eq!(events.len(), 3);
    assert_eq!(events[0].block_number, 10);
    assert_eq!(events[1].block_number, 8);
    assert_eq!(events[2].block_number, 4);
}

#[test]
fn test_index_event_era_index() {
    let trees = open_trees("target/debug/test_era_index".into()).unwrap();
    let indexer = Indexer::<TestIndexer>::new_test(trees.clone());
    indexer.index_event_era_index(8, 4, 5).unwrap();

    let key1 = U32Key {
        key: 8.into(),
        block_number: 4.into(),
        event_index: 5.into(),
    };

    let k = trees
        .era_index
        .scan_prefix(8_u32.to_be_bytes())
        .keys()
        .next()
        .unwrap();
    let key2 = U32Key::read_from(&k.unwrap()).unwrap();
    assert_eq!(key1, key2);
}

#[tokio::test]
async fn test_process_msg_era_index() {
    let trees = open_trees("target/debug/test_process_msg_era_index".into()).unwrap();
    let indexer = Indexer::<TestIndexer>::new_test(trees.clone());
    let era_index = 88;
    indexer.index_event_era_index(era_index, 4, 5).unwrap();
    indexer.index_event_era_index(era_index, 8, 5).unwrap();
    indexer.index_event_era_index(era_index, 10, 5).unwrap();

    let response = process_msg_get_events(&trees, Key::EraIndex(era_index));

    let ResponseMessage::Events {
        key: Key::EraIndex(response_era_index),
        events,
    } = response
    else {
        panic!("Wrong response message.");
    };
    assert_eq!(era_index, response_era_index);
    assert_eq!(events.len(), 3);
    assert_eq!(events[0].block_number, 10);
    assert_eq!(events[1].block_number, 8);
    assert_eq!(events[2].block_number, 4);
}

#[test]
fn test_index_event_message_id() {
    let trees = open_trees("target/debug/test_message_id".into()).unwrap();
    let indexer = Indexer::<TestIndexer>::new_test(trees.clone());
    indexer.index_event_message_id([8; 32], 4, 5).unwrap();

    let key1 = Bytes32Key {
        key: [8; 32],
        block_number: 4.into(),
        event_index: 5.into(),
    };

    let k = trees.message_id.scan_prefix([8; 32]).keys().next().unwrap();
    let key2 = Bytes32Key::read_from(&k.unwrap()).unwrap();
    assert_eq!(key1, key2);
}

#[tokio::test]
async fn test_process_msg_message_id() {
    let trees = open_trees("target/debug/test_process_msg_message_id".into()).unwrap();
    let indexer = Indexer::<TestIndexer>::new_test(trees.clone());
    let message_id = Bytes32([8; 32]);
    indexer.index_event_message_id(message_id.0, 4, 5).unwrap();
    indexer.index_event_message_id(message_id.0, 8, 5).unwrap();
    indexer.index_event_message_id(message_id.0, 10, 5).unwrap();

    let response = process_msg_get_events(&trees, Key::MessageId(message_id));

    let ResponseMessage::Events {
        key: Key::MessageId(response_message_id),
        events,
    } = response
    else {
        panic!("Wrong response message.");
    };
    assert_eq!(message_id, response_message_id);
    assert_eq!(events.len(), 3);
    assert_eq!(events[0].block_number, 10);
    assert_eq!(events[1].block_number, 8);
    assert_eq!(events[2].block_number, 4);
}

#[test]
fn test_index_event_para_id() {
    let trees = open_trees("target/debug/test_para_id".into()).unwrap();
    let indexer = Indexer::<TestIndexer>::new_test(trees.clone());
    indexer.index_event_para_id(8, 4, 5).unwrap();

    let key1 = U32Key {
        key: 8.into(),
        block_number: 4.into(),
        event_index: 5.into(),
    };

    let k = trees
        .para_id
        .scan_prefix(8_u32.to_be_bytes())
        .keys()
        .next()
        .unwrap();
    let key2 = U32Key::read_from(&k.unwrap()).unwrap();
    assert_eq!(key1, key2);
}

#[tokio::test]
async fn test_process_msg_para_id() {
    let trees = open_trees("target/debug/test_process_msg_para_id".into()).unwrap();
    let indexer = Indexer::<TestIndexer>::new_test(trees.clone());
    let para_id = 88;
    indexer.index_event_para_id(para_id, 4, 5).unwrap();
    indexer.index_event_para_id(para_id, 8, 5).unwrap();
    indexer.index_event_para_id(para_id, 10, 5).unwrap();

    let response = process_msg_get_events(&trees, Key::ParaId(para_id));

    let ResponseMessage::Events {
        key: Key::ParaId(response_para_id),
        events,
    } = response
    else {
        panic!("Wrong response message.");
    };
    assert_eq!(para_id, response_para_id);
    assert_eq!(events.len(), 3);
    assert_eq!(events[0].block_number, 10);
    assert_eq!(events[1].block_number, 8);
    assert_eq!(events[2].block_number, 4);
}

#[test]
fn test_index_event_pool_id() {
    let trees = open_trees("target/debug/test_pool_id".into()).unwrap();
    let indexer = Indexer::<TestIndexer>::new_test(trees.clone());
    indexer.index_event_pool_id(8, 4, 5).unwrap();

    let key1 = U32Key {
        key: 8.into(),
        block_number: 4.into(),
        event_index: 5.into(),
    };

    let k = trees
        .pool_id
        .scan_prefix(8_u32.to_be_bytes())
        .keys()
        .next()
        .unwrap();
    let key2 = U32Key::read_from(&k.unwrap()).unwrap();
    assert_eq!(key1, key2);
}

#[tokio::test]
async fn test_process_msg_pool_id() {
    let trees = open_trees("target/debug/test_process_msg_pool_id".into()).unwrap();
    let indexer = Indexer::<TestIndexer>::new_test(trees.clone());
    let pool_id = 88;
    indexer.index_event_pool_id(pool_id, 4, 5).unwrap();
    indexer.index_event_pool_id(pool_id, 8, 5).unwrap();
    indexer.index_event_pool_id(pool_id, 10, 5).unwrap();

    let response = process_msg_get_events(&trees, Key::PoolId(pool_id));

    let ResponseMessage::Events {
        key: Key::PoolId(response_pool_id),
        events,
    } = response
    else {
        panic!("Wrong response message.");
    };
    assert_eq!(pool_id, response_pool_id);
    assert_eq!(events.len(), 3);
    assert_eq!(events[0].block_number, 10);
    assert_eq!(events[1].block_number, 8);
    assert_eq!(events[2].block_number, 4);
}

#[test]
fn test_index_event_preimage_hash() {
    let trees = open_trees("target/debug/test_preimage_hash".into()).unwrap();
    let indexer = Indexer::<TestIndexer>::new_test(trees.clone());
    indexer.index_event_preimage_hash([8; 32], 4, 5).unwrap();

    let key1 = Bytes32Key {
        key: [8; 32],
        block_number: 4.into(),
        event_index: 5.into(),
    };

    let k = trees
        .preimage_hash
        .scan_prefix([8; 32])
        .keys()
        .next()
        .unwrap();
    let key2 = Bytes32Key::read_from(&k.unwrap()).unwrap();
    assert_eq!(key1, key2);
}

#[tokio::test]
async fn test_process_msg_preimage_hash() {
    let trees = open_trees("target/debug/test_process_msg_preimage_hash".into()).unwrap();
    let indexer = Indexer::<TestIndexer>::new_test(trees.clone());
    let preimage_hash = Bytes32([8; 32]);
    indexer
        .index_event_preimage_hash(preimage_hash.0, 4, 5)
        .unwrap();
    indexer
        .index_event_preimage_hash(preimage_hash.0, 8, 5)
        .unwrap();
    indexer
        .index_event_preimage_hash(preimage_hash.0, 10, 5)
        .unwrap();

    let response = process_msg_get_events(&trees, Key::PreimageHash(preimage_hash));

    let ResponseMessage::Events {
        key: Key::PreimageHash(response_preimage_hash),
        events,
    } = response
    else {
        panic!("Wrong response message.");
    };
    assert_eq!(preimage_hash, response_preimage_hash);
    assert_eq!(events.len(), 3);
    assert_eq!(events[0].block_number, 10);
    assert_eq!(events[1].block_number, 8);
    assert_eq!(events[2].block_number, 4);
}

#[test]
fn test_index_event_proposal_hash() {
    let trees = open_trees("target/debug/test_proposal_hash".into()).unwrap();
    let indexer = Indexer::<TestIndexer>::new_test(trees.clone());
    indexer.index_event_proposal_hash([8; 32], 4, 5).unwrap();

    let key1 = Bytes32Key {
        key: [8; 32],
        block_number: 4.into(),
        event_index: 5.into(),
    };

    let k = trees
        .proposal_hash
        .scan_prefix([8; 32])
        .keys()
        .next()
        .unwrap();
    let key2 = Bytes32Key::read_from(&k.unwrap()).unwrap();
    assert_eq!(key1, key2);
}

#[tokio::test]
async fn test_process_msg_proposal_hash() {
    let trees = open_trees("target/debug/test_process_msg_proposal_hash".into()).unwrap();
    let indexer = Indexer::<TestIndexer>::new_test(trees.clone());
    let proposal_hash = Bytes32([8; 32]);
    indexer
        .index_event_proposal_hash(proposal_hash.0, 4, 5)
        .unwrap();
    indexer
        .index_event_proposal_hash(proposal_hash.0, 8, 5)
        .unwrap();
    indexer
        .index_event_proposal_hash(proposal_hash.0, 10, 5)
        .unwrap();

    let response = process_msg_get_events(&trees, Key::ProposalHash(proposal_hash));

    let ResponseMessage::Events {
        key: Key::ProposalHash(response_proposal_hash),
        events,
    } = response
    else {
        panic!("Wrong response message.");
    };
    assert_eq!(proposal_hash, response_proposal_hash);
    assert_eq!(events.len(), 3);
    assert_eq!(events[0].block_number, 10);
    assert_eq!(events[1].block_number, 8);
    assert_eq!(events[2].block_number, 4);
}

#[test]
fn test_index_event_proposal_index() {
    let trees = open_trees("target/debug/test_proposal_index".into()).unwrap();
    let indexer = Indexer::<TestIndexer>::new_test(trees.clone());
    indexer.index_event_proposal_index(8, 4, 5).unwrap();

    let key1 = U32Key {
        key: 8.into(),
        block_number: 4.into(),
        event_index: 5.into(),
    };

    let k = trees
        .proposal_index
        .scan_prefix(8_u32.to_be_bytes())
        .keys()
        .next()
        .unwrap();
    let key2 = U32Key::read_from(&k.unwrap()).unwrap();
    assert_eq!(key1, key2);
}

#[tokio::test]
async fn test_process_msg_proposal_index() {
    let trees = open_trees("target/debug/test_process_msg_proposal_index".into()).unwrap();
    let indexer = Indexer::<TestIndexer>::new_test(trees.clone());
    let proposal_index = 88;
    indexer
        .index_event_proposal_index(proposal_index, 4, 5)
        .unwrap();
    indexer
        .index_event_proposal_index(proposal_index, 8, 5)
        .unwrap();
    indexer
        .index_event_proposal_index(proposal_index, 10, 5)
        .unwrap();

    let response = process_msg_get_events(&trees, Key::ProposalIndex(proposal_index));

    let ResponseMessage::Events {
        key: Key::ProposalIndex(response_proposal_index),
        events,
    } = response
    else {
        panic!("Wrong response message.");
    };
    assert_eq!(proposal_index, response_proposal_index);
    assert_eq!(events.len(), 3);
    assert_eq!(events[0].block_number, 10);
    assert_eq!(events[1].block_number, 8);
    assert_eq!(events[2].block_number, 4);
}

#[test]
fn test_index_event_ref_index() {
    let trees = open_trees("target/debug/test_ref_index".into()).unwrap();
    let indexer = Indexer::<TestIndexer>::new_test(trees.clone());
    indexer.index_event_ref_index(8, 4, 5).unwrap();

    let key1 = U32Key {
        key: 8.into(),
        block_number: 4.into(),
        event_index: 5.into(),
    };

    let k = trees
        .ref_index
        .scan_prefix(8_u32.to_be_bytes())
        .keys()
        .next()
        .unwrap();
    let key2 = U32Key::read_from(&k.unwrap()).unwrap();
    assert_eq!(key1, key2);
}

#[tokio::test]
async fn test_process_msg_ref_index() {
    let trees = open_trees("target/debug/test_process_msg_ref_index".into()).unwrap();
    let indexer = Indexer::<TestIndexer>::new_test(trees.clone());
    let ref_index = 88;
    indexer.index_event_ref_index(ref_index, 4, 5).unwrap();
    indexer.index_event_ref_index(ref_index, 8, 5).unwrap();
    indexer.index_event_ref_index(ref_index, 10, 5).unwrap();

    let response = process_msg_get_events(&trees, Key::RefIndex(ref_index));

    let ResponseMessage::Events {
        key: Key::RefIndex(response_ref_index),
        events,
    } = response
    else {
        panic!("Wrong response message.");
    };
    assert_eq!(ref_index, response_ref_index);
    assert_eq!(events.len(), 3);
    assert_eq!(events[0].block_number, 10);
    assert_eq!(events[1].block_number, 8);
    assert_eq!(events[2].block_number, 4);
}

#[test]
fn test_index_event_registrar_index() {
    let trees = open_trees("target/debug/test_registrar_index".into()).unwrap();
    let indexer = Indexer::<TestIndexer>::new_test(trees.clone());
    indexer.index_event_registrar_index(8, 4, 5).unwrap();

    let key1 = U32Key {
        key: 8.into(),
        block_number: 4.into(),
        event_index: 5.into(),
    };

    let k = trees
        .registrar_index
        .scan_prefix(8_u32.to_be_bytes())
        .keys()
        .next()
        .unwrap();
    let key2 = U32Key::read_from(&k.unwrap()).unwrap();
    assert_eq!(key1, key2);
}

#[tokio::test]
async fn test_process_msg_registrar_index() {
    let trees = open_trees("target/debug/test_process_msg_registrar_index".into()).unwrap();
    let indexer = Indexer::<TestIndexer>::new_test(trees.clone());
    let registrar_index = 88;
    indexer
        .index_event_registrar_index(registrar_index, 4, 5)
        .unwrap();
    indexer
        .index_event_registrar_index(registrar_index, 8, 5)
        .unwrap();
    indexer
        .index_event_registrar_index(registrar_index, 10, 5)
        .unwrap();

    let response = process_msg_get_events(&trees, Key::RegistrarIndex(registrar_index));

    let ResponseMessage::Events {
        key: Key::RegistrarIndex(response_registrar_index),
        events,
    } = response
    else {
        panic!("Wrong response message.");
    };
    assert_eq!(registrar_index, response_registrar_index);
    assert_eq!(events.len(), 3);
    assert_eq!(events[0].block_number, 10);
    assert_eq!(events[1].block_number, 8);
    assert_eq!(events[2].block_number, 4);
}

#[test]
fn test_index_event_session_index() {
    let trees = open_trees("target/debug/test_session_index".into()).unwrap();
    let indexer = Indexer::<TestIndexer>::new_test(trees.clone());
    indexer.index_event_session_index(8, 4, 5).unwrap();

    let key1 = U32Key {
        key: 8.into(),
        block_number: 4.into(),
        event_index: 5.into(),
    };

    let k = trees
        .session_index
        .scan_prefix(8_u32.to_be_bytes())
        .keys()
        .next()
        .unwrap();
    let key2 = U32Key::read_from(&k.unwrap()).unwrap();
    assert_eq!(key1, key2);
}

#[tokio::test]
async fn test_process_msg_session_index() {
    let trees = open_trees("target/debug/test_process_msg_session_index".into()).unwrap();
    let indexer = Indexer::<TestIndexer>::new_test(trees.clone());
    let session_index = 88;
    indexer
        .index_event_session_index(session_index, 4, 5)
        .unwrap();
    indexer
        .index_event_session_index(session_index, 8, 5)
        .unwrap();
    indexer
        .index_event_session_index(session_index, 10, 5)
        .unwrap();

    let response = process_msg_get_events(&trees, Key::SessionIndex(session_index));

    let ResponseMessage::Events {
        key: Key::SessionIndex(response_session_index),
        events,
    } = response
    else {
        panic!("Wrong response message.");
    };
    assert_eq!(session_index, response_session_index);
    assert_eq!(events.len(), 3);
    assert_eq!(events[0].block_number, 10);
    assert_eq!(events[1].block_number, 8);
    assert_eq!(events[2].block_number, 4);
}

#[test]
fn test_index_event_tip_hash() {
    let trees = open_trees("target/debug/test_tip_hash".into()).unwrap();
    let indexer = Indexer::<TestIndexer>::new_test(trees.clone());
    indexer.index_event_tip_hash([8; 32], 4, 5).unwrap();

    let key1 = Bytes32Key {
        key: [8; 32],
        block_number: 4.into(),
        event_index: 5.into(),
    };

    let k = trees.tip_hash.scan_prefix([8; 32]).keys().next().unwrap();
    let key2 = Bytes32Key::read_from(&k.unwrap()).unwrap();
    assert_eq!(key1, key2);
}

#[tokio::test]
async fn test_process_msg_tip_hash() {
    let trees = open_trees("target/debug/test_process_msg_tip_hash".into()).unwrap();
    let indexer = Indexer::<TestIndexer>::new_test(trees.clone());
    let tip_hash = Bytes32([8; 32]);
    indexer.index_event_tip_hash(tip_hash.0, 4, 5).unwrap();
    indexer.index_event_tip_hash(tip_hash.0, 8, 5).unwrap();
    indexer.index_event_tip_hash(tip_hash.0, 10, 5).unwrap();

    let response = process_msg_get_events(&trees, Key::TipHash(tip_hash));

    let ResponseMessage::Events {
        key: Key::TipHash(response_tip_hash),
        events,
    } = response
    else {
        panic!("Wrong response message.");
    };
    assert_eq!(tip_hash, response_tip_hash);
    assert_eq!(events.len(), 3);
    assert_eq!(events[0].block_number, 10);
    assert_eq!(events[1].block_number, 8);
    assert_eq!(events[2].block_number, 4);
}

#[tokio::test]
async fn test_process_msg_status() {
    let trees = open_trees("target/debug/test_process_msg".into()).unwrap();
    trees
        .root
        .insert("last_head_block", &845433_u32.to_be_bytes())
        .unwrap();
    trees
        .root
        .insert("last_batch_block", &8445_u32.to_be_bytes())
        .unwrap();
    let response = process_msg_status(&trees);

    if let Ok(ResponseMessage::Status {
        last_head_block,
        last_batch_block,
        batch_indexing_complete,
    }) = response
    {
        assert_eq!(last_head_block, 845433);
        assert_eq!(last_batch_block, 8445);
        assert!(!batch_indexing_complete);
    }
}

#[test]
fn test_load_spans() {
    let trees = open_trees("target/debug/test_check_span".into()).unwrap();
    trees.span.clear().unwrap();
    let spans = load_spans::<TestIndexer>(&trees.span).unwrap();
    assert_eq!(trees.span.len(), 0);
    assert_eq!(spans.len(), 0);
    let value = SpanDbValue {
        start: 80_u32.into(),
        version: 0_u16.into(),
    };
    trees
        .span
        .insert(100_u32.to_be_bytes(), value.as_bytes())
        .unwrap();
    let spans = load_spans::<TestIndexer>(&trees.span).unwrap();
    assert_eq!(trees.span.len(), 1);
    assert_eq!(spans.len(), 1);
    assert_eq!(
        spans[0],
        Span {
            start: 80,
            end: 100
        }
    );
    let value = SpanDbValue {
        start: 180_u32.into(),
        version: 0_u16.into(),
    };
    trees
        .span
        .insert(200_u32.to_be_bytes(), value.as_bytes())
        .unwrap();
    let spans = load_spans::<TestIndexer>(&trees.span).unwrap();
    assert_eq!(trees.span.len(), 2);
    assert_eq!(spans.len(), 2);
    assert_eq!(
        spans[0],
        Span {
            start: 80,
            end: 100
        }
    );
    assert_eq!(
        spans[1],
        Span {
            start: 180,
            end: 200
        }
    );
    let spans = load_spans::<TestIndexer2>(&trees.span).unwrap();
    assert_eq!(trees.span.len(), 2);
    assert_eq!(spans.len(), 2);
    assert_eq!(
        spans[0],
        Span {
            start: 80,
            end: 100
        }
    );
    assert_eq!(
        spans[1],
        Span {
            start: 180,
            end: 200
        }
    );
    let value = SpanDbValue {
        start: 400_u32.into(),
        version: 0_u16.into(),
    };
    trees
        .span
        .insert(600_u32.to_be_bytes(), value.as_bytes())
        .unwrap();
    let spans = load_spans::<TestIndexer2>(&trees.span).unwrap();
    assert_eq!(trees.span.len(), 3);
    assert_eq!(spans.len(), 3);
    assert_eq!(
        spans[0],
        Span {
            start: 80,
            end: 100
        }
    );
    assert_eq!(
        spans[1],
        Span {
            start: 180,
            end: 200
        }
    );
    assert_eq!(
        spans[2],
        Span {
            start: 400,
            end: 499
        }
    );
    let value = SpanDbValue {
        start: 500_u32.into(),
        version: 0_u16.into(),
    };
    trees
        .span
        .insert(600_u32.to_be_bytes(), value.as_bytes())
        .unwrap();
    let spans = load_spans::<TestIndexer2>(&trees.span).unwrap();
    assert_eq!(trees.span.len(), 3);
    assert_eq!(spans.len(), 3);
    assert_eq!(
        spans[0],
        Span {
            start: 80,
            end: 100
        }
    );
    assert_eq!(
        spans[1],
        Span {
            start: 180,
            end: 200
        }
    );
    assert_eq!(
        spans[2],
        Span {
            start: 400,
            end: 499
        }
    );
    let value = SpanDbValue {
        start: 500_u32.into(),
        version: 1_u16.into(),
    };
    trees
        .span
        .insert(600_u32.to_be_bytes(), value.as_bytes())
        .unwrap();
    let spans = load_spans::<TestIndexer2>(&trees.span).unwrap();
    assert_eq!(trees.span.len(), 4);
    assert_eq!(spans.len(), 4);
    assert_eq!(
        spans[0],
        Span {
            start: 80,
            end: 100
        }
    );
    assert_eq!(
        spans[1],
        Span {
            start: 180,
            end: 200
        }
    );
    assert_eq!(
        spans[2],
        Span {
            start: 400,
            end: 499
        }
    );
    assert_eq!(
        spans[3],
        Span {
            start: 500,
            end: 600
        }
    );
}
