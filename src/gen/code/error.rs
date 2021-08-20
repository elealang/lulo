//! Code Generation Errors

use crate::types::base::typ::TypeId;


#[derive(Debug, Eq, PartialEq)]
pub enum Error {
    TypeNotInSchema(ErrorTypeNotInSchema), 
    CouldNotRenderTemplate(ErrorCouldNotRenderTemplate), 
    NotSupported(ErrorNotSupported), 
}

impl ToString for Error {
    fn to_string(&self) -> String {
        match &*self {
            Error::TypeNotInSchema(err)        => err.to_string(),
            Error::CouldNotRenderTemplate(err) => err.to_string(),
            Error::NotSupported(err)           => err.to_string(),
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
pub struct ErrorCouldNotRenderTemplate {
}

impl ToString for ErrorCouldNotRenderTemplate {
    fn to_string(&self) -> String {
        format!("Could not render template")
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct ErrorNotSupported {
    pub msg: String,
}

impl ToString for ErrorNotSupported {
    fn to_string(&self) -> String {
        format!("Not supported: {}", self.msg)
    }
}
