use std::net::{TcpStream};
use std::io::{BufRead,BufReader, Write};
use std::str;
//https://www.youtube.com/watch?v=RJS6wMMwiA8
/*
struct First {
    id: String,
    method: String,
    params: Vec<String>,
}
struct Following {
    id: String,
    method: String,
    params: Vec<String>,
}


enum Answer{
    Answ_1(First),
    Answ_N(Following)
}

*/
fn main() {
    match TcpStream::connect("eu.stratum.slushpool.com:3333") {
        Ok(mut stream) => {
            println!("Connected to POOL");
            // Subscribe to Pool
            let subscribe=concat!(r#"{"id": 1, "method": "mining.subscribe", "params": ["", null, "eu.stratum.slushpool.com", 3333]}"#,"\n");
            // Write Both messages
            let authorize= concat!(r#" {"method": "mining.configure","id": 1,"params": [["minimum-difficulty"],{"minimum-difficulty.value": 1}]}"#,"\n");
            stream.write(authorize.as_bytes()).unwrap();
            stream.write(subscribe.as_bytes()).unwrap();
            println!("Sent mining.subscribe and mining.configure, awaiting reply...");
            let mut reader = BufReader::new(&stream);
            let mut data_con_sub: Vec<u8>= Vec::new();
            match reader.read_until(b'\n',&mut data_con_sub) {
                Ok(_) => {
                    let mut text = str::from_utf8(&data_con_sub).expect("Could not convert to String");
                    println!("\n\n\n");
                    println!("Answer: {}", text);
                    println!("\n\n\n");
                    },
                Err(e) => {
                    println!("Failed to Read Data {}",e);
                }
            }
            // Jump into Routine to get permanent Data solv them them and submit shares
            let mut data: Vec<u8>= Vec::new();
            loop{ 
                reader.flush()
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
