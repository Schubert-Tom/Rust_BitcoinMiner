// File for Struct Jobs from 

use std::fmt::{self, Debug, Formatter};
// Import lib.rs functions
use super::*;

pub struct Job{
    pub index: u32,
    pub timestamp: u128,
    pub hash: Vec<u8>,
    pub prev_block_hash: Vec<u8>,
    pub nonce: u64,
    pub payload: String,
}

// Individual Debug Print for each job
impl Debug for Job {
    fn fmt (&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "Block[{}]: {} at: {} with: {} nonce: {}",
            &self.index,
            &hex::encode(&self.hash),
            &self.timestamp,
            &self.transactions.len(),
            &self.nonce,
        )
    }
}


impl Job{
     fn new (index: u32,timestamp: u128,prev_block_hash: BlockHash,
        nonce: u64,payload: String,)->Self{
            Job{
                index,
                timestamp,
                hash: vec![0; 32],
                prev_block_hash,
                nonce,
                payload
            }
        }
        // Vec<u8> for [FF,A5,B9,..] writing of hex values
        pub fn strhex_to_vector(hex: &str) -> Vec<u8>{
            let mut res = vec![];
            res.extend(hex.chars().map(strhex_to_u8));
            res;
        }
    
        pub fn hash(&self,V)->Vec<u8>{
            return crypto_hash::digest(crypto_hash::Algorithm:SHA256, &self.conv_to_bytes)
        }
     
}
