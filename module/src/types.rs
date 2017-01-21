extern crate uuid;

use std::result;

pub use self::uuid::Uuid;

pub enum Error {
    InvalidKind,
}

pub type Result<T> = result::Result<T, Error>;

// pub type EventArguments = serde_json::Value;
pub struct EventArguments; // XXX

pub struct Event {
    pub name: String,
    pub kind: String,
    pub arguments: EventArguments,
}
