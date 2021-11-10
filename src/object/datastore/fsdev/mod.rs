//! Datastore: Filesystem
//! Datastore implementation for a generic filesystem
//! for testing purposes, intended to keep abstractions very clear
//! but also very inefficient


mod error; 


use serde;
use serde::{Deserialize, Serialize};
use std::path::Path;
use std::fs::{
    File,
    create_dir,
};

use crate::object::datastore::DataStore;
use crate::types::database::Database;
use crate::types::uri::URI;

use error::{
    Error,
    ErrorCouldNotCreateDir, ErrorCouldNotOpenFile,
    ErrorSerialization,
};


// Filesystem Dev
pub struct FSDev {
    uri: URI,
}


impl DataStore<Error> for FSDev {

    fn at(uri: &URI) -> Result<Self, Error> {
        return Ok(FSDev {
            uri: (*uri).clone(),
        });
    }

    fn sync_db(&self, db: &Database) -> Result<(), Error> {

        // Create directory
        let file_path_buf = self.uri.to_file_path();
        let dir = file_path_buf.as_path();
        create_dir_if_not_exists(dir)?;

        // Sync the metadata file
        return sync_metadata(dir, db);
    }
}


fn create_dir_if_not_exists(dir: &Path) -> Result<(), Error> {

    // If dir doesn't exist, create it
    // TODO Should verify URI is a directory
    if !dir.exists() {
        match create_dir(dir) {
            Err(io_err) => return Err(Error::CouldNotCreateDir(
                ErrorCouldNotCreateDir {
                    io_err: io_err,
                }
            )),
            Ok(_)       => return Ok(()),
        }
    }
    return Ok(());
}


fn sync_metadata(dir: &Path, db: &Database) -> Result<(), Error> {

    let db_metadata = DatabaseMetadata::from_database(db);
    let metadata_file_path = dir.join("metadata.yaml");

    let metadata_file = open_file(metadata_file_path.as_path())?;
    return write_yaml(metadata_file, &db_metadata);
}


//fn sync_schemas(dir: &str, db: &Database) -> Result<(), Error> {

//}


fn open_file(path: &Path) -> Result<File, Error> {
    match std::fs::File::create(path) {
        Ok(f)       => Ok(f),
        Err(io_err) => Err(Error::CouldNotOpenFile(
            ErrorCouldNotOpenFile {
                io_err: io_err,
            },
        )),
    }
}


fn write_yaml<W, T: ?Sized>(writer: W, value: &T) -> Result<(), Error>
where
    W: std::io::Write,
    T: serde::ser::Serialize,
{
    match serde_yaml::to_writer(writer, value) {
        Ok(_)         => return Ok(()),
        Err(yaml_err) => return Err(Error::Serialization(
            ErrorSerialization {
                ser_err: yaml_err.to_string(),
            },
        )),
    }
}


#[derive(Deserialize, Serialize)]
struct DatabaseMetadata {
    id: String,
}

impl DatabaseMetadata {

    pub fn from_database(db: &Database) -> DatabaseMetadata {
        return DatabaseMetadata {
            id: db.id.clone(),
        }
    }

}
