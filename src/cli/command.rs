//! Lulo Command Line Interface

use crate::uri;
use crate::validate;
use crate::gen::{
    Artifact,
    ArtifactProgLangTypes,
};
use crate::gen;
use crate::types::base::typ::TypeId;
use crate::types::obj::value::Value;
use crate::types::obj::typ::Type;
use crate::types::obj::schema::Schema;


// COMMAND > CHECK -------------------------------------------------------------

/// Type check a value
///
///   Example:
///   lulo --schema file://local/dir/schema.yaml check file://local/dir/value.yaml app.character 
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

/// Check Command Input
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

// COMMNAD > GEN ---------------------------------------------------------------

/// Generate artifacts based on a schema
///
///   Example:
///   lulo --schema file://local/dir/schema.yaml gen rust
pub fn gen(schema: Schema, matches: &clap::ArgMatches) -> Result<String, Error> {
    let input = GenInput::new(matches)?;
    match gen::string(&schema, &input.artifact) {
        Ok(s)  => return Ok(s),
        Err(e) => return Err(Error::CouldNotGenerateArtifact(
            ErrorCouldNotGenerateArtifact{
                err: e,
            }
        )),
    };
}

/// Gen Command Input
struct GenInput {
    artifact: Artifact,
}

impl GenInput {

    pub fn new(matches: &clap::ArgMatches) -> Result<Self, Error> {

        let artifact_type = matches.value_of("artifact-type").ok_or(
            Error::MissingInput(ErrorMissingInput{
                err_msg: String::from("Expected first argument 'artifact-type'"),
            }
        ))?;

        match artifact_type {
            "rust" => {
                return Ok(GenInput {
                    artifact: Artifact::ProgLangTypes(ArtifactProgLangTypes::Rust),
                });
            },
            _ => {
                return Err(Error::UnknownArtifactType(ErrorUnknownArtifactType{
                    artifact_type: artifact_type.to_string(),
                }))
            },
        }
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
pub enum Error {
    MissingInput(ErrorMissingInput),
    CouldNotLoadValue(ErrorCouldNotLoadValue),
    SchemaMissingType(ErrorSchemaMissingType),
    TypeCheckFailed(ErrorTypeCheckFailed),
    CouldNotGenerateArtifact(ErrorCouldNotGenerateArtifact),
    UnknownArtifactType(ErrorUnknownArtifactType)
}

impl ToString for Error {
    fn to_string(&self) -> String {
        match &*self {
            Error::MissingInput(err)             => err.to_string(),
            Error::CouldNotLoadValue(err)        => err.to_string(),
            Error::SchemaMissingType(err)        => err.to_string(),
            Error::TypeCheckFailed(err)          => err.to_string(),
            Error::CouldNotGenerateArtifact(err) => err.to_string(),
            Error::UnknownArtifactType(err)      => err.to_string(),
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
/// Missing Input
pub struct ErrorMissingInput {
    err_msg: String,
}

impl ToString for ErrorMissingInput {
    fn to_string(&self) -> String {
        return self.err_msg.clone();
    }
}

#[derive(Debug, Eq, PartialEq)]
/// Could not load value from URI
pub struct ErrorCouldNotLoadValue {
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
pub struct ErrorSchemaMissingType {
    type_id: TypeId,
}

impl ToString for ErrorSchemaMissingType {
    fn to_string(&self) -> String {
        return format!("Schema does not have type [{}]", self.type_id.to_string())
    }
}

#[derive(Debug, Eq, PartialEq)]
/// Could not load value from URI
pub struct ErrorTypeCheckFailed {
    err: validate::Error,
}

impl ToString for ErrorTypeCheckFailed {
    fn to_string(&self) -> String {
        return format!("Type check failed with: {}", self.err.to_string());
    }
}

#[derive(Debug, Eq, PartialEq)]
/// Could not load value from URI
pub struct ErrorCouldNotGenerateArtifact {
    err: gen::error::Error,
}

impl ToString for ErrorCouldNotGenerateArtifact {
    fn to_string(&self) -> String {
        return format!("Could not generate artifact: {}", self.err.to_string());
    }
}

#[derive(Debug, Eq, PartialEq)]
/// Could not load value from URI
pub struct ErrorUnknownArtifactType {
    artifact_type: String,
}

impl ToString for ErrorUnknownArtifactType {
    fn to_string(&self) -> String {
        format!("Unknown artifact type: {}", self.artifact_type)
    }
}
