///// copyright 2019 Tom Schubert
/// 
/// STRATUM Protocoll
/// 
/// Client                                Server
///|                                     |
///| --------- mining.subscribe -------> |
///| --------- mining.authorize -------> |
///| <-------- mining.set_target ------- |
///|                                     |----
///| <---------- mining.notify --------- |<--/
///|                                     |
///| ---------- mining.submit ---------> |
/// 
use std::net::{TcpListener, TcpStream, Shutdown};
use std::io::BufWriter;
use serde_json::json;

struct TCPclient {
    stream: BufWriter<TcpStream>,
}

impl TCPclient {
    fn new(&self, adress) -> Self {
        TCPclient {
            &self.adress=adress,
            stream,
        }
    }
    fn intit_connection(&self){
        &self.stream  = loop{
            let mut trycounter=0;
            match BufWriter::new(TcpStream::connect(&self.adress)){
                Ok(_stream) => break _stream;
                Err(_err)=>
                {   if trycounter < 5{
                    println!("Trying again to connect")
                    trycounter+=1;
                }
                else{
                    println!("Shutting down Connection!")
                }
            }
            }
        }
            match streamTcpStream::connect(&self.adress) {{
               
        } 
        match stream.read(&mut data) {
            Ok(_) => {
                let text = from_utf8(&data).unwrap();
                println!("Answer: {}", text);
                },
            Err(e) => {
                println!("Failed to receive data: {}", e);
            }
        }
    }

}
fn main(){
    : TCPclient = TCPclient.new("eu.stratum.slushpool.com:3333");
    let tcp:tcp.intit_connection();

}

