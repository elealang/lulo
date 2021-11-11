//! Lib Database errors
//! database errors

pub enum Error {
    Sync(ErrorSync),
    Read(ErrorRead),
}

impl ToString for Error {
    fn to_string(&self) -> String {
        match &*self {
            Error::Sync(err) => err.to_string(),
            Error::Read(err) => err.to_string(),
        }
    }
}

pub struct ErrorSync {
    pub err: String,
}

impl ToString for ErrorSync {
    fn to_string(&self) -> String {
        return self.err.clone();
    }
}

pub struct ErrorRead {
    pub err: String,
}

impl ToString for ErrorRead {
    fn to_string(&self) -> String {
        return self.err.clone();
    }
}
