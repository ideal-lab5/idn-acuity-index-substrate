use byteorder::BigEndian;
use sled::Tree;
use zerocopy::byteorder::{U16, U32};
use zerocopy_derive::{AsBytes, FromBytes, FromZeroes, Unaligned};

use serde::{Deserialize, Serialize};

use tokio::sync::mpsc::UnboundedSender;
use tokio_tungstenite::tungstenite;

#[derive(Debug)]
pub enum IndexError {
    Sled(sled::Error),
    Subxt(subxt::Error),
    Tungstenite(tungstenite::Error),
    BlockNotFound(u32),
}

impl From<sled::Error> for IndexError {
    fn from(err: sled::Error) -> IndexError {
        IndexError::Sled(err)
    }
}

impl From<subxt::Error> for IndexError {
    fn from(err: subxt::Error) -> IndexError {
        IndexError::Subxt(err)
    }
}

impl From<tungstenite::Error> for IndexError {
    fn from(err: tungstenite::Error) -> IndexError {
        IndexError::Tungstenite(err)
    }
}

/// Indexer for a specific chain.
pub trait RuntimeIndexer {
    type RuntimeConfig: subxt::Config;

    fn get_name() -> &'static str;

    fn get_genesis_hash() -> <Self::RuntimeConfig as subxt::Config>::Hash;

    fn get_default_url() -> &'static str;

    fn get_default_start_block() -> u32;

    fn process_event(
        indexer: &crate::Indexer<Self>,
        block_number: u32,
        event_index: u16,
        event: subxt::events::EventDetails<Self::RuntimeConfig>,
    ) -> Result<u32, IndexError>;
}

#[derive(Clone)]
pub struct Trees {
    pub root: sled::Db,
    pub span: Tree,
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

#[derive(FromZeroes, FromBytes, AsBytes, Unaligned, PartialEq, Debug)]
#[repr(C)]
pub struct VariantKey {
    pub pallet_index: u8,
    pub variant_index: u8,
    pub block_number: U32<BigEndian>,
    pub event_index: U16<BigEndian>,
}

#[derive(FromZeroes, FromBytes, AsBytes, Unaligned, PartialEq, Debug)]
#[repr(C)]
pub struct Bytes32Key {
    pub key: [u8; 32],
    pub block_number: U32<BigEndian>,
    pub event_index: U16<BigEndian>,
}

#[derive(FromZeroes, FromBytes, AsBytes, Unaligned, PartialEq, Debug)]
#[repr(C)]
pub struct U32Key {
    pub key: U32<BigEndian>,
    pub block_number: U32<BigEndian>,
    pub event_index: U16<BigEndian>,
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
                Ok(message_id) => Ok(Bytes32(message_id.try_into().unwrap())),
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
    pub event_index: u16,
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
