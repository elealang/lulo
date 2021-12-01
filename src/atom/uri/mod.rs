//! URI
//! Uniform Resource Identifiers used by Lulo to provide generic interfaces

pub mod error;
pub mod fetch;
pub mod parse;

use serde;
use serde::{Deserialize, Deserializer, Serialize};
use serde::de::Error;
use strum_macros::{Display, EnumString};


// =================================================================================================
// PUBLIC
// =================================================================================================

/// URIs
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct URI {
    pub scheme: Scheme,
    pub path: Path,
}

impl URI {
    /// Create a URI from a string
    pub fn from_string(uri_string: &str) -> Result<URI, error::Error> {
        return parse::uri_from_string(uri_string);
    }
}

impl ToString for URI {
    fn to_string(&self) -> String {
        return format!("{}://{}", self.scheme.to_string(), self.path.to_string());
    }
}


impl<'de> Deserialize<'de> for URI {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: &str = Deserialize::deserialize(deserializer)?;
        let uri_or_err = match URI::from_string(&s) {
            Ok(s)    => return Ok(s),
            Err(err) => return Err(D::Error::custom(err)),
        }
    }
}



impl URI {
    pub fn to_file_path(&self) -> std::path::PathBuf {
        let path_str = self.path.to_string();
        return std::path::PathBuf::from(&path_str);
    }
}

/// Path
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Path {
    pub segments: Vec<PathSegment>,
}

impl ToString for Path {
    fn to_string(&self) -> String {
        return self
            .segments
            .iter()
            .fold(String::from(""), |path_str, seg| {
                format!("{}/{}", path_str, seg.0)
            });
    }
}

/// Path Segment
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct PathSegment(String);

/// Scheme
#[derive(Clone, Debug, Deserialize, Display, EnumString, Eq, PartialEq, Serialize)]
#[strum(serialize_all = "snake_case")]
pub enum Scheme {
    File,
    Http,
}
