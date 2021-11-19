//! Lib: Schema
//! Schema public library functions


pub mod error;


use error::Error;


/// Create a new database
pub fn object_from_uri(uri: &URI) -> Schema<Schema, Error> {
    return Database::new(uri);
}

