//! FSDev datastore util functions
//! Helper functions for common tasks

use serde::de::DeserializeOwned;
use std::fs::{create_dir, File};
use std::path::Path;

use super::error::{Error, ErrorCouldNotCreateDir, ErrorCouldNotOpenFile, ErrorSerialization};

pub enum FileMode {
    Read,
    Write,
}

pub fn open_file(path: &Path, mode: FileMode) -> Result<File, Error> {
    match mode {
        FileMode::Write => match std::fs::File::create(path) {
            Ok(f) => Ok(f),
            Err(io_err) => Err(Error::CouldNotOpenFile(ErrorCouldNotOpenFile {
                io_err: io_err,
            })),
        },
        FileMode::Read => match std::fs::File::open(path) {
            Ok(f) => Ok(f),
            Err(io_err) => Err(Error::CouldNotOpenFile(ErrorCouldNotOpenFile {
                io_err: io_err,
            })),
        },
    }
}

pub fn write_yaml<W, T: ?Sized>(writer: W, value: &T) -> Result<(), Error>
where
    W: std::io::Write,
    T: serde::ser::Serialize,
{
    match serde_yaml::to_writer(writer, value) {
        Ok(_) => return Ok(()),
        Err(yaml_err) => {
            return Err(Error::Serialization(ErrorSerialization {
                ser_err: yaml_err.to_string(),
            }))
        }
    }
}

pub fn read_yaml<R, T>(reader: R) -> Result<T, Error>
where
    R: std::io::Read,
    T: DeserializeOwned,
{
    match serde_yaml::from_reader(reader) {
        Ok(val) => return Ok(val),
        Err(yaml_err) => {
            return Err(Error::Serialization(ErrorSerialization {
                ser_err: yaml_err.to_string(),
            }))
        }
    }
}

pub fn create_dir_if_not_exists(dir: &Path) -> Result<(), Error> {
    // If dir doesn't exist, create it
    // TODO Should verify URI is a directory
    if !dir.exists() {
        match create_dir(dir) {
            Err(io_err) => {
                return Err(Error::CouldNotCreateDir(ErrorCouldNotCreateDir {
                    io_err: io_err,
                }))
            }
            Ok(_) => return Ok(()),
        }
    }
    return Ok(());
}
