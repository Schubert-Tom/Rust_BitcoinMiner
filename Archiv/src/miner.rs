//import hashlib, struct
use std::io::Cursor;
use byteorder::{BigEndian, WriteBytesExt, ReadBytesExt};

pub fn test()
{
    println!("Test");

let ver = 2;
let prev_block = "000000000000000117c80378b8da0e33559b5997f2ad55e2f7d18ec1975b9717";
let mrkl_root = "871714dcbae6c8193a2bb9b2a69fe1c0440399f38d94b3a0f1b447275a29978a";
let time_ = 0x53058b35; // 2014-02-20 04:57:25
let bits = 0x19015f53;
 
//https://en.bitcoin.it/wiki/Difficulty
let exp = bits >> 24;
let mant = bits & 0xffffff;

//let target_hexstr = format!("{:0<64}", (mant * (1<<(8*(exp - 3)))));
//println!("{:?}", target_hexstr);
}

// target_str = target_hexstr.decode('hex')


// nonce = 0
// while nonce < 0x100000000:
//     header = ( struct.pack("<L", ver) + prev_block.decode('hex')[::-1] +
//           mrkl_root.decode('hex')[::-1] + struct.pack("<LLL", time_, bits, nonce))
//     hash = hashlib.sha256(hashlib.sha256(header).digest()).digest()
//     print nonce, hash[::-1].encode('hex')
//     if hash[::-1] < target_str:
//         print 'success'
//         break
//     nonce += 1

pub fn ret_extranonce2(_length: usize) -> String
{
    //Nur Nullen in der vorgeschriebenen Länge
    "0".repeat(_length)
}

pub fn build_coinbase(coinb1: &str, coinb2: &str, extranonce1: &str, extranonce2: &str) -> String
{
    coinb1.to_string() + coinb2 + extranonce1 + extranonce2
}

pub fn build_root(branches: &[&str], coinbase: &str) -> String
{
    let mut root = coinbase.to_string();
    for branch in branches 
    {
        root = (root + branch);
    }
    //root = "0ffe1abd1a08215353c233d6e009613e95eec4253832a761af28ff37ac5a150c".to_string();
    let mut parts: [u64; 4] = [0; 4];
    for el in 0..parts.len() //16 Character = 128 Bit //Durch 4 ersetzen
    {
        let new = strhex_to_u64(&root[16*el..16*(el+1)]);
        println!("{}\n", new);  
        println!("{:b}\n", new);
        parts[parts.len()-el-1] = new.swap_bytes();
        println!("{:b}\n", new.swap_bytes());
        println!("{}\n\n\n", new.swap_bytes());  
    }
    let mut swapped_root = "".to_string();
    for el in (0..4) //16 Character = 128 Bit //Durch 4 ersetzen
    {
        let s: String = format!("{:x}", parts[el]).to_owned();
        swapped_root = swapped_root + &s[..];
    }

    return swapped_root;
}

fn strhex_to_u64(hex: &str) -> u64 
{
    let res = u64::from_str_radix(hex, 16); 
    match res {
        Ok(v) => v,
        Err(e) => panic!("Conversion hex to u128 failed: {}", e),
    }
}
//111111111110000110101011110100011010000010000010000101010011010100111100001000110011110101101110000000001001011000010011111010010101111011101100010000100101001110000011001010100111011000011010111100101000111111110011011110101100010110100001010100001100
//1100000101010101101010101100001101111111111100101000101011110000000000000000000000000000000000000000000000000000000000000000011000011010011100110010001110000010010111000100111011101001010100000000000000000000000000000000000000000000000000000000000000000011111001100001000010011110000011010110001100111100001001010011000000000000000000000000000000000000000000000000000000000000000001010011001000010000100000011010101111010001101011111110000011110000000000000000000000000000000000000000000000000000000000000000