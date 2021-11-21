//! Database: Error

use crate::atom::uri;

pub enum Error {
    URI(uri::error::Error),
}
