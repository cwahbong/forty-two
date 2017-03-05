#[macro_use]
extern crate clap;
extern crate env_logger;
extern crate fte_module;
extern crate serde_json;

use clap::App;
use fte_module::arg;
use fte_module::types::RequestEvent;
use fte_module::util::exit;
use std::net::SocketAddr;

pub fn main() {
    env_logger::init().unwrap();
    let matches = App::new("fte-send")
        .version(crate_version!())
        .author(crate_authors!())
        .arg(arg::address().required(true))
        .arg(arg::port().required(true))
        .arg(arg::request_name().required(true))
        .arg(arg::request_args().required(true))
        .get_matches();
    let arg_addr = matches.value_of("addr").unwrap();
    let arg_port = value_t!(matches, "port", u16).unwrap_or_else(|e| e.exit());
    let arg_name = matches.value_of("name").unwrap();
    let arg_args = value_t!(matches, "args", serde_json::Value).unwrap_or_else(|e| e.exit());
    println!("=== req ===");
    println!("addr: {}", arg_addr);
    println!("port: {}", arg_port);
    println!("name: {}", arg_name);
    println!("args: {}", serde_json::to_string_pretty(&arg_args).unwrap());

    let addr = arg_addr.parse()
        .unwrap_or_else(|e| exit(1, "Not a valid address", e));
    let sock_addr = SocketAddr::new(addr, arg_port);
    let mut client = fte_module::client::connect(&sock_addr)
        .unwrap_or_else(|e| exit(1, "Failed to connect", e));
    let resp = client.request(RequestEvent {
            name: arg_name.into(),
            arguments: arg_args,
        })
        .unwrap_or_else(|e| exit(1, "Failed to request", e));
    println!("=== resp ===");
    println!("{}", serde_json::to_string_pretty(&resp).unwrap());
}
