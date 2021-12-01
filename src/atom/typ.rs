//! Type
//! Types are a composition of values and changes.

use serde::{Deserialize, Serialize};

use super::change::Change;
use super::value::Value;

/// TypeId
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, PartialOrd, Ord, Serialize)]
pub struct TypeId(String);

impl TypeId {
    pub fn from_string(id: &str) -> Self {
        TypeId(id.to_string())
    }
}

impl ToString for TypeId {
    fn to_string(&self) -> String {
        self.0.clone()
    }
}

/// Type
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Type {
    pub id: TypeId,
    pub value: Value,
    pub changes: Vec<Change>,
}

/// Kind
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all(deserialize = "lowercase", serialize = "lowercase"))]
pub enum Kind {
    Set,
    List,
    Text,
    Integer,
    Float,
    Symbol,
    Timestamp,
    Date,
}
