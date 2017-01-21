
use std::error;
use std::fmt;
use std::result;

pub use fte_module::types::Uuid;

#[derive(Debug)]
pub enum Error {
    UuidCollision,
    UuidInvalid,
    UuidNotExists,
}

impl Error {
    pub fn as_str(&self) -> &'static str {
        match *self {
            Error::UuidCollision => "uuid collision",
            Error::UuidInvalid => "uuid is invalid",
            Error::UuidNotExists => "uuid not exists",
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // TODO enhance this
        error::Error::description(self).fmt(f)
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        self.as_str()
    }
}

pub type Result<T> = result::Result<T, Error>;
