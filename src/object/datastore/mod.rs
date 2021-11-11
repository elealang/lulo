//! Datastores
//! Abstraction for storage resources that can host Lulo databases

pub mod error;
pub mod fsdev;

use crate::types::database::Database;
use crate::types::uri::URI;

/// DataStore trait
pub trait DataStore<E> {
    /// Instantiate the datastore at the given URI
    fn at(uri: &URI) -> Result<Self, E>
    where
        Self: Sized;

    /// Writes a database's metadata to storage, overwriting any existing data.
    /// Registers (that contain client data) are handled separately.
    fn sync_db(&self, db: &Database) -> Result<(), E>
    where
        Self: Sized;

    /// Registers (that contain client data) are handled separately.
    fn database(&self) -> Result<Database, E>;
}
