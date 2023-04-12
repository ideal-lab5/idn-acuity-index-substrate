use crate::shared::*;
use crate::substrate::*;

use subxt::{
    utils::AccountId32,
};

use std::str::FromStr;

#[test]
fn test_account_id_key() {
    let key1: AccountIdKey = AccountIdKey {
        account_id: AccountId32::from_str("5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY").unwrap(),
        block_number: 4,
        i: 5,
    };

    let key2 = AccountIdKey::unserialize(key1.serialize());
    assert_eq!(key1, key2);
}

fn init_db(name: &str) -> Trees {
    let db = sled::open(name).unwrap();
    Trees {
        root: db.clone(),
        account_id: db.open_tree("account_id").unwrap(),
        account_index: db.open_tree("account_index").unwrap(),
        auction_index: db.open_tree("auction_index").unwrap(),
        bounty_index: db.open_tree("bounty_index").unwrap(),
        candidate_hash: db.open_tree("candiate_hash").unwrap(),
        message_id: db.open_tree("para_id").unwrap(),
        para_id: db.open_tree("para_id").unwrap(),
        pool_id: db.open_tree("bounty_index").unwrap(),
        proposal_hash: db.open_tree("proposal_hash").unwrap(),
        proposal_index: db.open_tree("proposal_index").unwrap(),
        ref_index: db.open_tree("ref_index").unwrap(),
        registrar_index: db.open_tree("registrar_index").unwrap(),
        tip_hash: db.open_tree("tip_hash").unwrap(),
    }
}

#[test]
fn test_index_event_account_id() {
    let trees = init_db("target/debug/test_account_id");
    let account_id = AccountId32::from_str("5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY").unwrap();
    index_event_account_id(trees.clone(), account_id.clone(), 4, 5, &[0; 0]);

    let key1 = AccountIdKey {
        account_id: account_id.clone(),
        block_number: 4,
        i: 5,
    };

    let k = trees.account_id.scan_prefix(account_id).keys().next().unwrap();
    let key2 = AccountIdKey::unserialize(k.unwrap().to_vec());
    assert_eq!(key1, key2);
}

#[test]
fn test_account_index_key() {
    let key1: AccountIndexKey = AccountIndexKey {
        account_index: 8,
        block_number: 4,
        i: 5,
    };

    let key2 = AccountIndexKey::unserialize(key1.serialize());
    assert_eq!(key1, key2);
}

#[test]
fn test_index_event_account_index() {
    let trees = init_db("target/debug/test_account_index");
    index_event_account_index(trees.clone(), 8, 4, 5, &[0; 0]);

    let key1 = AccountIndexKey {
        account_index: 8,
        block_number: 4,
        i: 5,
    };

    let k = trees.account_index.scan_prefix(8_u32.to_be_bytes().to_vec()).keys().next().unwrap();
    let key2 = AccountIndexKey::unserialize(k.unwrap().to_vec());
    assert_eq!(key1, key2);
}

#[test]
fn test_auction_index_key() {
    let key1: AuctionIndexKey = AuctionIndexKey {
        auction_index: 8,
        block_number: 4,
        i: 5,
    };

    let key2 = AuctionIndexKey::unserialize(key1.serialize());
    assert_eq!(key1, key2);
}

#[test]
fn test_index_event_auction_index() {
    let trees = init_db("target/debug/test_auction_index");
    index_event_auction_index(trees.clone(), 8, 4, 5, &[0; 0]);

    let key1 = AuctionIndexKey {
        auction_index: 8,
        block_number: 4,
        i: 5,
    };

    let k = trees.auction_index.scan_prefix(8_u32.to_be_bytes().to_vec()).keys().next().unwrap();
    let key2 = AuctionIndexKey::unserialize(k.unwrap().to_vec());
    assert_eq!(key1, key2);
}

#[test]
fn test_bounty_index_key() {
    let key1: BountyIndexKey = BountyIndexKey {
        bounty_index: 8,
        block_number: 4,
        i: 5,
    };

    let key2 = BountyIndexKey::unserialize(key1.serialize());
    assert_eq!(key1, key2);
}

