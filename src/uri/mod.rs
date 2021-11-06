//! URI Module
//! URIs uniquely and globally identify resources


pub mod common;
pub mod error;


use crate::types::atom;
use crate::types::object::value::{SetValue, Value};
use crate::types::object::schema::Schema;

use error::{
    Err,
    ErrCannotDesSchema,
    ErrCannotDesValue,
};


/// Get the value identified by the URI
pub fn schema(uri_str: &str) -> Result<Schema, Err> {
    let schema_str = common::string(uri_str)?;
    let schema_res: Result<atom::schema::Schema, _> = serde_yaml::from_str(&schema_str);
    match schema_res {
        Ok(schema) => Ok(Schema::from_atom(schema)),
        Err(serde_err) => Err(error::Err::CannotDesSchema(ErrCannotDesSchema {
            schema_uri: uri_str.to_string(),
            des_err: serde_err.to_string(),
        })),
    }
}

/// Get the value identified by the URI
pub fn value(uri_str: &str) -> Result<Value, Err> {
    let value_str = common::string(uri_str)?;
    let value_res: Result<atom::value::SetValue, _> = serde_yaml::from_str(&value_str);
    println!("{}", uri_str);
    match value_res {
        Ok(value) => Ok(Value::Set(SetValue::from_base(value, Some(uri_str.to_string())))),
        Err(serde_err) => Err(Err::CannotDesValue(ErrCannotDesValue {
            value_uri: uri_str.to_string(),
            des_err: serde_err.to_string(),
        })),
    }
}

