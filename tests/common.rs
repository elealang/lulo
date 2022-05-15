//! Tests: Common
//! Common integration test utility functions


use std::env;
use std::path::{Path};

use lulo::atom::uri::URI;


pub fn data_uri(resource_path: &str) -> URI {
    let cwd = env::current_dir().unwrap();    
    let uri_str = format!("file://{}/data{}", cwd.display(), resource_path);
    return URI::from_string(&uri_str).unwrap();
}
