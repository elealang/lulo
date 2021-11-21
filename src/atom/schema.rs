//! Schema
//! Collections of related types

use serde;
use serde::{Deserialize, Serialize};

use crate::atom::typ::Type;

/// Schemas
#[derive(Deserialize, Serialize)]
pub struct Schema {
    pub id: String,
    pub namespace: String,
    pub types: Vec<Type>,
}
