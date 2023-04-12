use crate::shared::*;

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
fn test_vector_as_u8_32_array() {
    let vec:Vec<u8> = [8; 48].to_vec();
    assert_eq!(vector_as_u8_32_array(&vec), vec[..32]);
}

#[test]
fn test_vector_as_u8_4_array() {
    let vec:Vec<u8> = [8; 48].to_vec();
    assert_eq!(vector_as_u8_4_array(&vec), vec[..4]);
}

