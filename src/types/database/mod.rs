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
    pub uri: URI,
    pub id: String,
    pub schemas: Vec<Schema>,
}

impl Database {
    /// Create a new database
    pub fn new(uri: URI) -> Database {
        return Database {
            uri: uri,
            id: String::from(""),
            schemas: Vec::new(),
        };
    }

    /// Set the database ID
    pub fn set_id(&mut self, id: &str) {
        self.id = id.to_string();
    }

    /// Get the database name
    pub fn get_id(&self) -> String {
        return self.id.clone();
    }

    // Display Methods

    /// Print CLI representation
    pub fn print_cli(&self) {
        // Name
        println!("{} {}", format!("{:<8}", style("name").bold()), self.id);

        // URI
        println!(
            "{} {}",
            format!("{:<8}", style("uri").bold()),
            self.uri.to_string()
        );

        // Schemas
        let mut schemas_str = self
            .schemas
            .iter()
            .fold(String::from(""), |str_acc, schema| {
                format!("{}, {}", str_acc, schema.id)
            });
        if self.schemas.len() == 0 {
            schemas_str = String::from("None");
        }
        println!(
            "{} {}",
            format!("{:<8}", style("schemas").bold()),
            schemas_str
        );
    }
}
