//! Types

use convert_case::{Case, Casing};
use serde;
use serde::{Deserialize, Serialize};
use std::string::String;


/// Type
#[derive(Clone, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(tag = "kind", rename_all(deserialize = "lowercase"))]
pub enum Type {
    Set(SetType),
    List(ListType),
    Text(TextType),
    Integer(IntegerType),
    Float(FloatType),
    Symbol(SymbolType),
    Timestamp(TimestampType),
    Date(DateType),
}

impl Type {
    pub fn title(&self) -> String {
        return self.id().to_string().to_case(Case::UpperCamel);
    }
}

impl TypeSum for Type {
    fn kind(&self) -> String {
        match &*self {
            Type::Set(t) => t.kind(),
            Type::List(t) => t.kind(),
            Type::Text(t) => t.kind(),
            Type::Integer(t) => t.kind(),
            Type::Float(t) => t.kind(),
            Type::Symbol(t) => t.kind(),
            Type::Timestamp(t) => t.kind(),
            Type::Date(t) => t.kind(),
        }
    }

    fn id(&self) -> &TypeId {
        match &*self {
            Type::Set(t) => t.id(),
            Type::List(t) => t.id(),
            Type::Text(t) => t.id(),
            Type::Integer(t) => t.id(),
            Type::Float(t) => t.id(),
            Type::Symbol(t) => t.id(),
            Type::Timestamp(t) => t.id(),
            Type::Date(t) => t.id(),
        }
    }
}

/// TypeSum
pub trait TypeSum {
    fn kind(&self) -> String;
    fn id(&self) -> &TypeId;
}

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

/// Set Type
#[derive(Clone, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
pub struct SetType {
    /// The [globally] unique identifer of the set type
    pub id: TypeId,

    /// A formal title for the type
    pub label: String,

    /// A detailed description of the type
    pub description: String,

    /// Required types
    #[serde(default)]
    pub required: Vec<TypeId>,

    /// The size of the set
    #[serde(default)]
    pub size: SetSize,

    /// Member type ids
    pub type_ids: Vec<TypeId>,
}

impl TypeSum for SetType {
    fn kind(&self) -> String {
        return String::from("set");
    }

    fn id(&self) -> &TypeId {
        return &self.id;
    }
}

/// SetSize
#[derive(Clone, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
pub enum SetSize {
    All,
    N(i64),
}

impl Default for SetSize {
    fn default() -> Self {
        SetSize::All
    }
}

impl ToString for SetSize {
    fn to_string(&self) -> String {
        match *&self {
            SetSize::All  => String::from("All"),
            SetSize::N(n) => n.to_string(),
        }     
    }
}

/// List Type
#[derive(Clone, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
pub struct ListType {
    pub id: TypeId,
    pub label: String,
    pub description: String,
    pub type_ids: Vec<TypeId>,
}

impl TypeSum for ListType {
    fn kind(&self) -> String {
        return String::from("list");
    }

    fn id(&self) -> &TypeId {
        return &self.id;
    }
}

/// Text Type
#[derive(Clone, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
pub struct TextType {
    pub id: TypeId,
    pub label: String,

    #[serde(default)]
    pub description: String,
}

impl TypeSum for TextType {
    fn kind(&self) -> String {
        return String::from("text");
    }

    fn id(&self) -> &TypeId {
        return &self.id;
    }
}

/// Integer Type
#[derive(Clone, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
pub struct IntegerType {
    pub id: TypeId,
    pub label: String,

    #[serde(default)]
    pub description: String,
}

impl TypeSum for IntegerType {
    fn kind(&self) -> String {
        return String::from("integer");
    }

    fn id(&self) -> &TypeId {
        return &self.id;
    }
}

/// Float Type
#[derive(Clone, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
pub struct FloatType {
    pub id: TypeId,
    pub label: String,
    pub description: String,
}

impl TypeSum for FloatType {
    fn kind(&self) -> String {
        return String::from("float");
    }

    fn id(&self) -> &TypeId {
        return &self.id;
    }
}

/// Symbol Type
#[derive(Clone, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
pub struct SymbolType {
    pub id: TypeId,
    pub label: String,

    #[serde(default)]
    pub description: String,
}

impl TypeSum for SymbolType {
    fn kind(&self) -> String {
        return String::from("symbol");
    }

    fn id(&self) -> &TypeId {
        return &self.id;
    }
}

/// Timestamp Type
#[derive(Clone, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
pub struct TimestampType {
    pub id: TypeId,
    pub label: String,
    pub description: String,
}

impl TypeSum for TimestampType {
    fn kind(&self) -> String {
        return String::from("timestamp");
    }

    fn id(&self) -> &TypeId {
        return &self.id;
    }
}

/// Date Type
#[derive(Clone, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
pub struct DateType {
    pub id: TypeId,
    pub label: String,
    pub description: String,
}

impl TypeSum for DateType {
    fn kind(&self) -> String {
        return String::from("date");
    }

    fn id(&self) -> &TypeId {
        return &self.id;
    }
}
