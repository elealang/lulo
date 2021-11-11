//! Schema
//! Collections of related types

use serde;
use serde::{Deserialize, Serialize};

use crate::types::typ::Type;

/// The Schema Schema
#[derive(Deserialize, Serialize)]
pub struct Schema {
    pub id: String,
    pub namespace: String,
    pub types: Vec<Type>,
}
