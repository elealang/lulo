//! Values

use serde::{Deserialize, Serialize};


/// Value
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all(deserialize = "lowercase"))]
pub enum Value {
    Comb(Combination),
    Perm(Permutation),
    Text(Text),
    Intg(Integer),
    Real(Real),
    Symb(Symbol),
}

/// Combination
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Combination {
    #[serde(flatten)]
    pub values: Vec<Value>,
}

/// Permutation
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Permutation {
    pub values: Vec<Value>,
}

/// Text
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum Text {
    Literal(TextLiteral),
    Empty(TextEmpty),
    All(TextAll),
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct TextLiteral {
    pub value: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct TextEmpty {}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct TextAll {}

/// Integer Value
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(transparent)]
pub struct Integer {
    pub value: i64,
}

/// Real Number
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(transparent)]
pub struct Real {
    pub value: i64,
}

/// Symbol Value
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(transparent)]
pub struct Symbol {
    pub value: String,
}

