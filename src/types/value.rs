//! Value
//! The available data types.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::typ::TypeId;

/// Value
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all(deserialize = "lowercase"))]
pub enum Value {
    Set(SetValue),
    List(ListValue),
    Text(TextValue),
    Integer(IntegerValue),
    Float(FloatValue),
    Symbol(SymbolValue),
    Timestamp(TimestampValue),
    Date(DateValue),
}

/// Set Value
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct SetValue {
    #[serde(flatten)]
    pub values: HashMap<TypeId, Value>,
}

/// List value
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ListValue {
    pub values: Vec<Value>,
}

/// Text Value
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(transparent)]
pub struct TextValue {
    pub value: String,
}

/// Integer Value
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(transparent)]
pub struct IntegerValue {
    pub value: i64,
}

/// Float Value
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(transparent)]
pub struct FloatValue {
    pub value: i64,
}

/// Symbol Value
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(transparent)]
pub struct SymbolValue {
    pub value: String,
}

/// Timestamp Value
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(transparent)]
pub struct TimestampValue {
    pub value: i64,
}

/// Date Value
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(transparent)]
pub struct DateValue {
    pub value: i64,
}
