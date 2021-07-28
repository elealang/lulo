
use http::Uri;

use crate::value::Value;

// FUNCTIONS -------------------------------------------------------------------

#[allow(dead_code)]
pub fn value(uri_str: &str) -> Result<Value, ErrURI> {

    let uri = uri_str.parse::<Uri>().unwrap();

    println!("uri: {:?}", uri);

    return Err(ErrURI::FileSystem(ErrURIFileSystem{}));
}

// ERRORS ----------------------------------------------------------------------

#[derive(Debug, Eq, PartialEq)]
pub enum ErrURI {
    #[allow(dead_code)]
    FileSystem(ErrURIFileSystem),
}

#[derive(Debug, Eq, PartialEq)]
pub struct ErrURIFileSystem {
}

