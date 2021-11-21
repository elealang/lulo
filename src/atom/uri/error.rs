//! URI: Error

#[derive(Debug, Eq, PartialEq)]
pub enum Error {
    Parse(ErrorParse),
    CannotOpenFile(ErrorCannotOpenFile),
    CannotReadFile(ErrorCannotReadFile),
    CannotDeserializeValue(ErrorCannotDeserializeValue),
    CannotDeserializeSchema(ErrorCannotDeserializeSchema),
}

impl ToString for Error {
    fn to_string(&self) -> String {
        match &*self {
            Error::Parse(err) => err.to_string(),
            Error::CannotOpenFile(err) => err.to_string(),
            Error::CannotReadFile(err) => err.to_string(),
            Error::CannotDeserializeValue(err) => err.to_string(),
            Error::CannotDeserializeSchema(err) => err.to_string(),
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct ErrorParse {
    pub parse_error: String,
}

impl ToString for ErrorParse {
    fn to_string(&self) -> String {
        return self.parse_error.to_string();
    }
}



//#[derive(Debug, Eq, PartialEq)]
///// URI Error when accessing the local filesystem.
//pub enum ErrFileSystem {
    //CannotOpenFile(ErrCannotOpenFile),
    //CannotReadFile(ErrCannotReadFile),
//}

//impl ToString for ErrFileSystem {
    //fn to_string(&self) -> String {
        //match &*self {
            //ErrFileSystem::CannotOpenFile(err) => err.to_string(),
            //ErrFileSystem::CannotReadFile(err) => err.to_string(),
        //}
    //}
//}

#[derive(Debug, Eq, PartialEq)]
/// Error when trying to open a file from the local filesystem.
pub struct ErrorCannotOpenFile {
    pub filepath: String,
    pub io_err: String,
}


impl ToString for ErrorCannotOpenFile {
    fn to_string(&self) -> String {
        format!("Cannot open file [{}]: {}", self.filepath, self.io_err)
    }
}


#[derive(Debug, Eq, PartialEq)]
/// Error when trying to read a file from the local filesystem.
pub struct ErrorCannotReadFile {
    pub filepath: String,
    pub io_err: String,
}

impl ToString for ErrorCannotReadFile {
    fn to_string(&self) -> String {
        format!("Cannot read file [{}]: {}", self.filepath, self.io_err)
    }
}

//#[derive(Debug, Eq, PartialEq)]
///// The URI Schema is not supported
//pub struct ErrSchemeNotSupported {
    //pub scheme: String,
//}

//impl ToString for ErrSchemeNotSupported {
    //fn to_string(&self) -> String {
        //format!("URI Scheme [{}] not supported", self.scheme)
    //}
//}

#[derive(Debug, Eq, PartialEq)]
pub struct ErrorCannotDeserializeValue {
    pub value_uri: String,
    pub des_err: String,
}

impl ToString for ErrorCannotDeserializeValue {
    fn to_string(&self) -> String {
        format!("Could not deserialze value at [{}]: {}", self.value_uri, self.des_err)
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct ErrorCannotDeserializeSchema {
    pub schema_uri: String,
    pub des_err: String,
}

impl ToString for ErrorCannotDeserializeSchema {
    fn to_string(&self) -> String {
        format!("Could not deserialze schema at [{}]: {}", self.schema_uri, self.des_err)
    }
}

//#[derive(Debug, Eq, PartialEq)]
//pub struct ErrInvalidURI {
    //pub uri: String,
//}

//impl ToString for ErrInvalidURI {
    //fn to_string(&self) -> String {
        //format!("Invalid URI [{}]", self.uri)
    //}
//}
