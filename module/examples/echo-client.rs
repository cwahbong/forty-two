#[macro_use]
extern crate clap;
extern crate env_logger;
extern crate fte_module;
extern crate serde_json;

use clap::{Arg, App};
use fte_module::types::RequestEvent;
use std::net::SocketAddr;

pub fn main() {
    env_logger::init().unwrap();
    let matches = App::new("echo-client")
        .version(crate_version!())
        .author(crate_authors!())
        .arg(Arg::with_name("addr")
             .short("a")
             .long("addr")
             .takes_value(true)
             .required(true))
        .arg(Arg::with_name("port")
             .short("p")
             .long("port")
             .takes_value(true)
             .required(true))
        .get_matches();
    let addr = matches.value_of("addr").unwrap();
    let port = value_t!(matches, "port", u16).unwrap_or_else(|e| e.exit());
    println!("addr: {}, port: {}", addr, port);

    let sock_addr = SocketAddr::new(addr.parse().unwrap(), port);
    let mut client = fte_module::client::connect(&sock_addr).unwrap();
    let request_event = RequestEvent {
        name: String::from("echo"),
        arguments: serde_json::Value::String(String::from("echo-arguments")),
    };
    let response_event = client.request(request_event).unwrap();
    println!("Response {}", serde_json::to_string(&response_event).unwrap());
}
