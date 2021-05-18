use openssl::ssl::{SslMethod, SslConnectorBuilder};
use std::io::{Read, Write};
use std::net::TcpStream;

let connector = SslConnectorBuilder::new(SslMethod::tls()).unwrap().build();

let stream = TcpStream::connect("eu1.ethermine.org:4444").unwrap();

stream.write_all(b'{"id": 1, "method": "mining.subscribe", "params": []}\n').unwrap();

let mut res = vec![];
stream.read_to_end(&mut res).unwrap();
println!("{}", String::from_utf8_lossy(&res));

stream.write_all(b'{"params": ["0x931d5537aa741a62dadd03b33f74d9f45bcce70a", "password"], "id": 2, "method": "mining.authorize"}\n').unwrap();
stream.read_to_end(&mut res).unwrap();
println!("{}", String::from_utf8_lossy(&res));