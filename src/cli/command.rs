//! Lulo Command Line Interface

use crate::uri;
use crate::validate;
use crate::types::base::typ::TypeId;
use crate::types::obj::value::Value;
use crate::types::obj::typ::Type;
use crate::types::obj::schema::Schema;


/// Type check a value
///
///   Example:
///   lulo check /local/dir/value.yaml app.character --schema /local/dir/schema.yaml
pub fn check(schema: Schema, matches: &clap::ArgMatches) {
    match _check(schema, matches) {
        Ok(validation) => print!("{}", validation.to_string()),
        Err(err)       => print!("{}", err.to_string()),
    }
}

fn _check(schema: Schema, matches: &clap::ArgMatches) -> Result<validate::IsTypeValidation,Error> {
    let input = CheckInput::new(matches)?;
    let value = value_from_uri(input.value_uri)?;
    let typ = type_with_id(&schema, &input.type_id)?; 
    return _is_type(&value, &typ, &schema);
}

struct CheckInput {
    value_uri: String,
    type_id: TypeId,
}

impl CheckInput {

    pub fn new(matches: &clap::ArgMatches) -> Result<Self,Error> {

        let value_uri = matches.value_of("value").ok_or(
            Error::MissingInput(ErrorMissingInput{
                err_msg: String::from("Expected flag 'value'"),
            }
        ))?;

        let type_id_str = matches.value_of("type").ok_or(
            Error::MissingInput(ErrorMissingInput{
                err_msg: String::from("Expected flag 'type'"),
            }
        ))?;

        Ok(CheckInput {
            value_uri: value_uri.to_string(),
            type_id: TypeId::from_string(type_id_str),
        })
    }
}

// UTILITY ---------------------------------------------------------------------

/// Value from a URI
fn value_from_uri(value_uri: String) -> Result<Value,Error> {
    match uri::value(&value_uri) {
        Ok(value)   => Ok(value),
        Err(uri_err) => Err(Error::CouldNotLoadValue(ErrorCouldNotLoadValue{
            value_uri: value_uri,
            err: uri_err,
        })),
    }
}

fn type_with_id<'a>(schema: &'a Schema, type_id: &TypeId) -> Result<&'a Type,Error> {
    return schema.type_with_id(type_id).ok_or(
        Error::SchemaMissingType(ErrorSchemaMissingType{
            type_id: type_id.clone(), 
        })
    );
}

fn _is_type(
    value: &Value, 
    typ: &Type, 
    schema: &Schema,
) -> Result<validate::IsTypeValidation,Error> {

    match validate::is_type(value, typ, schema) {
        Ok(v)    => Ok(v),
        Err(err) => Err(Error::TypeCheckFailed(ErrorTypeCheckFailed{
            err: err,
        })),
    }
}

// ERRORS ----------------------------------------------------------------------

#[derive(Debug, Eq, PartialEq)]
/// Command Error
enum Error {
    MissingInput(ErrorMissingInput),
    CouldNotLoadValue(ErrorCouldNotLoadValue),
    SchemaMissingType(ErrorSchemaMissingType),
    TypeCheckFailed(ErrorTypeCheckFailed),
}

impl ToString for Error {
    fn to_string(&self) -> String {
        match &*self {
            Error::MissingInput(err)      => err.to_string(),
            Error::CouldNotLoadValue(err) => err.to_string(),
            Error::SchemaMissingType(err) => err.to_string(),
            Error::TypeCheckFailed(err)   => err.to_string(),
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
/// Missing Input
struct ErrorMissingInput {
    err_msg: String,
}

impl ToString for ErrorMissingInput {
    fn to_string(&self) -> String {
        return self.err_msg.clone();
    }
}

#[derive(Debug, Eq, PartialEq)]
/// Could not load value from URI
struct ErrorCouldNotLoadValue {
    value_uri: String,
    err: uri::Err,
}

impl ToString for ErrorCouldNotLoadValue {
    fn to_string(&self) -> String {
        return format!("Could not load value [{}]: {}", self.value_uri, self.err.to_string())
    }
}

#[derive(Debug, Eq, PartialEq)]
/// Could not load value from URI
struct ErrorSchemaMissingType {
    type_id: TypeId,
}

impl ToString for ErrorSchemaMissingType {
    fn to_string(&self) -> String {
        return format!("Schema does not have type [{}]", self.type_id.to_string())
    }
}

#[derive(Debug, Eq, PartialEq)]
/// Could not load value from URI
struct ErrorTypeCheckFailed {
    err: validate::Error,
}

impl ToString for ErrorTypeCheckFailed {
    fn to_string(&self) -> String {
        return format!("Type check failed with: {}", self.err.to_string());
    }
}
