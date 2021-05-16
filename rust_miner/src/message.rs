/// Different kind of messages
use serde_json::json;

pub fn contentbuilder(typee: messagetype) {

}

enum messagetype {
    submit(String),
    authorize(String),
    subscribe(String),
}
let home = IpAddr::V4(String::from("127.0.0.1"));

let loopback = IpAddr::V6(String::from("::1"));