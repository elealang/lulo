//! URI
//! Uniform Resource Identifiers used by Lulo to provide generic interfaces

pub mod error;
pub mod fetch;
pub mod parse;

use serde;
use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumString};


// =================================================================================================
// PUBLIC
// =================================================================================================

/// URIs
#[derive(Clone, Debug, Deserialize, Serialize)]
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

impl URI {
    pub fn to_file_path(&self) -> std::path::PathBuf {
        let path_str = self.path.to_string();
        return std::path::PathBuf::from(&path_str);
    }
}

/// Path
#[derive(Clone, Debug, Deserialize, Serialize)]
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
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PathSegment(String);

/// Scheme
#[derive(Clone, Debug, Deserialize, Display, PartialEq, EnumString, Serialize)]
#[strum(serialize_all = "snake_case")]
pub enum Scheme {
    File,
    Http,
}
