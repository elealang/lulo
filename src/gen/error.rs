//! Artifact Generation Errors

use crate::types::base::typ::TypeId;
use crate::gen::code;


#[derive(Debug, Eq, PartialEq)]
pub enum Error {
    TypeNotInSchema(ErrorTypeNotInSchema), 
    CouldNotGenerateCode(ErrorCouldNotGenerateCode), 
}

impl ToString for Error {
    fn to_string(&self) -> String {
        match &*self {
            Error::TypeNotInSchema(err)      => err.to_string(),
            Error::CouldNotGenerateCode(err) => err.to_string(),
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct ErrorTypeNotInSchema {
    pub missing_type_id: TypeId,
}

impl ToString for ErrorTypeNotInSchema {
    fn to_string(&self) -> String {
        format!("Type not in schema: {}", self.missing_type_id.to_string())
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct ErrorCouldNotGenerateCode {
    pub err: code::error::Error,
}

impl ToString for ErrorCouldNotGenerateCode {
    fn to_string(&self) -> String {
        format!("Could not generate code: {}", self.err.to_string())
    }
}
