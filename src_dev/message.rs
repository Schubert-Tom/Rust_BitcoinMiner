use std::net::{TcpStream};
use std::io::{BufRead,BufReader, Write};
use serde::{Serialize, Deserialize};
extern crate serde_json;
use super::*;

pub enum Message{
    subscribe(Subscribe),
    authorize(Authorize),
    submit(Submit),
    notify(Notify),
    set_difficulty(SetDifficulty)
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Subscribe{
    id:u8,
    method:String,
    params:Vec<String>,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Authorize{
    
}
#[derive(Serialize, Deserialize, Debug)]                                   
pub struct Notify{
    pub job_id: u32
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Submit{
    pub job_id: u32
}
#[derive(Serialize, Deserialize, Debug)]
pub struct SetDifficulty{
    pub job_id: u32
}
impl Subscribe{
    fn new()->Self{
        Subscribe{
            id:1,
            method:"mining.subscribe".to_owned(),
            params:vec![],
        }
    }
}
/*impl Authorize{
    fn new()->Self{
        Authorize{
            id:1,
            method:"mining.subscribe".to_owned(),
            params:vec![],
        }
    }
}
impl Notify{
    fn new()->Self{
        Notify{
            id:1,
            method:"mining.subscribe".to_owned(),
            params:vec![],
        }
    }
}
impl Submit{
    fn new()->Self{
        Submit{
            id:1,
            method:"mining.subscribe".to_owned(),
            params:vec![],
        }
    }
}
impl SetDifficulty{
    fn new()->Self{
        SetDifficulty{
            id:1,
            method:"mining.subscribe".to_owned(),
            params:vec![],
        }
    }
}

