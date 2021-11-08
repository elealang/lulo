//! Lib Database
//! Database library functions


use crate::types::database::Database;
use crate::types::uri::URI;


/// Create a new database
pub fn new(uri: URI) -> Database {
    return Database::new(uri);
}
