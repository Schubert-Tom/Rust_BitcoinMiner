use std::iter::Iterator;
use crypto_hash::{Algorithm, digest};
use hex::*;
use std::{fmt, num::ParseIntError};

fn main(){
    let mut hex="FF6236e623";
    let result=&hex::encode(doublesha(&hex));
    println!("{:?}", result); // prints [102, 111, 111]
    
}

/*// Creates mutable Vector
let mut bytes = vec![];
// Use lib function to convert u32, u64, u128 Integers
println("{:?}", &bytes);
bytes.extend(&u32_bytes(&self.index))
println("{:?}", &bytes);

*/

pub fn doublesha(hex:&str)->Vec<u8>{
    let res=match hex::decode(hex) 
    {
        Ok(v) => v,
        Err(e) => panic!("Conversion hex to Vec<u8> failed: {}", e),
    };
    let hash1=crypto_hash::digest(crypto_hash::Algorithm::SHA256, &res);
    return crypto_hash::digest(crypto_hash::Algorithm::SHA256, &hash1);
}


