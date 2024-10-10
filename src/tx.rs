
use bitcoincash_addr::Address;
use log::debug;
use serde::{Deserialize, Serialize};
use crate::{errors::Result, wallet::hash_pub_key};


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TXInput {
    pub txid: String,
    pub vout: i32,
    pub signature: Vec<u8>,
    pub pub_key: Vec<u8>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TXOutput {
    pub value: i32,
    pub pub_key_hash: Vec<u8>,
}

impl TXInput {
    pub fn can_unlock_output_with(&self, unlocking_data: &[u8]) -> bool {
        let mut pub_key_hash = self.pub_key.clone();
        hash_pub_key(&mut pub_key_hash);
        pub_key_hash == unlocking_data
    }
}



impl TXOutput {
    pub fn can_be_unlock_with(&self, unlocking_data:  &[u8]) -> bool {
        self.pub_key_hash == unlocking_data
    }
    
    fn lock(&mut self, address: &str) -> Result<()> {
        let pub_key_hash = Address::decode(address).unwrap().body;
        debug!("lock: {}", address);
        self.pub_key_hash = pub_key_hash;
        Ok(())
    }

    pub fn new(value: i32, address: String) -> Result<Self> {
        let mut txo = TXOutput {
            value,
            pub_key_hash: Vec::new(),
        };
        txo.lock(&address)?;
        Ok(txo)
    }
}