use clap::Parser;

use subxt::{config::Header, utils::AccountId32, Config, SubstrateConfig};

use sled::Tree;

use parity_scale_codec::{Decode, Encode};
use serde::{Deserialize, Serialize};

use tokio::sync::mpsc::UnboundedSender;

#[derive(Encode, Decode, Serialize, Debug, Clone)]
pub struct ParaId(pub u32);

#[subxt::subxt(runtime_metadata_path = "metadata.scale")]
pub mod polkadot {}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// URL of Substrate node to connect to.
    #[arg(short, long)]
    pub url: Option<String>,
    /// Block number to start indexing from.
    #[arg(short, long)]
    pub block_height: Option<u32>,
    /// How many blocks to query at the same time [128]
    #[arg(short, long)]
    pub async_blocks: Option<u32>,
}

pub trait RuntimeIndexer {
    fn process_event(
        block_number: u32,
        event_index: u32,
        event: subxt::events::EventDetails<subxt::PolkadotConfig>,
    );
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
    pub i: u32,
}

impl VariantKey {
    pub fn serialize(&self) -> Vec<u8> {
        [
            self.pallet_index.to_be_bytes().to_vec(),
            self.variant_index.to_be_bytes().to_vec(),
            self.block_number.to_be_bytes().to_vec(),
            self.i.to_be_bytes().to_vec(),
        ]
        .concat()
    }

    pub fn unserialize(vec: Vec<u8>) -> Self {
        VariantKey {
            pallet_index: vec[0],
            variant_index: vec[1],
            block_number: u32::from_be_bytes(vector_as_u8_4_array(&vec[2..6])),
            i: u32::from_be_bytes(vector_as_u8_4_array(&vec[6..10])),
        }
    }
}

#[derive(PartialEq, Debug)]
pub struct AccountIdKey {
    pub account_id: <SubstrateConfig as Config>::AccountId,
    pub block_number: <<SubstrateConfig as Config>::Header as Header>::Number,
    pub i: u32,
}

impl AccountIdKey {
    pub fn serialize(&self) -> Vec<u8> {
        [
            self.account_id.0.to_vec(),
            self.block_number.to_be_bytes().to_vec(),
            self.i.to_be_bytes().to_vec(),
        ]
        .concat()
    }

    pub fn unserialize(vec: Vec<u8>) -> Self {
        AccountIdKey {
            account_id: AccountId32(vector_as_u8_32_array(&vec[0..32])),
            block_number: u32::from_be_bytes(vector_as_u8_4_array(&vec[32..36])),
            i: u32::from_be_bytes(vector_as_u8_4_array(&vec[36..40])),
        }
    }
}

#[derive(PartialEq, Debug)]
pub struct U32Key {
    pub key: u32,
    pub block_number: <<SubstrateConfig as Config>::Header as Header>::Number,
    pub i: u32,
}

impl U32Key {
    pub fn serialize(&self) -> Vec<u8> {
        [
            self.key.to_be_bytes().to_vec(),
            self.block_number.to_be_bytes().to_vec(),
            self.i.to_be_bytes().to_vec(),
        ]
        .concat()
    }

    pub fn unserialize(vec: Vec<u8>) -> Self {
        U32Key {
            key: u32::from_be_bytes(vector_as_u8_4_array(&vec[0..4])),
            block_number: u32::from_be_bytes(vector_as_u8_4_array(&vec[4..8])),
            i: u32::from_be_bytes(vector_as_u8_4_array(&vec[8..12])),
        }
    }
}

#[derive(PartialEq, Debug)]
pub struct CandidateHashKey {
    pub candidate_hash: [u8; 32],
    pub block_number: <<SubstrateConfig as Config>::Header as Header>::Number,
    pub i: u32,
}

impl CandidateHashKey {
    pub fn serialize(&self) -> Vec<u8> {
        [
            self.candidate_hash.to_vec(),
            self.block_number.to_be_bytes().to_vec(),
            self.i.to_be_bytes().to_vec(),
        ]
        .concat()
    }

    pub fn unserialize(vec: Vec<u8>) -> Self {
        CandidateHashKey {
            candidate_hash: vector_as_u8_32_array(&vec[0..32]),
            block_number: u32::from_be_bytes(vector_as_u8_4_array(&vec[32..36])),
            i: u32::from_be_bytes(vector_as_u8_4_array(&vec[36..40])),
        }
    }
}

#[derive(PartialEq, Debug)]
pub struct MessageIdKey {
    pub message_id: [u8; 32],
    pub block_number: <<SubstrateConfig as Config>::Header as Header>::Number,
    pub i: u32,
}

impl MessageIdKey {
    pub fn serialize(&self) -> Vec<u8> {
        [
            self.message_id.to_vec(),
            self.block_number.to_be_bytes().to_vec(),
            self.i.to_be_bytes().to_vec(),
        ]
        .concat()
    }

    pub fn unserialize(vec: Vec<u8>) -> Self {
        MessageIdKey {
            message_id: vector_as_u8_32_array(&vec[0..32]),
            block_number: u32::from_be_bytes(vector_as_u8_4_array(&vec[32..36])),
            i: u32::from_be_bytes(vector_as_u8_4_array(&vec[36..40])),
        }
    }
}

#[derive(PartialEq, Debug)]
pub struct HashKey {
    pub hash: [u8; 32],
    pub block_number: <<SubstrateConfig as Config>::Header as Header>::Number,
    pub i: u32,
}

