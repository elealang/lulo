//! Change
//! How values can be mutated

use serde::{Deserialize, Serialize};

/// Change
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Change {
    pub required: bool,
}
