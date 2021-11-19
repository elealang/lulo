//! Tests: Common
//! Common integration test utility functions


use std::env;
use std::path::{Path};

use lulo::uri::URI;


pub fn data_uri(resource_path: &str) -> URI {
    let cwd_path_buf = env::current_dir().unwrap();    
    let cwd_path = cwd_path_buf.as_path();    
    let path = cwd_path.join(Path::new(resource_path));
    let uri_str = format!("file://{}", path.as_path().display());
    return URI::from_string(&uri_str).unwrap();
}
