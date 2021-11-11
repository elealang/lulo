//! URI: Error

#[derive(Debug, Eq, PartialEq)]
pub enum Error {
    Parse(ErrorParse),
}

impl ToString for Error {
    fn to_string(&self) -> String {
        match &*self {
            Error::Parse(err) => err.to_string(),
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
