//! Schema
//! Collections of related types


use serde;
use serde::{Deserialize, Serialize};

use crate::types::atom::typ::Type;


/// The Schema Schema
#[derive(Deserialize, Serialize)]
pub struct Schema {
    pub types: Vec<Type>,
}
