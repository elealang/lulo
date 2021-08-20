use http::Uri;
use std::fs::File;
use std::io::prelude::*;

use crate::types::base;
use crate::types::obj::value::{SetValue, Value};
use crate::types::obj::schema::Schema;


// FUNCTIONS > Public ----------------------------------------------------------

/// Get the value identified by the URI
pub fn schema(uri_str: &str) -> Result<Schema, Err> {
    let schema_str = string(uri_str)?;
    let schema_res: Result<base::schema::Schema, _> = serde_yaml::from_str(&schema_str);
    match schema_res {
        Ok(schema) => Ok(Schema::from_base(schema)),
        Err(serde_err) => Err(Err::CannotDesSchema(ErrCannotDesSchema {
            schema_uri: uri_str.to_string(),
            des_err: serde_err.to_string(),
        })),
    }
}

/// Get the value identified by the URI
pub fn value(uri_str: &str) -> Result<Value, Err> {
    let value_str = string(uri_str)?;
    let value_res: Result<base::value::SetValue, _> = serde_yaml::from_str(&value_str);
    println!("{}", uri_str);
    match value_res {
        Ok(value) => Ok(Value::Set(SetValue::from_base(value, Some(uri_str.to_string())))),
        Err(serde_err) => Err(Err::CannotDesValue(ErrCannotDesValue {
            value_uri: uri_str.to_string(),
            des_err: serde_err.to_string(),
        })),
    }
}

// FUNCTIONS > Private ---------------------------------------------------------

/// Get the string identified by the URI
fn string(uri_str: &str) -> Result<String, Err> {
    let uri = uri_str.parse::<Uri>().unwrap();
    match uri.scheme_str() {
        None => {
            return Err(Err::InvalidURI(ErrInvalidURI {
                uri: uri_str.to_string(),
            }))
        }
        Some("file") => return string_from_file(&uri_str[6..]),
        _ => {
            return Err(Err::SchemeNotSupported(ErrSchemeNotSupported {
                scheme: uri.scheme_str().unwrap().to_string(),
            }))
        }
    }
}

/// Get the string from a file
fn string_from_file(filepath: &str) -> Result<String, Err> {
    match File::open(filepath) {
        Ok(mut file) => {
            let mut contents = String::new();
            let read_res = file.read_to_string(&mut contents);
            match read_res {
                Ok(_) => return Ok(contents),
                Err(io_err) => {
                    return Err(Err::FileSystem(ErrFileSystem::CannotReadFile(
                        ErrCannotReadFile {
                            filepath: filepath.to_string(),
                            io_err: io_err.to_string(),
                        },
                    )))
                }
            }
        }
        Err(io_err) => Err(Err::FileSystem(ErrFileSystem::CannotOpenFile(
            ErrCannotOpenFile {
                filepath: filepath.to_string(),
                io_err: io_err.to_string(),
            },
        ))),
    }
}

// ERRORS ----------------------------------------------------------------------

#[derive(Debug, Eq, PartialEq)]
/// URI Error
pub enum Err {
    SchemeNotSupported(ErrSchemeNotSupported),
    InvalidURI(ErrInvalidURI),
    CannotDesSchema(ErrCannotDesSchema),
    CannotDesValue(ErrCannotDesValue),
    FileSystem(ErrFileSystem),
}

impl ToString for Err {
    fn to_string(&self) -> String {
        match &*self {
            Err::SchemeNotSupported(err) => err.to_string(),
            Err::InvalidURI(err) => err.to_string(),
            Err::CannotDesSchema(err) => err.to_string(),
            Err::CannotDesValue(err) => err.to_string(),
            Err::FileSystem(err) => err.to_string(),
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
/// URI Error when accessing the local filesystem.
pub enum ErrFileSystem {
    CannotOpenFile(ErrCannotOpenFile),
    CannotReadFile(ErrCannotReadFile),
}

impl ToString for ErrFileSystem {
    fn to_string(&self) -> String {
        match &*self {
            ErrFileSystem::CannotOpenFile(err) => err.to_string(),
            ErrFileSystem::CannotReadFile(err) => err.to_string(),
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
/// Error when trying to open a file from the local filesystem.
pub struct ErrCannotOpenFile {
    filepath: String,
    io_err: String,
}

impl ToString for ErrCannotOpenFile {
    fn to_string(&self) -> String {
        format!("Cannot open file [{}]: {}", self.filepath, self.io_err)
    }
}

#[derive(Debug, Eq, PartialEq)]
/// Error when trying to read a file from the local filesystem.
pub struct ErrCannotReadFile {
    filepath: String,
    io_err: String,
}

impl ToString for ErrCannotReadFile {
    fn to_string(&self) -> String {
        format!("Cannot read file [{}]: {}", self.filepath, self.io_err)
    }
}

#[derive(Debug, Eq, PartialEq)]
/// The URI Schema is not supported
pub struct ErrSchemeNotSupported {
    scheme: String,
}

impl ToString for ErrSchemeNotSupported {
    fn to_string(&self) -> String {
        format!("URI Scheme [{}] not supported", self.scheme)
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct ErrCannotDesValue {
    value_uri: String,
    des_err: String,
}

impl ToString for ErrCannotDesValue {
    fn to_string(&self) -> String {
        format!("Could not deserialze value at [{}]: {}", self.value_uri, self.des_err)
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct ErrCannotDesSchema {
    schema_uri: String,
    des_err: String,
}

impl ToString for ErrCannotDesSchema {
    fn to_string(&self) -> String {
        format!("Could not deserialze schema at [{}]: {}", self.schema_uri, self.des_err)
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct ErrInvalidURI {
    uri: String,
}

impl ToString for ErrInvalidURI {
    fn to_string(&self) -> String {
        format!("Invalid URI [{}]", self.uri)
    }
}

