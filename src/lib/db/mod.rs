//! Lib: Database
//! Database library functions


pub mod error;


use crate::object::datastore::DataStore;
use crate::object::datastore::fsdev::FSDev;

use crate::types::database::Database;
use crate::types::uri::URI;
use error::{
    Error,
    ErrorSync,
};


/// Create a new database
pub fn new(uri: URI) -> Database {
    return Database::new(uri);
}


pub fn sync(db: &Database) -> Result<&Database, Error> {

    // TODO make parameter
    let err = FSDev::at(&db.uri)
        .map(|ds| ds.sync_db(db));
    match err {
        Ok(_)    => return Ok(db),
        Err(err) => return Err(Error::Sync(
            ErrorSync {
                err: err.to_string(),
            }
        )),
    }
}
    
