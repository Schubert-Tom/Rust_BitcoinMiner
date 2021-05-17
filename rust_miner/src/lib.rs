mod job;
pub use crate::job::Job;
//pub mod miner;
//pub use crate::miner::*;
pub mod connect;
pub use crate::connect::*;

pub fn replace_symbols(s: String)->String{
    return str::replace(&str::replace(&s,r#"\"#,""),r#"""#,"");
}