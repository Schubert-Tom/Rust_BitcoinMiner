use std::net::{TcpStream};
use std::io::{Read, Write};
use std::str::from_utf8;
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
            println!("Connected to Slushpool");
            // Configure connection and set diff to min value
            let configure= concat!(r#" {"method": "mining.configure","id": 1,"params": [["minimum-difficulty"],{"minimum-difficulty.value": 1}]}"#,"\n");
            // Subscribe to Pool
            let subscribe=concat!(r#"{"id": 1, "method": "mining.subscribe", "params": ["", null, "eu.stratum.slushpool.com", 3333]}"#,"\n");
            // Write Both messages
            stream.write(configure.as_bytes()).unwrap();
            stream.write(subscribe.as_bytes()).unwrap();
            println!("Sent mining.subscribe and mining.configure, awaiting reply...");
            let mut data_con_sub = [0 as u8; 9000]; // using 9000 byte buffer
            match stream.read(&mut data_con_sub) {
                Ok(_) => {
                    let text = from_utf8(&data_con_sub).unwrap();
                    println!("Answer: {}", text);
                    },
                Err(e) => {
                    println!("Failed to receive data: {}", e);
                }
            }
            
            let mut data = [0 as u8; 9000]; // using 9000 byte buffer
            // Jump into Routine to get permanent Data solv them them and submit shares
            loop{
                match stream.read(&mut data) {
                    Ok(_) => {
                        let text1 = from_utf8(&data).unwrap();
                        println!("Answer: {}", text1);
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
