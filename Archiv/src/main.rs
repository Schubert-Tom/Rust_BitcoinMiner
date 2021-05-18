mod miner;
use hex_literal::hex;
use sha2::{Sha256, Digest};
use std::fmt::Binary;
use std::str;
use hex::decode;
use bitcoin_hashes::sha256d;

pub fn build_root(branches: &[Vec<u8>], coinbase: &Vec<u8>) -> Vec<u8>
{
    let mut root = vec![];
    for branch in branches 
    {
        let to_hash = root.extend(branch);
        root = doublesha(to_hash);
    }
    root //= root.reverse()
}
//extern crate crypto;

fn main() {
    let mut x: u128 = 1152387954141307219;
    println!("{:b}", x);
    println!("{:b}", x.swap_bytes());
    let mut hasher = Sha256::new();

// write input message
hasher.update(b"hello world");

// read hash digest and consume hasher
let result = hasher.finalize();
println!("{}", miner::build_root(&[], "0ffe1abd1a08215353c233d6e009613e95eec4253832a761af28ff37ac5a150c"));

    println!("{}", miner::ret_extranonce2(10));
    let mut cb1 = Vec::new();
    cb1.push(1);
    println!("{:?}", miner::build_coinbase(& cb1, "defg", "hijk", "lmnop"));
    let mut sha = Sha256::new(); 
    sha.update("1111");
    let result = sha.finalize();
    let mut sres = "".to_string();
    strhex_to_i32("f");

    //let res: &[u8] = &result;
    //result.iter().for_each(|&x| println!("{}", hex::decode(x)));
    //println!("{:?}", format!("{:b}", result[0]));
    //let str_result = std::str::from_utf8(&result).unwrap();
    //println!("{:?}", str_result);
//     println!("{:?}", 1<<(8*(2)));
//     miner::test();
//     let _a = strhex_to_i32("6a09e667");
//     //let _a = chex("6a0qe667");
//     let _b = strhex_to_i32("bb67ae85");
//     let _c = strhex_to_i32("3c6ef372");
//     let _d = strhex_to_i32("a54ff53a");
//     let _e = strhex_to_i32("510e527f");
//     let _f = strhex_to_i32("9605688c");
//     let _g = strhex_to_i32("1f83d9ab");
//     let _h = strhex_to_i32("5be0cd19");
//     println!("Conv: {}", _a);
//     println!("Conv: {}", _b);
//     println!("Conv: {}", _c);
//     println!("Conv: {}", _d);
//     println!("Conv: {}", _e);
//     println!("Conv: {}", _f);
//     println!("Conv: {}", _g);
//     println!("Conv: {}", _h);
// 
}

fn chex(hex: &str) -> String {
    hex.chars().map(to_binary).collect()
}

fn to_binary(c: char) -> &'static str {
    match c {
        '0' => "0000",
        '1' => "0001",
        '2' => "0010",
        '3' => "0011",
        '4' => "0100",
        '5' => "0101",
        '6' => "0110",
        '7' => "0111",
        '8' => "1000",
        '9' => "1001",
        'a' => "1010",
        'b' => "1011",
        'c' => "1100",
        'd' => "1101",
        'e' => "1110",
        'f' => "1111",
        _ => "",
    }
}
fn strhex_to_i32(hex: &str) -> u32 //Eigentlich reicht 32 für sha-256, vllt unsigned
{
    let res = u32::from_str_radix(hex, 16); 
    match res {
        Ok(v) => v,
        Err(e) => panic!("Conversion hex to i32 failed: {}", e),
    }
}
