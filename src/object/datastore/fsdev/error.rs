//! FSDev datastore errors
//! Datastore errors

use std::io;

pub enum Error {
    CouldNotCreateDir(ErrorCouldNotCreateDir),
    CouldNotOpenFile(ErrorCouldNotOpenFile),
    Serialization(ErrorSerialization),
}

impl ToString for Error {
    fn to_string(&self) -> String {
        match &*self {
            Error::CouldNotCreateDir(err) => err.to_string(),
            Error::CouldNotOpenFile(err) => err.to_string(),
            Error::Serialization(err) => err.to_string(),
        }
    }
}

pub struct ErrorCouldNotCreateDir {
    pub io_err: io::Error,
}

impl ToString for ErrorCouldNotCreateDir {
    fn to_string(&self) -> String {
        return self.io_err.to_string();
    }
}

pub struct ErrorSerialization {
    pub ser_err: String,
}

impl ToString for ErrorSerialization {
    fn to_string(&self) -> String {
        return self.ser_err.clone();
    }
}

pub struct ErrorCouldNotOpenFile {
    pub io_err: io::Error,
}

impl ToString for ErrorCouldNotOpenFile {
    fn to_string(&self) -> String {
        return self.io_err.to_string();
    }
}
