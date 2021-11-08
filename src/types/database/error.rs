//! Database: Error


use crate::types::uri;


pub enum Error {
    URI(uri::error::Error),
}


