pub type Result<T> = std::result::Result<T, Error>;
use std::{fmt::Display, io, str::Utf8Error, string::FromUtf8Error};


#[derive(Debug)]
pub struct Error {
    code: ErrorCode
}

impl Error {
    pub fn make_other(msg: String) -> Self {
        Error {
            code: ErrorCode::Other(msg.into_boxed_str())
        }
    }

    pub fn make_data(msg: String) -> Self {
        Error {
            code: ErrorCode::Data(msg.into_boxed_str())
        }
    }

    pub fn make_read(err: io::Error) -> Self {
        Error {
            code: ErrorCode::Io(err)
        }
    }

    pub fn make_string(err: FromUtf8Error) -> Self {
        Error {
            code: ErrorCode::StringParse(err)
        }
    }
}


#[derive(Debug)]
pub enum ErrorCode {
    Io(io::Error),
    StringParse(FromUtf8Error),
    Data(Box<str>),
    Other(Box<str>)
}


impl serde::de::Error for Error {
    fn custom<T>(msg: T) -> Self where T: std::fmt::Display {
        Error::make_other(msg.to_string())
    }
}

impl serde::ser::Error for Error {
    fn custom<T>(msg: T)->Self where T: Display {
        Error::make_other(msg.to_string())
    }
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Error::make_read(e)
    }
}

impl From<FromUtf8Error> for Error {
    fn from(e: FromUtf8Error) -> Self {
        Error::make_string(e)
    }
}

impl serde::de::StdError for Error {
    fn source(&self) -> Option<&(dyn serde::de::StdError + 'static)> {
        match self.code {
            ErrorCode::Io(ref err) => Some(err),
            _ => None
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.code, f)
    }
}

impl Display for ErrorCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            ErrorCode::Io(ref err) => Display::fmt(err, f),
            ErrorCode::StringParse(ref err) => Display::fmt(err, f),
            ErrorCode::Other(ref msg) => f.write_str(msg),
            ErrorCode::Data(ref msg) => f.write_str(msg)
        }
    }
}