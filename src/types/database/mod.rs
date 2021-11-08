//! Database
//! Databases


pub mod error;


use console::style;
use serde;
use serde::{Deserialize, Serialize};

use crate::types::schema::Schema;
use crate::types::uri::URI;


/// Database
#[derive(Deserialize, Serialize)]
pub struct Database {
    uri: URI,
    name: String,
    schemas: Vec<Schema>,
}


impl Database {

    /// Create a new database
    pub fn new(uri: URI) -> Database {
        return Database {
            uri: uri,
            name: String::from(""),
            schemas: Vec::new(),
        } 
    }

    /// Set the database name
    pub fn set_name(&mut self, name: &str) {
        self.name = name.to_string();
    }

    /// Get the database name
    pub fn get_name(&self) -> String {
        return self.name.clone();
    }


    // Display Methods

    /// Print CLI representation
    pub fn print_cli(&self) {

        // name
        println!("{} {}", format!("{:<8}", style("name").bold()), self.name);

        // uri
        println!("{} {}", format!("{:<8}", style("uri").bold()), self.uri.to_string());

        // schemas
        let mut schemas_str = self.schemas.iter().fold(
            String::from(""), 
            |str_acc, schema| format!("{}, {}", str_acc, schema.name),
        );
        if self.schemas.len() == 0 {
            schemas_str = String::from("None");
        }
        println!("{} {}", format!("{:<8}", style("schemas").bold()), schemas_str);
    }
}

