//! Lib: Database
//! Database library functions

pub mod error;

use crate::object::datastore::fsdev;
use crate::object::datastore::fsdev::FSDev;
use crate::object::datastore::DataStore;

use crate::types::database::Database;
use crate::types::uri::URI;
use error::{Error, ErrorRead, ErrorSync};

/// Create a new database
/// call this new atom? need a name for base types still...
pub fn new(uri: URI) -> Database {
    return Database::new(uri);
}

pub fn sync(db: &Database) -> Result<&Database, Error> {
    // TODO make parameter
    let err = FSDev::at(&db.uri).map(|ds| ds.sync_db(db));
    match err {
        Ok(_) => return Ok(db),
        Err(err) => {
            return Err(Error::Sync(ErrorSync {
                err: err.to_string(),
            }))
        }
    }
}

/// Load a database object from its URI
pub fn database(uri: &URI) -> Result<Database, Error> {
    return db_from_ds(uri).map_err(|err| {
        Error::Read(ErrorRead {
            err: err.to_string(),
        })
    });
}

/// Database from datastore
fn db_from_ds(uri: &URI) -> Result<Database, fsdev::error::Error> {
    let datastore = FSDev::at(uri)?;
    return datastore.database();
}
