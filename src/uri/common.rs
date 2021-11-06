//! URI Common Functions


use http::Uri;
use std::fs::File;
use std::io::prelude::*;

use super::error::{
    Err,
    ErrCannotOpenFile,
    ErrCannotReadFile,
    ErrFileSystem,
    ErrInvalidURI,
    ErrSchemeNotSupported,
};


/// Get the string identified by the URI
pub fn string(uri_str: &str) -> Result<String, Err> {
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
pub fn string_from_file(filepath: &str) -> Result<String, Err> {
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

