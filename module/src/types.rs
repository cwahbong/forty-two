extern crate uuid;

use std::result;
use std::sync::mpsc;

pub use self::uuid::Uuid;

#[derive(Debug)]
pub enum Error {
    InvalidStatus,
    InvalidKind,
    MpscRecieve(mpsc::RecvError),
}

impl From<mpsc::RecvError> for Error {
    fn from(error: mpsc::RecvError) -> Self {
        Error::MpscRecieve(error)
    }
}

pub type Result<T> = result::Result<T, Error>;

// pub type EventArguments = serde_json::Value;
pub struct EventArguments; // XXX

pub struct Event {
    pub name: String,
    pub kind: String,
    pub arguments: EventArguments,
}
