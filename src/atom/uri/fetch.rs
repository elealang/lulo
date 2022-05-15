//! URI: Fetch
//! URIs uniquely and globally identify resources


use std::fs;

use crate::atom;
use super::URI;
use crate::object::schema::Schema;

use super::parse;
use super::error::{
    Error,
    ErrorCannotReadFile,
    ErrorCannotDeserializeSchema
};


// =================================================================================================
// PUBLIC
// =================================================================================================

/// Get the value identified by the URI
pub fn schema(uri: &URI) -> Result<Schema, Error> {
    let schema_str = string(uri)?;
    let schema_res: Result<atom::schema::Schema, _> = serde_yaml::from_str(&schema_str);
    match schema_res {
        Ok(schema) => Ok(Schema::from_atom(&schema)),
        Err(serde_err) => Err(Error::CannotDeserializeSchema(
            ErrorCannotDeserializeSchema {
                schema_uri: uri.to_string(),
                des_err: serde_err.to_string(),
            }
        )),
    }
}

/// Get the value identified by the URI
//pub fn value(uri_str: &str) -> Result<Value, Err> {
    //let value_str = string(uri_str)?;
    //let value_res: Result<SetValue, _> = serde_yaml::from_str(&value_str);
    //println!("{}", uri_str);
    //match value_res {
        //Ok(value) => Ok(Value::Set(SetValue::from_base(value, Some(uri_str.to_string())))),
        //Err(serde_err) => Err(Error::CannotDeserializeValue(
            //ErrCannotDeserializeValue {
                //value_uri: uri_str.to_string(),
                //des_err: serde_err.to_string(),
            //}
        //)),
    //}
//}


// =================================================================================================
// HELPERS
// =================================================================================================


/// Get the string identified by the URI
fn string(uri: &URI) -> Result<String, Error> {
    match uri.scheme {
        _ => return string_from_file(uri),
    }
}

/// Get the string from a file
fn string_from_file(uri: &URI) -> Result<String, Error> {
    let file_str_or_err = fs::read_to_string(uri.path.to_string());
    match file_str_or_err {
        Ok(file_str) => return Ok(file_str),
        Err(io_err) => {
            return Err(Error::CannotReadFile(
                ErrorCannotReadFile {
                    filepath: uri.path.to_string(),
                    io_err: io_err.to_string(),
                }
             ))
        }    
    }
}

