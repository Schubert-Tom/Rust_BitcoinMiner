///////////////////////////// Working Code for building connection /////////////////////////
use std::net::{TcpStream};
use std::io::{BufRead,BufReader, Write};
use std::fmt::{self, Debug, Formatter};
use std::str;
use serde_json::{Result, Value};
use hex::*;
use lib::*;

pub struct PoolConnection{
    pub username:String,
    pub adress:String,
    pub stream:TcpStream,
    // So viele aktive Jobs 
    pub active_jobs:Vec<Job>,
    pub extranonce1: Vec<u8>,
    pub extranonce2_size: u32,
}

impl PoolConnection{
    fn new (username: &str ,adress: &str)->Self
    {
        let stream = match TcpStream::connect(adress) {
            Ok(mut stream) =>{stream}
            Err(e) => {panic!("Failed to connect: {}", e);}
        };
            return PoolConnection{
                    extranonce1:Vec::new(),
                    extranonce2_size:0,
                    username:username.to_owned(),
                    adress:adress.to_owned(),
                    stream:stream,
                    active_jobs:vec![],
                };
    }
    fn send_message(& mut self, message:&str){
       match self.stream.write(message.as_bytes()){
        Ok(mut stream) =>{stream}
        Err(e) => {panic!("Failed to connect: {}", e);}
       };
    }
    fn subscribe_pool(& mut self){
        let subscribe=concat!(r#"{"id": 1, "method": "mining.subscribe", "params":[]}"#,"\n");
        self.send_message(subscribe);
    }
    fn authorize_pool(& mut self){
        let authorize= concat!(r#"{"id": 2, "method": "mining.authorize", "params":["ITA_Miner.worker1", ""]}"#,"\n");
        self.send_message(authorize);
    }
    fn create_jobs(& mut self,json: Value)->Job{
        // Notify Response
            // Set VAlues for Mining Function Job
            let params=&json["params"];
            let job_id:Vec<u8>=vec![];//hex::decode(replace_symbols(params[0].to_string())).unwrap();
            let extranonce1=&self.extranonce1;
            let extranonce2=self.extranonce2_size;
            let prev_block_hash: Vec<u8>=hex::decode(replace_symbols(params[1].to_string())).expect("Conversion failure");
            let coinb1: Vec<u8>=hex::decode(replace_symbols(params[2].to_string())).expect("Conversion failure");
            let coinb2: Vec<u8>=hex::decode(replace_symbols(params[3].to_string())).expect("Conversion failure");
            // Create Merklebranch VEctor of u8 sized hex
            let mut merkle_branch: [Vec<u8>; 12] = [vec![],vec![],vec![],vec![],vec![],vec![],vec![],vec![],vec![],vec![],vec![],vec![]];
            //println!("{}",merkle_branch.len()-1);
            for i in 0..merkle_branch.len()-1{
                //println!("{}",replace_symbols(params[4][i].to_string()));
                merkle_branch[i]=hex::decode(replace_symbols(params[4][i].to_string())).expect("Conversion failure");
            }
            let version:Vec<u8>=hex::decode(replace_symbols(params[5].to_string())).expect("Conversion failure");
            let nbits:u32= u32::from_str_radix(&replace_symbols(params[6].to_string()), 16).expect("Conversion failure");
            let ntime:Vec<u8>=hex::decode(replace_symbols(params[7].to_string())).expect("Conversion failure");
            return Job::new(job_id,extranonce1.to_vec(),extranonce2,prev_block_hash,coinb1,coinb2,merkle_branch,version,nbits,ntime);
    }
    fn openstream(& mut self){
        // no function for flushing the buffer so each loop a new one gets created
        // Two reads exactly behind each other because of fast incoming Messages and for loops is sometimes too slow
        self.subscribe_pool();
        self.authorize_pool();
        // Read from INIT MEssage
        loop{
        let mut reader = BufReader::new(&self.stream);
        let mut data: Vec<u8>= Vec::new();
        match reader.read_until(b'\n',&mut data) {
            Ok(_) => {
                let text = str::from_utf8(&data).expect("Could not convert to String");
                println!("Answer: {}", text);
                println!("unterer Buffer");
                let json: Value =match serde_json::from_str(text){
                    Ok(json) => json,
                    Err(_) => continue,
                };
                // Handle First MEssage
                if json["id"]==1{
                    // Set VAlues for ExtraNonce1 and ExtraNonce_Size2
                    println!("Extranonce1: {}", str::replace(&json["result"][1].to_string(),r#"""#,""));
                    println!("Size of ExtraNonce2 {}", json["result"][2]);
                    self.extranonce2_size=u32::from_str_radix(&json["result"][2].to_string(), 16).expect("Conversion failure");
                    self.extranonce1=hex::decode(replace_symbols(json["result"][1].to_string())).expect("Conversion failure");
                }
                // NEw Job
                if json["method"]=="mining.notify"{
                    // IF clean_jobs flag is true reset active jobs stack
                    if json["params"][8]==true{
                        println!("Reset active Job stack!");
                        self.active_jobs=Vec::new();
                    }
                    else{
                    let new_job:Job=self.create_jobs(json);
                    self.active_jobs.push(new_job);
                    };
                    // Check return Type if Return is Null repeat 

                    // if start_miner(new_job)!=null{

                    //};
                };
                
            },
            Err(e) => {
                println!("Failed to Read Data {}",e);
            }
        }
        
    }
}
}

impl Debug for PoolConnection {
    fn fmt (&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "PoolConnection: {}  {} ",
            &self.adress,
            &self.active_jobs.len(),
        )
    }
}

fn main(){
    let mut conn=PoolConnection::new("TEst","eu.stratum.slushpool.com:3333");
    conn.openstream();
}