#[test]
fn test_index_event_bounty_index() {
    let trees = init_db("target/debug/test_bounty_index");
    index_event_bounty_index(trees.clone(), 8, 4, 5, &[0; 0]);

    let key1 = BountyIndexKey {
        bounty_index: 8,
        block_number: 4,
        i: 5,
    };

    let k = trees.bounty_index.scan_prefix(8_u32.to_be_bytes().to_vec()).keys().next().unwrap();
    let key2 = BountyIndexKey::unserialize(k.unwrap().to_vec());
    assert_eq!(key1, key2);
}

#[test]
fn test_candidate_hash_key() {
    let key1: CandidateHashKey = CandidateHashKey {
        candidate_hash: [8; 32],
        block_number: 4,
        i: 5,
    };

    let key2 = CandidateHashKey::unserialize(key1.serialize());
    assert_eq!(key1, key2);
}

#[test]
fn test_index_event_candidate_hash() {
    let trees = init_db("target/debug/test_candidate_hash");
    index_event_candidate_hash(trees.clone(), [8; 32], 4, 5, &[0; 0]);

    let key1 = CandidateHashKey {
        candidate_hash: [8; 32],
        block_number: 4,
        i: 5,
    };

    let k = trees.candidate_hash.scan_prefix([8; 32].to_vec()).keys().next().unwrap();
    let key2 = CandidateHashKey::unserialize(k.unwrap().to_vec());
    assert_eq!(key1, key2);
}

#[test]
fn test_message_id_key() {
    let key1: MessageIdKey = MessageIdKey {
        message_id: [8; 32],
        block_number: 4,
        i: 5,
    };

    let key2 = MessageIdKey::unserialize(key1.serialize());
    assert_eq!(key1, key2);
}

#[test]
fn test_index_event_message_id() {
    let trees = init_db("target/debug/test_message_id");
    index_event_message_id(trees.clone(), [8; 32], 4, 5, &[0; 0]);

    let key1 = MessageIdKey {
        message_id: [8; 32],
        block_number: 4,
        i: 5,
    };

    let k = trees.message_id.scan_prefix([8; 32].to_vec()).keys().next().unwrap();
    let key2 = MessageIdKey::unserialize(k.unwrap().to_vec());
    assert_eq!(key1, key2);
}

#[test]
fn test_para_id_key() {
    let key1: ParaIdKey = ParaIdKey {
        para_id: 8,
        block_number: 4,
        i: 5,
    };

    let key2 = ParaIdKey::unserialize(key1.serialize());
    assert_eq!(key1, key2);
}

#[test]
fn test_index_event_para_id() {
    let trees = init_db("target/debug/test_para_id");
    index_event_para_id(trees.clone(), 8, 4, 5, &[0; 0]);

    let key1 = ParaIdKey {
        para_id: 8,
        block_number: 4,
        i: 5,
    };

    let k = trees.para_id.scan_prefix(8_u32.to_be_bytes().to_vec()).keys().next().unwrap();
    let key2 = ParaIdKey::unserialize(k.unwrap().to_vec());
    assert_eq!(key1, key2);
}

#[test]
fn test_pool_id_key() {
    let key1: PoolIdKey = PoolIdKey {
        pool_id: 8,
        block_number: 4,
        i: 5,
    };

    let key2 = PoolIdKey::unserialize(key1.serialize());
    assert_eq!(key1, key2);
}

#[test]
fn test_index_event_pool_id() {
    let trees = init_db("target/debug/test_pool_id");
    index_event_pool_id(trees.clone(), 8, 4, 5, &[0; 0]);

    let key1 = PoolIdKey {
        pool_id: 8,
        block_number: 4,
        i: 5,
    };

    let k = trees.pool_id.scan_prefix(8_u32.to_be_bytes().to_vec()).keys().next().unwrap();
    let key2 = PoolIdKey::unserialize(k.unwrap().to_vec());
    assert_eq!(key1, key2);
}

#[test]
fn test_ref_index_key() {
    let key1: RefIndexKey = RefIndexKey {
        ref_index: 8,
        block_number: 4,
        i: 5,
    };

    let key2 = RefIndexKey::unserialize(key1.serialize());
    assert_eq!(key1, key2);
}

