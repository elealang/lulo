//! CLI Error Types

use crate::uri;


#[derive(Debug, Eq, PartialEq)]
pub enum Error {
    EnvVarNotSet(ErrorEnvVarNotSet),
    FlagNotSet(ErrorFlagNotSet),
    CouldNotLoadSchema(ErrorCouldNotLoadSchema),
}

impl ToString for Error {
    fn to_string(&self) -> String {
        match &*self {
            Error::EnvVarNotSet(err)       => err.to_string(),
            Error::FlagNotSet(err)         => err.to_string(),
            Error::CouldNotLoadSchema(err) => err.to_string(),
        }
    }
}

/// Environment variable is not set
#[derive(Debug, Eq, PartialEq)]
pub struct ErrorEnvVarNotSet {
    pub env_var: String,
}

impl ToString for ErrorEnvVarNotSet {
    fn to_string(&self) -> String {
        format!("Environment variable not found: {}", self.env_var)
    }
}

/// Environment variable is not set
#[derive(Debug, Eq, PartialEq)]
pub struct ErrorFlagNotSet {
    pub flag: String,
}

impl ToString for ErrorFlagNotSet {
    fn to_string(&self) -> String {
        format!("Flag not found: {flag}", flag = self.flag)
    }
}

/// Could not load schema
#[derive(Debug, Eq, PartialEq)]
pub struct ErrorCouldNotLoadSchema {
    pub uri: String,
    pub err: uri::Err,
}

impl ToString for ErrorCouldNotLoadSchema {
    fn to_string(&self) -> String {
        format!(
            "Could no load schema from {}: {}", 
            self.uri, self.err.to_string(),
        ) 
    }
}
