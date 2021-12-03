//! Change
//! How values can be mutated


use serde::{Deserialize, Serialize};


use super::typ::TypeId;


/// Change
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Change {
    pub required: bool,
    pub operation: Operation,
}


/// Operation
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Operation {
    Add(AddOperation),
}


#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct AddOperation {
    pub value_of_type: TypeId,
}
