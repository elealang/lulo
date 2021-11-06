
#[derive(Debug, Eq, PartialEq)]
/// URI Error
pub enum Err {
    SchemeNotSupported(ErrSchemeNotSupported),
    InvalidURI(ErrInvalidURI),
    CannotDesSchema(ErrCannotDesSchema),
    CannotDesValue(ErrCannotDesValue),
    FileSystem(ErrFileSystem),
}

impl ToString for Err {
    fn to_string(&self) -> String {
        match &*self {
            Err::SchemeNotSupported(err) => err.to_string(),
            Err::InvalidURI(err) => err.to_string(),
            Err::CannotDesSchema(err) => err.to_string(),
            Err::CannotDesValue(err) => err.to_string(),
            Err::FileSystem(err) => err.to_string(),
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
/// URI Error when accessing the local filesystem.
pub enum ErrFileSystem {
    CannotOpenFile(ErrCannotOpenFile),
    CannotReadFile(ErrCannotReadFile),
}

impl ToString for ErrFileSystem {
    fn to_string(&self) -> String {
        match &*self {
            ErrFileSystem::CannotOpenFile(err) => err.to_string(),
            ErrFileSystem::CannotReadFile(err) => err.to_string(),
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
/// Error when trying to open a file from the local filesystem.
pub struct ErrCannotOpenFile {
    pub filepath: String,
    pub io_err: String,
}

impl ToString for ErrCannotOpenFile {
    fn to_string(&self) -> String {
        format!("Cannot open file [{}]: {}", self.filepath, self.io_err)
    }
}

#[derive(Debug, Eq, PartialEq)]
/// Error when trying to read a file from the local filesystem.
pub struct ErrCannotReadFile {
    pub filepath: String,
    pub io_err: String,
}

impl ToString for ErrCannotReadFile {
    fn to_string(&self) -> String {
        format!("Cannot read file [{}]: {}", self.filepath, self.io_err)
    }
}

#[derive(Debug, Eq, PartialEq)]
/// The URI Schema is not supported
pub struct ErrSchemeNotSupported {
    pub scheme: String,
}

impl ToString for ErrSchemeNotSupported {
    fn to_string(&self) -> String {
        format!("URI Scheme [{}] not supported", self.scheme)
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct ErrCannotDesValue {
    pub value_uri: String,
    pub des_err: String,
}

impl ToString for ErrCannotDesValue {
    fn to_string(&self) -> String {
        format!("Could not deserialze value at [{}]: {}", self.value_uri, self.des_err)
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct ErrCannotDesSchema {
    pub schema_uri: String,
    pub des_err: String,
}

impl ToString for ErrCannotDesSchema {
    fn to_string(&self) -> String {
        format!("Could not deserialze schema at [{}]: {}", self.schema_uri, self.des_err)
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct ErrInvalidURI {
    pub uri: String,
}

impl ToString for ErrInvalidURI {
    fn to_string(&self) -> String {
        format!("Invalid URI [{}]", self.uri)
    }
}

