use clap::Parser;

use subxt::{
    Config,
    SubstrateConfig,
    config::Header,
    utils::AccountId32,
};

use sled::Tree;

use bincode::{Encode, Decode};
use serde::{Serialize, Deserialize};

#[subxt::subxt(runtime_metadata_path = "metadata.scale")]
pub mod polkadot {}

use crate::pallets::balances::Balances;
use crate::pallets::identity::Identity;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
   /// URL of Substrate node to connect to.
   #[arg(short, long)]
   pub url: String,
   /// Block number to start indexing from.
   #[arg(short, long)]
   pub block_height: u32,
}

#[derive(Clone)]
pub struct Trees {
    pub account_id: Tree,
    pub registrar_index: Tree,
}

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
        ].concat()
    }

    pub fn unserialize(vec: Vec<u8>) -> AccountIdKey {
        AccountIdKey {
            account_id: AccountId32(vector_as_u8_32_array(&vec[0..32].to_vec())),
            block_number: u32::from_be_bytes(vector_as_u8_4_array(&vec[32..36].to_vec())),
            i: u32::from_be_bytes(vector_as_u8_4_array(&vec[36..40].to_vec())),
        }
    }
}

pub struct RegistrarIndexKey {
    pub registrar_index: u32,
    pub block_number: <<SubstrateConfig as Config>::Header as Header>::Number,
    pub i: u32,
}

impl RegistrarIndexKey {
    pub fn serialize(&self) -> Vec<u8> {
        [
            self.registrar_index.to_be_bytes().to_vec(),
            self.block_number.to_be_bytes().to_vec(),
            self.i.to_be_bytes().to_vec(),
        ].concat()
    }

    pub fn unserialize(vec: Vec<u8>) -> RegistrarIndexKey {
        RegistrarIndexKey {
            registrar_index: u32::from_be_bytes(vector_as_u8_4_array(&vec[0..4].to_vec())),
            block_number: u32::from_be_bytes(vector_as_u8_4_array(&vec[4..8].to_vec())),
            i: u32::from_be_bytes(vector_as_u8_4_array(&vec[8..12].to_vec())),
        }
    }
}

#[derive(Encode, Decode, Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "pallet")]
pub enum Event {
    #[serde(rename_all = "camelCase")]
    Balances(Balances),
    #[serde(rename_all = "camelCase")]
    Identity(Identity),
}

pub fn vector_as_u8_32_array(vector: &Vec<u8>) -> [u8; 32] {
    let mut arr = [0u8; 32];
    for i in 0..32 {
        arr[i] = vector[i];
    }
    arr
}

pub fn vector_as_u8_4_array(vector: &Vec<u8>) -> [u8; 4] {
    let mut arr = [0u8; 4];
    for i in 0..4 {
        arr[i] = vector[i];
    }
    arr
}



