//! URI: Error
//!


use std::fmt;
use std::fmt::Display;


#[derive(Debug, Eq, PartialEq)]
pub enum Error {
    Parse(ErrorParse),
    CannotOpenFile(ErrorCannotOpenFile),
    CannotReadFile(ErrorCannotReadFile),
    CannotDeserializeValue(ErrorCannotDeserializeValue),
    CannotDeserializeSchema(ErrorCannotDeserializeSchema),
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &*self {
            Error::Parse(err) => err.fmt(f),
            Error::CannotOpenFile(err) => err.fmt(f),
            Error::CannotReadFile(err) => err.fmt(f),
            Error::CannotDeserializeValue(err) => err.fmt(f),
            Error::CannotDeserializeSchema(err) => err.fmt(f),
        }
    }
}


#[derive(Debug, Eq, PartialEq)]
/// Error: Parse
pub struct ErrorParse {
    pub parse_error: String,
}

impl Display for ErrorParse {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.parse_error.to_string())
    }
}


#[derive(Debug, Eq, PartialEq)]
/// Error when trying to open a file from the local filesystem.
pub struct ErrorCannotOpenFile {
    pub filepath: String,
    pub io_err: String,
}

impl Display for ErrorCannotOpenFile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", format!("Cannot open file [{}]: {}", self.filepath, self.io_err))
    }
}


#[derive(Debug, Eq, PartialEq)]
/// Error when trying to read a file from the local filesystem.
pub struct ErrorCannotReadFile {
    pub filepath: String,
    pub io_err: String,
}

impl Display for ErrorCannotReadFile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", format!("Cannot read file [{}]: {}", self.filepath, self.io_err))
    }
}


#[derive(Debug, Eq, PartialEq)]
/// Error: Cannot deserialze value
pub struct ErrorCannotDeserializeValue {
    pub value_uri: String,
    pub des_err: String,
}

impl Display for ErrorCannotDeserializeValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", format!("Could not deserialze value at [{}]: {}", self.value_uri, self.des_err))
    }
}


#[derive(Debug, Eq, PartialEq)]
/// Error: Cannot deserialze schema
pub struct ErrorCannotDeserializeSchema {
    pub schema_uri: String,
    pub des_err: String,
}

impl Display for ErrorCannotDeserializeSchema {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", format!("Could not deserialze schema at [{}]: {}", self.schema_uri, self.des_err))
    }
}

