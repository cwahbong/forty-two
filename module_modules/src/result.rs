use serde_json;
use std::error;
use std::fmt;
use std::net::AddrParseError;
use std::result;
use std::sync::PoisonError;

#[derive(Serialize, Deserialize, Debug)]
pub enum Error {
    NotImplemented,
    InvalidApi,
    BadParameter,
    LockPoisoned,
    AlreadyRegisted,
    Timeout,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        error::Error::description(self).fmt(f)
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::NotImplemented => "Not implemented",
            Error::BadParameter => "Bad parameter",
            Error::InvalidApi => "Invalid api",
            Error::LockPoisoned => "Lock is poisoned",
            Error::AlreadyRegisted => "Already registed",
            Error::Timeout => "Timeout"
        }
    }
}

impl<T> From<PoisonError<T>> for Error {
    fn from(_: PoisonError<T>) -> Self {
        Error::LockPoisoned
    }
}

impl From<serde_json::Error> for Error {
    fn from(_: serde_json::Error) -> Self {
        Error::BadParameter
    }
}

impl From<AddrParseError> for Error {
    fn from(_: AddrParseError) -> Self {
        Error::BadParameter
    }
}

// impl From<serde_json::Error> for Error {
//     fn from(error: serde_json::Error) -> Self {
//         Error::Json(error)
//     }
// }

pub type Result<T> = result::Result<T, Error>;
