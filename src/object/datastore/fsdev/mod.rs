//! Datastore: Filesystem
//! Datastore implementation for a generic filesystem
//! for testing purposes, intended to keep abstractions very clear
//! but also very inefficient

pub mod error;
mod util;

use serde;
use serde::{Deserialize, Serialize};
use std::path::Path;

use crate::object::datastore::DataStore;
use crate::atom::database::Database;
use crate::atom::uri::URI;

use error::Error;

// Filesystem Dev
pub struct FSDev {
    uri: URI,
}

#[derive(Deserialize, Serialize)]
struct DatabaseMetadata {
    id: String,
}

impl DatabaseMetadata {
    pub fn from_database(db: &Database) -> DatabaseMetadata {
        return DatabaseMetadata { id: db.id.clone() };
    }
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
        util::create_dir_if_not_exists(dir)?;

        // Sync the metadata file
        return sync_metadata(dir, db);
    }

    fn database(&self) -> Result<Database, Error> {
        let file_path_buf = self.uri.to_file_path();
        let dir = file_path_buf.as_path();

        let db_metadata = read_metadata(dir)?;

        return Ok(Database {
            id: db_metadata.id.clone(),
            uri: self.uri.clone(),
            schemas: Vec::new(),
        });
    }
}

fn sync_metadata(dir: &Path, db: &Database) -> Result<(), Error> {
    let db_metadata = DatabaseMetadata::from_database(db);
    let metadata_file_path = dir.join("metadata.yaml");

    let metadata_file = util::open_file(metadata_file_path.as_path(), util::FileMode::Write)?;
    return util::write_yaml(metadata_file, &db_metadata);
}

fn read_metadata(dir: &Path) -> Result<DatabaseMetadata, Error> {
    let metadata_file_path = dir.join("metadata.yaml");
    println!("{:?}", metadata_file_path);

    let metadata_file = util::open_file(metadata_file_path.as_path(), util::FileMode::Read)?;

    return util::read_yaml(metadata_file);
}

//fn sync_schemas(dir: &str, db: &Database) -> Result<(), Error> {

//}
