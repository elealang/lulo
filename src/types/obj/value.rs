use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::types::base;
use crate::types::base::typ::TypeId;


/// Value
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
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

impl Value {
    pub fn from_base(value: base::value::Value, origin_uri: Option<String>) -> Self {
        match value {
            base::value::Value::Set(t) => Value::Set(SetValue::from_base(t, origin_uri)),
            base::value::Value::List(t) => Value::List(ListValue::from_base(t, origin_uri)),
            base::value::Value::Text(t) => Value::Text(TextValue::from_base(t, origin_uri)),
            base::value::Value::Integer(t) => Value::Integer(IntegerValue::from_base(t, origin_uri)),
            base::value::Value::Float(t) => Value::Float(FloatValue::from_base(t, origin_uri)),
            base::value::Value::Symbol(t) => Value::Symbol(SymbolValue::from_base(t, origin_uri)),
            base::value::Value::Timestamp(t) => Value::Timestamp(TimestampValue::from_base(t, origin_uri)),
            base::value::Value::Date(t) => Value::Date(DateValue::from_base(t, origin_uri)),
        }
    }
}

impl ValueObject for Value {
    fn origin(&self) -> Option<String> {
        match &*self {
            Value::Set(v)       => v.origin(),
            Value::List(v)      => v.origin(),
            Value::Text(v)      => v.origin(),
            Value::Integer(v)   => v.origin(),
            Value::Float(v)     => v.origin(),
            Value::Symbol(v)    => v.origin(),
            Value::Timestamp(v) => v.origin(),
            Value::Date(v)      => v.origin(),
        }
    }
}


/// Value Object
pub trait ValueObject {
    fn origin(&self) -> Option<String>;
}


/// Set Value
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct SetValue {
    pub values: HashMap<TypeId, Value>,
    origin_uri: Option<String>,
}

impl SetValue {
    pub fn from_base(value: base::value::SetValue, origin_uri: Option<String>) -> Self {
        SetValue {
            values: value
                .values
                .into_iter()
                .map(|(id, v)| (id, Value::from_base(v, None)))
                .collect(),
            origin_uri: origin_uri,
        }
    }

    pub fn from_vec(values: Vec<(TypeId, Value)>, origin_uri: Option<String>) -> Self {
        SetValue {
            values: values.into_iter().collect(),
            origin_uri: origin_uri,
        }
    }
}

impl ValueObject for SetValue {
    fn origin(&self) -> Option<String> {
        return self.origin_uri.clone();
    }
}

/// List Value
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ListValue {
    pub values: Vec<Value>,
    origin_uri: Option<String>,
}

impl ListValue {
    pub fn from_base(value: base::value::ListValue, origin_uri: Option<String>) -> Self {
        ListValue {
            values: value
                .values
                .into_iter()
                .map(|v| Value::from_base(v, None))
                .collect(),
            origin_uri: origin_uri,
        }
    }
}

impl ValueObject for ListValue {
    fn origin(&self) -> Option<String> {
        return self.origin_uri.clone();
    }
}

/// Text Value
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct TextValue {
    value: String,
    origin_uri: Option<String>,
}

impl TextValue {
    pub fn from_base(value: base::value::TextValue, origin_uri: Option<String>) -> Self {
        TextValue { 
            value: value.value,
            origin_uri: origin_uri,
        }
    }

    pub fn from_string(s: &str) -> TextValue {
        TextValue {
            value: s.to_string(),
            origin_uri: None,
        }
    }
}

impl ValueObject for TextValue {
    fn origin(&self) -> Option<String> {
        return self.origin_uri.clone();
    }
}

/// Integer Value
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct IntegerValue {
    value: i64,
    origin_uri: Option<String>,
}

impl IntegerValue {
    pub fn from_base(value: base::value::IntegerValue, origin_uri: Option<String>) -> Self {
        IntegerValue { 
            value: value.value,
            origin_uri: origin_uri,
        }
    }

    pub fn from_integer(i: i64) -> IntegerValue {
        IntegerValue { 
            value: i,
            origin_uri: None,
        }
    }
}

impl ValueObject for IntegerValue {
    fn origin(&self) -> Option<String> {
        return self.origin_uri.clone();
    }
}

/// Float Value
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct FloatValue {
    value: i64,
    origin_uri: Option<String>,
}

impl FloatValue {
    pub fn from_base(value: base::value::FloatValue, origin_uri: Option<String>) -> Self {
        FloatValue { 
            value: value.value,
            origin_uri: origin_uri,
        }
    }
}

impl ValueObject for FloatValue {
    fn origin(&self) -> Option<String> {
        return self.origin_uri.clone();
    }
}

/// Symbol Value
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct SymbolValue {
    value: String,
    origin_uri: Option<String>,
}

impl SymbolValue {
    pub fn from_base(value: base::value::SymbolValue, origin_uri: Option<String>) -> Self {
        SymbolValue { 
            value: value.value,
            origin_uri: origin_uri,
        }
    }

    pub fn from_string(s: &str) -> SymbolValue {
        SymbolValue {
            value: s.to_string(),
            origin_uri: None,
        }
    }
}

impl ValueObject for SymbolValue {
    fn origin(&self) -> Option<String> {
        return self.origin_uri.clone();
    }
}

/// Timestamp Value
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct TimestampValue {
    value: i64,
    origin_uri: Option<String>,
}

impl TimestampValue {
    pub fn from_base(value: base::value::TimestampValue, origin_uri: Option<String>) -> Self {
        TimestampValue { 
            value: value.value,
            origin_uri: origin_uri,
        }
    }

    pub fn from_string(_: &str) -> TimestampValue {
        TimestampValue { 
            value: 1000,
            origin_uri: None,
        }
    }
}

impl ValueObject for TimestampValue {
    fn origin(&self) -> Option<String> {
        return self.origin_uri.clone();
    }
}

/// Date Value
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct DateValue {
    value: i64,
    origin_uri: Option<String>,
}

impl DateValue {
    pub fn from_base(value: base::value::DateValue, origin_uri: Option<String>) -> Self {
        DateValue { 
            value: value.value,
            origin_uri: origin_uri,
        }
    }

    pub fn from_string(_: &str) -> DateValue {
        DateValue { 
            value: 1000,
            origin_uri: None,
        }
    }
}

impl ValueObject for DateValue {
    fn origin(&self) -> Option<String> {
        return self.origin_uri.clone();
    }
}