#[test]
fn test_index_event_ref_index() {
    let trees = init_db("target/debug/test_ref_index");
    index_event_ref_index(trees.clone(), 8, 4, 5, &[0; 0]);

    let key1 = RefIndexKey {
        ref_index: 8,
        block_number: 4,
        i: 5,
    };

    let k = trees.ref_index.scan_prefix(8_u32.to_be_bytes().to_vec()).keys().next().unwrap();
    let key2 = RefIndexKey::unserialize(k.unwrap().to_vec());
    assert_eq!(key1, key2);
}

#[test]
fn test_registrar_index_key() {
    let key1: RegistrarIndexKey = RegistrarIndexKey {
        registrar_index: 8,
        block_number: 4,
        i: 5,
    };

    let key2 = RegistrarIndexKey::unserialize(key1.serialize());
    assert_eq!(key1, key2);
}

#[test]
fn test_index_event_registrar_index() {
    let trees = init_db("target/debug/test_registrar_index");
    index_event_registrar_index(trees.clone(), 8, 4, 5, &[0; 0]);

    let key1 = RegistrarIndexKey {
        registrar_index: 8,
        block_number: 4,
        i: 5,
    };

    let k = trees.registrar_index.scan_prefix(8_u32.to_be_bytes().to_vec()).keys().next().unwrap();
    let key2 = RegistrarIndexKey::unserialize(k.unwrap().to_vec());
    assert_eq!(key1, key2);
}

#[test]
fn test_proposal_hash_key() {
    let key1: ProposalHashKey = ProposalHashKey {
        proposal_hash: [8; 32],
        block_number: 4,
        i: 5,
    };

    let key2 = ProposalHashKey::unserialize(key1.serialize());
    assert_eq!(key1, key2);
}

#[test]
fn test_index_event_proposal_hash() {
    let trees = init_db("target/debug/test_proposal_hash");
    index_event_proposal_hash(trees.clone(), [8; 32], 4, 5, &[0; 0]);

    let key1 = ProposalHashKey {
        proposal_hash: [8; 32],
        block_number: 4,
        i: 5,
    };

    let k = trees.proposal_hash.scan_prefix([8; 32].to_vec()).keys().next().unwrap();
    let key2 = ProposalHashKey::unserialize(k.unwrap().to_vec());
    assert_eq!(key1, key2);
}

#[test]
fn test_proposal_index_key() {
    let key1: ProposalIndexKey = ProposalIndexKey {
        proposal_index: 8,
        block_number: 4,
        i: 5,
    };

    let key2 = ProposalIndexKey::unserialize(key1.serialize());
    assert_eq!(key1, key2);
}

#[test]
fn test_index_event_proposal_index() {
    let trees = init_db("target/debug/test_proposal_index");
    index_event_proposal_index(trees.clone(), 8, 4, 5, &[0; 0]);

    let key1 = ProposalIndexKey {
        proposal_index: 8,
        block_number: 4,
        i: 5,
    };

    let k = trees.proposal_index.scan_prefix(8_u32.to_be_bytes().to_vec()).keys().next().unwrap();
    let key2 = ProposalIndexKey::unserialize(k.unwrap().to_vec());
    assert_eq!(key1, key2);
}

#[test]
fn test_tip_hash_key() {
    let key1: TipHashKey = TipHashKey {
        tip_hash: [8; 32],
        block_number: 4,
        i: 5,
    };

    let key2 = TipHashKey::unserialize(key1.serialize());
    assert_eq!(key1, key2);
}

#[test]
fn test_index_event_tip_hash() {
    let trees = init_db("target/debug/test_tip_hash");
    index_event_tip_hash(trees.clone(), [8; 32], 4, 5, &[0; 0]);

    let key1 = TipHashKey {
        tip_hash: [8; 32],
        block_number: 4,
        i: 5,
    };

    let k = trees.tip_hash.scan_prefix([8; 32].to_vec()).keys().next().unwrap();
    let key2 = TipHashKey::unserialize(k.unwrap().to_vec());
    assert_eq!(key1, key2);
}

#[test]
fn test_vector_as_u8_32_array() {
    let vec:Vec<u8> = [8; 48].to_vec();
    assert_eq!(vector_as_u8_32_array(&vec), vec[..32]);
}

#[test]
fn test_vector_as_u8_4_array() {
    let vec:Vec<u8> = [8; 48].to_vec();
    assert_eq!(vector_as_u8_4_array(&vec), vec[..4]);
}

