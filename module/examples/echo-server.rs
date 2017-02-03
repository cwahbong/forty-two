#[macro_use]
extern crate clap;
extern crate env_logger;
extern crate fte_module;

use clap::{Arg, App};
use std::net::SocketAddr;

pub fn main() {
    env_logger::init().unwrap();
    let matches = App::new("echo-server")
        .version(crate_version!())
        .author(crate_authors!())
        .arg(Arg::with_name("port")
            .short("p")
            .long("port")
            .takes_value(true)
            .required(true))
        .get_matches();
    let port = value_t!(matches, "port", u16).unwrap_or_else(|e| e.exit());
    let sock_addr = SocketAddr::new("127.0.0.1".parse().unwrap(), port);
    fte_module::server::serve(sock_addr);
}
