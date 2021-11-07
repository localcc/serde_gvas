pub type Result<T> = std::result::Result<T, Error>;
use std::fmt::Display;


#[derive(Debug)]
pub struct Error {
    code: ErrorCode
}

impl Error {
    pub fn make_syntax() -> Self {
        make_error(String::from("Invalid Syntax"))
    }
}


#[derive(Debug)]
pub enum ErrorCode {
    InvalidFile(Box<str>)
}


impl serde::de::Error for Error {

    fn custom<T>(msg: T) -> Self where T: std::fmt::Display {
        make_error(msg.to_string())
    }
}

fn make_error(mut msg: String) -> Error {
    Error {
        code: ErrorCode::InvalidFile(msg.into_boxed_str())
    }
}

impl serde::de::StdError for Error {
    fn source(&self) -> Option<&(dyn serde::de::StdError + 'static)> {
        None
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.code, f)
    }
}

impl Display for ErrorCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let ErrorCode::InvalidFile(msg) = self;
        f.write_str(msg)
    }
}