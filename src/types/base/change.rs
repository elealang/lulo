//! Changes

use crate::types::base::value;


/// Change
pub enum Change {
    Add(Add),
} 

pub struct Add {
    value: value::Value,
}