impl HashKey {
    pub fn serialize(&self) -> Vec<u8> {
        [
            self.hash.to_vec(),
            self.block_number.to_be_bytes().to_vec(),
            self.i.to_be_bytes().to_vec(),
        ]
        .concat()
    }

    pub fn unserialize(vec: Vec<u8>) -> Self {
        HashKey {
            hash: vector_as_u8_32_array(&vec[0..32]),
            block_number: u32::from_be_bytes(vector_as_u8_4_array(&vec[32..36])),
            i: u32::from_be_bytes(vector_as_u8_4_array(&vec[36..40])),
        }
    }
}

#[derive(PartialEq, Debug)]
pub struct TipHashKey {
    pub tip_hash: [u8; 32],
    pub block_number: <<SubstrateConfig as Config>::Header as Header>::Number,
    pub i: u32,
}

impl TipHashKey {
    pub fn serialize(&self) -> Vec<u8> {
        [
            self.tip_hash.to_vec(),
            self.block_number.to_be_bytes().to_vec(),
            self.i.to_be_bytes().to_vec(),
        ]
        .concat()
    }

    pub fn unserialize(vec: Vec<u8>) -> Self {
        TipHashKey {
            tip_hash: vector_as_u8_32_array(&vec[0..32]),
            block_number: u32::from_be_bytes(vector_as_u8_4_array(&vec[32..36])),
            i: u32::from_be_bytes(vector_as_u8_4_array(&vec[36..40])),
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

// Direct copy of AccountId32 from subxt, but with Copy and Hash traits implemented.
// https://github.com/paritytech/subxt/blob/master/subxt/src/utils/account_id.rs
#[derive(Copy, Clone, Debug, PartialEq, Hash, Eq)]
pub struct AccountId32Hash(pub [u8; 32]);

impl AsRef<[u8]> for AccountId32Hash {
    fn as_ref(&self) -> &[u8] {
        &self.0[..]
    }
}

impl AccountId32Hash {
    // Return the ss58-check string for this key. Adapted from `sp_core::crypto`. We need this to
    // serialize our account appropriately but otherwise don't care.
    fn to_ss58check(self) -> String {
        // For serializing to a string to obtain the account nonce, we use the default substrate
        // prefix (since we have no way to otherwise pick one). It doesn't really matter, since when
        // it's deserialized back in system_accountNextIndex, we ignore this (so long as it's valid).
        const SUBSTRATE_SS58_PREFIX: u8 = 42;
        // prefix <= 63 just take up one byte at the start:
        let mut v = vec![SUBSTRATE_SS58_PREFIX];
        // then push the account ID bytes.
        v.extend(self.0);
        // then push a 2 byte checksum of what we have so far.
        let r = ss58hash(&v);
        v.extend(&r[0..2]);
        // then encode to base58.
        use base58::ToBase58;
        v.to_base58()
    }

    // This isn't strictly needed, but to give our AccountId32 a little more usefulness, we also
    // implement the logic needed to decode an AccountId32 from an SS58 encoded string. This is exposed
    // via a `FromStr` impl.
    fn from_ss58check(s: &str) -> Result<Self, FromSs58Error> {
        const CHECKSUM_LEN: usize = 2;
        let body_len = 32;

        use base58::FromBase58;
        let data = s.from_base58().map_err(|_| FromSs58Error::BadBase58)?;
        if data.len() < 2 {
            return Err(FromSs58Error::BadLength);
        }
        let prefix_len = match data[0] {
            0..=63 => 1,
            64..=127 => 2,
            _ => return Err(FromSs58Error::InvalidPrefix),
        };
        if data.len() != prefix_len + body_len + CHECKSUM_LEN {
            return Err(FromSs58Error::BadLength);
        }
        let hash = ss58hash(&data[0..body_len + prefix_len]);
        let checksum = &hash[0..CHECKSUM_LEN];
        if data[body_len + prefix_len..body_len + prefix_len + CHECKSUM_LEN] != *checksum {
            // Invalid checksum.
            return Err(FromSs58Error::InvalidChecksum);
        }

        let result = data[prefix_len..body_len + prefix_len]
            .try_into()
            .map_err(|_| FromSs58Error::BadLength)?;
        Ok(AccountId32Hash(result))
    }
}

/// An error obtained from trying to interpret an SS58 encoded string into an AccountId32
#[derive(thiserror::Error, Clone, Copy, Eq, PartialEq, Debug)]
#[allow(missing_docs)]
pub enum FromSs58Error {
    #[error("Base 58 requirement is violated")]
    BadBase58,
    #[error("Length is bad")]
    BadLength,
    #[error("Invalid checksum")]
    InvalidChecksum,
    #[error("Invalid SS58 prefix byte.")]
    InvalidPrefix,
}

// We do this just to get a checksum to help verify the validity of the address in to_ss58check
fn ss58hash(data: &[u8]) -> Vec<u8> {
    use blake2::{Blake2b512, Digest};
    const PREFIX: &[u8] = b"SS58PRE";
    let mut ctx = Blake2b512::new();
    ctx.update(PREFIX);
    ctx.update(data);
    ctx.finalize().to_vec()
}

impl Serialize for AccountId32Hash {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_ss58check())
    }
}

impl<'de> Deserialize<'de> for AccountId32Hash {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        AccountId32Hash::from_ss58check(&String::deserialize(deserializer)?)
            .map_err(|e| serde::de::Error::custom(format!("{e:?}")))
    }
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
    AccountId(AccountId32Hash),
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
    pub i: u32,
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
