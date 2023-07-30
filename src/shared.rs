use subxt::{config::Header, utils::AccountId32, Config, SubstrateConfig};

use sled::Tree;

use parity_scale_codec::{Decode, Encode};
use serde::{Deserialize, Serialize};

use tokio::sync::mpsc::UnboundedSender;

#[derive(Encode, Decode, Serialize, Debug, Clone)]
pub struct ParaId(pub u32);

pub trait RuntimeIndexer {
    type RuntimeConfig: subxt::Config;

    fn get_name() -> &'static str;

    fn get_url() -> &'static str;

    fn get_start_block() -> u32;

    fn process_event(
        indexer: &crate::Indexer<Self>,
        block_number: u32,
        event_index: u32,
        event: subxt::events::EventDetails<Self::RuntimeConfig>,
    ) -> Result<(), subxt::Error>
    where
        Self: Sized;
}

#[derive(Clone)]
pub struct Trees {
    pub root: sled::Db,
    pub variant: Tree,
    pub account_id: Tree,
    pub account_index: Tree,
    pub auction_index: Tree,
    pub bounty_index: Tree,
    pub candidate_hash: Tree,
    pub era_index: Tree,
    pub message_id: Tree,
    pub para_id: Tree,
    pub pool_id: Tree,
    pub preimage_hash: Tree,
    pub proposal_hash: Tree,
    pub proposal_index: Tree,
    pub ref_index: Tree,
    pub registrar_index: Tree,
    pub session_index: Tree,
    pub tip_hash: Tree,
}

/**
 * Each tree has its own key format.
 */

#[derive(PartialEq, Debug)]
pub struct VariantKey {
    pub pallet_index: u8,
    pub variant_index: u8,
    pub block_number: <<SubstrateConfig as Config>::Header as Header>::Number,
    pub event_index: u32,
}

impl VariantKey {
    pub fn serialize(&self) -> Vec<u8> {
        [
            self.pallet_index.to_be_bytes().to_vec(),
            self.variant_index.to_be_bytes().to_vec(),
            self.block_number.to_be_bytes().to_vec(),
            self.event_index.to_be_bytes().to_vec(),
        ]
        .concat()
    }

    pub fn unserialize(vec: Vec<u8>) -> Self {
        VariantKey {
            pallet_index: vec[0],
            variant_index: vec[1],
            block_number: u32::from_be_bytes(vector_as_u8_4_array(&vec[2..6])),
            event_index: u32::from_be_bytes(vector_as_u8_4_array(&vec[6..10])),
        }
    }
}

#[derive(PartialEq, Debug)]
pub struct AccountIdKey {
    pub account_id: <SubstrateConfig as Config>::AccountId,
    pub block_number: <<SubstrateConfig as Config>::Header as Header>::Number,
    pub event_index: u32,
}

impl AccountIdKey {
    pub fn serialize(&self) -> Vec<u8> {
        [
            self.account_id.0.to_vec(),
            self.block_number.to_be_bytes().to_vec(),
            self.event_index.to_be_bytes().to_vec(),
        ]
        .concat()
    }

    pub fn unserialize(vec: Vec<u8>) -> Self {
        AccountIdKey {
            account_id: AccountId32(vector_as_u8_32_array(&vec[0..32])),
            block_number: u32::from_be_bytes(vector_as_u8_4_array(&vec[32..36])),
            event_index: u32::from_be_bytes(vector_as_u8_4_array(&vec[36..40])),
        }
    }
}

#[derive(PartialEq, Debug)]
pub struct U32Key {
    pub key: u32,
    pub block_number: <<SubstrateConfig as Config>::Header as Header>::Number,
    pub event_index: u32,
}

impl U32Key {
    pub fn serialize(&self) -> Vec<u8> {
        [
            self.key.to_be_bytes().to_vec(),
            self.block_number.to_be_bytes().to_vec(),
            self.event_index.to_be_bytes().to_vec(),
        ]
        .concat()
    }

    pub fn unserialize(vec: Vec<u8>) -> Self {
        U32Key {
            key: u32::from_be_bytes(vector_as_u8_4_array(&vec[0..4])),
            block_number: u32::from_be_bytes(vector_as_u8_4_array(&vec[4..8])),
            event_index: u32::from_be_bytes(vector_as_u8_4_array(&vec[8..12])),
        }
    }
}

#[derive(PartialEq, Debug)]
pub struct CandidateHashKey {
    pub candidate_hash: [u8; 32],
    pub block_number: <<SubstrateConfig as Config>::Header as Header>::Number,
    pub event_index: u32,
}

impl CandidateHashKey {
    pub fn serialize(&self) -> Vec<u8> {
        [
            self.candidate_hash.to_vec(),
            self.block_number.to_be_bytes().to_vec(),
            self.event_index.to_be_bytes().to_vec(),
        ]
        .concat()
    }

    pub fn unserialize(vec: Vec<u8>) -> Self {
        CandidateHashKey {
            candidate_hash: vector_as_u8_32_array(&vec[0..32]),
            block_number: u32::from_be_bytes(vector_as_u8_4_array(&vec[32..36])),
            event_index: u32::from_be_bytes(vector_as_u8_4_array(&vec[36..40])),
        }
    }
}

#[derive(PartialEq, Debug)]
pub struct MessageIdKey {
    pub message_id: [u8; 32],
    pub block_number: <<SubstrateConfig as Config>::Header as Header>::Number,
    pub event_index: u32,
}

