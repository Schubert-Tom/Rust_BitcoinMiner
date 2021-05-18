use std::iter::Iterator;
use crypto_hash::{Algorithm, digest};
use hex::*;
use serde;
use serde::{Serialize, Deserialize};
use serde_json::*; 


#[derive(Debug, Deserialize)]
struct Example {
    field: i32,
    #[serde(flatten)]
    an_enum: AnEnum,
}

#[derive(Debug, Deserialize)]
enum AnEnum {
    A(i32),
    B(i32),
}

fn main() {
    let abba = r#"{ "field": 42, "A": 76 }"#;
    let b = r#"{ "field": 42, "B": 110 }"#;

    let abba = serde_json::from_str::<Example>(abba);
    let b = serde_json::from_str::<Example>(b);

    println!("{:?}", abba);
    println!("{:?}", b);
}
    /*
    let hex="FF6236e623";
    let result=&hex::encode(doublesha(&hex));
    println!("{:?}", result); // prints [102, 111, 111]
    */


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





/*
fn main() {
    match TcpStream::connect("eu.stratum.slushpool.com:3333") {
        Ok(mut stream) => {
            println!("Connected to POOL");
            // Subscribe to Pool
            let subscribe=concat!(r#"{"id": 1, "method": "mining.subscribe", "params":[]}"#,"\n");
            // Write Both messages
            let authorize= concat!(r#"{"id": 2, "method": "mining.authorize", "params":["ITA_Miner.worker1", ""]}"#,"\n");

            stream.write(subscribe.as_bytes()).unwrap();
            stream.write(authorize.as_bytes()).unwrap();
            println!("Sent mining.subscribe and mining.configure, awaiting reply...");
            let mut reader = BufReader::new(&stream);
            let mut data_con_sub: Vec<u8>= Vec::new();
            match reader.read_until(b'\n',&mut data_con_sub) {
                Ok(_) => {
                    let mut text = str::from_utf8(&data_con_sub).expect("Could not convert to String");
                    println!("\n\n\n");
                    println!("Answer: {}", text);
                    println!("unterer Buffer");
                    },
                Err(e) => {
                    println!("Failed to Read Data {}",e);
                }
            }
            // Jump into Routine to get permanent Data solv them them and submit shares
            
            loop{ 
                let mut data: Vec<u8>= Vec::new();
                match reader.read_until(b'\n',&mut data) {
                    Ok(_) => {
                        let mut text1 = str::from_utf8(&data).expect("Could not convert to String");
                        println!("\n\n\n");
                        println!("Answer: {}", text1);
                        println!("unterer Buffer");
                        },
                    Err(e) => {
                        println!("Failed to receive data: {}", e);
                    }
                }
            }
            
			//let message1=concat!(r#"{"params": ["0x931d5537aa741a62dadd03b33f74d9f45bcce70a", "password"], "id": 2, "method": "mining.authorize"}"#,"\n");
            
            //stream.write(message0.as_bytes()).unwrap();
			//stream.write(message1.as_bytes()).unwrap();
        },
        Err(e) => {
            println!("Failed to connect: {}", e);
        }
    }
    println!("Terminated.");
}
*/









