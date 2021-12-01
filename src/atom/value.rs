//! Value
//! The available data types.

use serde::{Deserialize, Serialize};

use super::typ::TypeId;
use super::uri::URI;

/// Value
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all(deserialize = "lowercase", serialize = "lowercase"))]
pub enum Value {
    Register(RegisterValue),
    Set(SetValue),
    List(ListValue),
    Text(TextValue),
    Integer(IntegerValue),
    Float(FloatValue),
    Symbol(SymbolValue),
    Timestamp(TimestampValue),
    Date(DateValue),
}


/// Register Value
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct RegisterValue {
    pub uri: URI,
}


/// Set Value
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct SetValue {
    pub values: Vec<SetValueMember>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct SetValueMember {
    pub type_id: TypeId,
    pub value: Value,
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

impl TextValue {

    pub fn from_string(s: &str) -> Self {
        return TextValue {
            value: s.to_string(),
        } 
    }
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

