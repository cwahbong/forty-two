extern crate uuid;

use std::result;
use std::sync::mpsc;

pub use self::uuid::Uuid;

#[derive(Debug)]
pub enum Error {
    MpscRecieve(mpsc::RecvError),
    MpscSend(mpsc::SendError<Event>),
}

impl From<mpsc::SendError<Event>> for Error {
    fn from(error: mpsc::SendError<Event>) -> Self {
        Error::MpscSend(error)
    }
}

impl From<mpsc::RecvError> for Error {
    fn from(error: mpsc::RecvError) -> Self {
        Error::MpscRecieve(error)
    }
}

pub type Result<T> = result::Result<T, Error>;

// pub type EventArguments = serde_json::Value;
pub struct Arguments; // XXX

pub struct Event {
    pub name: String,
    pub kind: String,
    pub arguments: Arguments,
}

pub struct Message {
    pub sender: Uuid,
    pub receiver: Uuid,
    pub event: Event,
}

