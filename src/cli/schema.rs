//! CLI Schema Loading

use clap;
use std::env;

use crate::types::obj::schema::Schema;
use super::error::{
    Error, 
    ErrorEnvVarNotSet, 
    ErrorFlagNotSet, 
    ErrorCouldNotLoadSchema,
};
use crate::uri;


/// Load the schema if one is provided
pub fn schema(matches: &clap::ArgMatches) -> Result<Schema,Error> {
    let from_flag = |_| {schema_from_flag(matches)};
    return schema_from_env().or_else(from_flag);
}

/// Schema from env vars
fn schema_from_env() -> Result<Schema,Error> {
    let schema_uri = schema_uri_from_env()?;
    return schema_from_uri(schema_uri);
}

/// Schema from CLI flag
fn schema_from_flag(matches: &clap::ArgMatches) -> Result<Schema,Error> {
    let schema_uri = schema_uri_from_flag(matches)?;
    return schema_from_uri(schema_uri);
}

/// Schema URI from environment variable
fn schema_uri_from_env() -> Result<String,Error> {
    match env::var("LULO_SCHEMA") {
        Ok(schema_uri) => Ok(schema_uri),
        Err(_)         => Err(Error::EnvVarNotSet(ErrorEnvVarNotSet{
            env_var: String::from("LULO_SCHEMA"),
        })),
    }
}

/// Schema URI from flag
fn schema_uri_from_flag(matches: &clap::ArgMatches) -> Result<String,Error> {
    match matches.value_of("schema") {
        Some(schema_uri) => Ok(schema_uri.to_string()),
        None             => Err(Error::FlagNotSet(ErrorFlagNotSet{
            flag: String::from("schema"),
        })),
    }
}

/// Schema from a URI
fn schema_from_uri(schema_uri: String) -> Result<Schema,Error> {
    match uri::schema(&schema_uri) {
        Ok(schema)   => Ok(schema),
        Err(uri_err) => Err(Error::CouldNotLoadSchema(ErrorCouldNotLoadSchema{
            uri: schema_uri,
            err: uri_err,
        })),
    }
}