impl MessageIdKey {
    pub fn serialize(&self) -> Vec<u8> {
        [
            self.message_id.to_vec(),
            self.block_number.to_be_bytes().to_vec(),
            self.event_index.to_be_bytes().to_vec(),
        ]
        .concat()
    }

    pub fn unserialize(vec: Vec<u8>) -> Self {
        MessageIdKey {
            message_id: vector_as_u8_32_array(&vec[0..32]),
            block_number: u32::from_be_bytes(vector_as_u8_4_array(&vec[32..36])),
            event_index: u32::from_be_bytes(vector_as_u8_4_array(&vec[36..40])),
        }
    }
}

#[derive(PartialEq, Debug)]
pub struct HashKey {
    pub hash: [u8; 32],
    pub block_number: <<SubstrateConfig as Config>::Header as Header>::Number,
    pub event_index: u32,
}

impl HashKey {
    pub fn serialize(&self) -> Vec<u8> {
        [
            self.hash.to_vec(),
            self.block_number.to_be_bytes().to_vec(),
            self.event_index.to_be_bytes().to_vec(),
        ]
        .concat()
    }

    pub fn unserialize(vec: Vec<u8>) -> Self {
        HashKey {
            hash: vector_as_u8_32_array(&vec[0..32]),
            block_number: u32::from_be_bytes(vector_as_u8_4_array(&vec[32..36])),
            event_index: u32::from_be_bytes(vector_as_u8_4_array(&vec[36..40])),
        }
    }
}

#[derive(PartialEq, Debug)]
pub struct TipHashKey {
    pub tip_hash: [u8; 32],
    pub block_number: <<SubstrateConfig as Config>::Header as Header>::Number,
    pub event_index: u32,
}

impl TipHashKey {
    pub fn serialize(&self) -> Vec<u8> {
        [
            self.tip_hash.to_vec(),
            self.block_number.to_be_bytes().to_vec(),
            self.event_index.to_be_bytes().to_vec(),
        ]
        .concat()
    }

    pub fn unserialize(vec: Vec<u8>) -> Self {
        TipHashKey {
            tip_hash: vector_as_u8_32_array(&vec[0..32]),
            block_number: u32::from_be_bytes(vector_as_u8_4_array(&vec[32..36])),
            event_index: u32::from_be_bytes(vector_as_u8_4_array(&vec[36..40])),
        }
    }
}

pub fn vector_as_u8_32_array(vector: &[u8]) -> [u8; 32] {
    let mut arr = [0u8; 32];
    arr[..32].copy_from_slice(&vector[..32]);
    arr
}

pub fn vector_as_u8_4_array(vector: &[u8]) -> [u8; 4] {
    let mut arr = [0u8; 4];
    arr[..4].copy_from_slice(&vector[..4]);
    arr
}

#[derive(Copy, Clone, Debug, PartialEq, Hash, Eq)]
pub struct Bytes32(pub [u8; 32]);

impl AsRef<[u8]> for Bytes32 {
    fn as_ref(&self) -> &[u8] {
        &self.0[..]
    }
}

impl Serialize for Bytes32 {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut hex_string = "0x".to_owned();
        hex_string.push_str(&hex::encode(self.0));
        serializer.serialize_str(&hex_string)
    }
}

impl<'de> Deserialize<'de> for Bytes32 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        match String::deserialize(deserializer)?.get(2..66) {
            Some(message_id) => match hex::decode(message_id) {
                Ok(message_id) => Ok(Bytes32(vector_as_u8_32_array(&message_id))),
                Err(_error) => Err(serde::de::Error::custom("error")),
            },
            None => Err(serde::de::Error::custom("error")),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, Hash)]
#[serde(tag = "type", content = "value")]
pub enum Key {
    AccountId(Bytes32),
    AccountIndex(u32),
    AuctionIndex(u32),
    BountyIndex(u32),
    CandidateHash(Bytes32),
    EraIndex(u32),
    MessageId(Bytes32),
    ParaId(u32),
    PoolId(u32),
    PreimageHash(Bytes32),
    ProposalHash(Bytes32),
    ProposalIndex(u32),
    RefIndex(u32),
    RegistrarIndex(u32),
    SessionIndex(u32),
    TipHash(Bytes32),
    Variant(u8, u8),
}

#[derive(Deserialize, Debug, Clone)]
#[serde(tag = "type")]
pub enum RequestMessage {
    Status,
    Variants,
    GetEvents { key: Key },
    SubscribeEvents { key: Key },
}

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Event {
    pub block_number: u32,
    pub event_index: u32,
}

#[derive(Serialize, Debug, Clone)]
pub struct EventMeta {
    pub index: u8,
    pub name: String,
}

#[derive(Serialize, Debug, Clone)]
pub struct PalletMeta {
    pub index: u8,
    pub name: String,
    pub events: Vec<EventMeta>,
}

#[derive(Serialize, Debug, Clone)]
#[serde(tag = "type", content = "data")]
#[serde(rename_all = "camelCase")]
pub enum ResponseMessage {
    #[serde(rename_all = "camelCase")]
    Status {
        last_head_block: u32,
        last_batch_block: u32,
        batch_indexing_complete: bool,
    },
    Variants(Vec<PalletMeta>),
    Events {
        key: Key,
        events: Vec<Event>,
    },
    Subscribed,
    //    Error,
}

#[derive(Debug)]
pub struct SubscribeMessage {
    pub key: Key,
    pub sub_response_tx: UnboundedSender<ResponseMessage>,
}
