use std::io;
use std::fmt;
use std::error::Error;
use std::result;

#[derive(Debug)]
pub enum KvsError {
    Io(io::Error),
    InvalidKey,
}

impl fmt::Display for KvsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            KvsError::Io(ref err) => err.fmt(f),
            KvsError::InvalidKey => write!(f, "Invalid Key"),
        }
    }
}

impl Error for KvsError {
    fn description(&self) -> &str {
        match *self {
            KvsError::Io(ref err) => err.description(),
            KvsError::InvalidKey => "Invalid Key",
        }
    }
}

impl From<io::Error> for KvsError {
    fn from(err: io::Error) -> KvsError {
        KvsError::Io(err)
    }
}

pub type Result<T> = result::Result<T, KvsError>;