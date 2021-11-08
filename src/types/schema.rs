//! Schema
//! Collections of related types


use serde;
use serde::{Deserialize, Serialize};

use crate::types::typ::Type;


/// The Schema Schema
#[derive(Deserialize, Serialize)]
pub struct Schema {
    pub name: String,
    pub types: Vec<Type>,
}
