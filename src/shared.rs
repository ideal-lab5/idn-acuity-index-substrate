use clap::Parser;

use subxt::{
    Config,
    SubstrateConfig,
    config::Header,
    utils::AccountId32,
};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
   /// URL of Substrate node to connect to.
   #[arg(short, long)]
   pub url: String,
}

pub struct AccountIdKey {
    pub account_id: <SubstrateConfig as Config>::AccountId,
    pub block_number: <<SubstrateConfig as Config>::Header as Header>::Number,
    pub idx: u32,
    pub i: u32,
}

impl AccountIdKey {
    pub fn serialize(&self) -> Vec<u8> {
        [
            self.account_id.0.to_vec(),
            self.block_number.to_be_bytes().to_vec(),
            self.idx.to_be_bytes().to_vec(),
            self.i.to_be_bytes().to_vec(),
        ].concat()
    }

    pub fn unserialize(vec: Vec<u8>) -> AccountIdKey {
        AccountIdKey {
            account_id: AccountId32(vector_as_u8_32_array(&vec[0..32].to_vec())),
            block_number: u32::from_be_bytes(vector_as_u8_4_array(&vec[32..36].to_vec())),
            idx: u32::from_be_bytes(vector_as_u8_4_array(&vec[36..40].to_vec())),
            i: u32::from_be_bytes(vector_as_u8_4_array(&vec[40..44].to_vec())),
        }
    }
}

pub struct TransferEventValue {
    pub from: AccountId32,
    pub to: AccountId32,
    pub value: u128,
}

impl TransferEventValue {
    pub fn serialize(&self) -> Vec<u8> {
        [
            self.from.0.to_vec(),
            self.to.0.to_vec(),
            self.value.to_be_bytes().to_vec(),
        ].concat()
    }

    pub fn unserialize(vec: Vec<u8>) -> TransferEventValue {
        TransferEventValue {
            from: AccountId32(vector_as_u8_32_array(&vec[0..32].to_vec())),
            to: AccountId32(vector_as_u8_32_array(&vec[32..64].to_vec())),
            value: u128::from_be_bytes(vector_as_u8_16_array(&vec[64..80].to_vec())),
        }
    }
}

pub fn vector_as_u8_32_array(vector: &Vec<u8>) -> [u8; 32] {
    let mut arr = [0u8; 32];
    for i in 0..32 {
        arr[i] = vector[i];
    }
    arr
}

pub fn vector_as_u8_16_array(vector: &Vec<u8>) -> [u8; 16] {
    let mut arr = [0u8; 16];
    for i in 0..16 {
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


