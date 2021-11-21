use serde;
use serde::{Deserialize, Serialize};

use crate::types::base::typ::Type;

/// The Schema Schema
#[derive(Deserialize, Serialize)]
pub struct Schema {
    pub types: Vec<Type>,
}
