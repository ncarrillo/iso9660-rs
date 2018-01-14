use std::error::Error;
use std::fmt::{self, Display};
use std::num::ParseIntError;
use std::{io, str};

#[derive(Debug)]
pub enum ISOError {
    Io(io::Error),
    Utf8(str::Utf8Error),
    InvalidFs(&'static str),
    ParseInt(ParseIntError),
}

impl Display for ISOError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ISOError::Io(ref err) => write!(f, "IO error: {}", err),
            ISOError::Utf8(ref err) => write!(f, "UTF8 error: {}", err),
            ISOError::InvalidFs(msg) => write!(f, "Invalid ISO9660: {}", msg),
            ISOError::ParseInt(ref err) => write!(f, "Int parse error: {}", err),
        }
    }
}

impl Error for ISOError {
    fn description(&self) -> &str {
        match *self {
            ISOError::Io(ref err) => err.description(),
            ISOError::Utf8(ref err) => err.description(),
            ISOError::InvalidFs(_) => "Not a valid ISO9660 filesystem",
            ISOError::ParseInt(ref err) => err.description(),
        }
    }

    fn cause(&self) -> Option<&Error> {
        match *self {
            ISOError::Io(ref err) => Some(err),
            ISOError::Utf8(ref err) => Some(err),
            ISOError::ParseInt(ref err) => Some(err),
            _ => None,
        }
    }
}

impl From<io::Error> for ISOError {
    fn from(err: io::Error) -> ISOError {
        ISOError::Io(err)
    }
}

impl From<str::Utf8Error> for ISOError {
    fn from(err: str::Utf8Error) -> ISOError {
        ISOError::Utf8(err)
    }
}

impl From<ParseIntError> for ISOError {
    fn from(err: ParseIntError) -> ISOError {
        ISOError::ParseInt(err)
    }
}