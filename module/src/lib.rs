#[macro_use]
extern crate log;

pub mod api;
pub mod module;
pub mod part;
pub mod types;
pub mod utils;

// Need a sender for high level event communication
//
// Regist
// * Function
// * Stream Receiver
//
// Sender for
// * Remote function call
// * Stream Sender
