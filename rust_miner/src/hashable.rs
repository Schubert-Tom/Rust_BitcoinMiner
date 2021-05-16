//Trait for hashing values from a struct 
// Just implement it use 

pub trait Hashable{
    // Implement a bytes fn that looks smth like this:
    /*
        // Creates mutable Vector
        let mut bytes = vec![];
        // Use lib function to convert u32, u64, u128 Integers
        bytes.extend(&u32_bytes(&self.index))

    */

    pub fn conv_to_bytes(&self) -> Vec<u8>;

    pub fn hash(&self)->Vec<u8>{
        return crypto_hash::digest(crypto_hash::Algorithm:SHA256, &self.conv_to_bytes)
    }
}