//! Types

use crate::types::base;
use crate::types::base::typ::TypeId;

/// Type Object
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
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
    pub fn from_base(typ: base::typ::Type) -> Self {
        match typ {
            base::typ::Type::Set(t) => Type::Set(SetType::from_base(t)),
            base::typ::Type::List(t) => Type::List(ListType::from_base(t)),
            base::typ::Type::Text(t) => Type::Text(TextType::from_base(t)),
            base::typ::Type::Integer(t) => Type::Integer(IntegerType::from_base(t)),
            base::typ::Type::Float(t) => Type::Float(FloatType::from_base(t)),
            base::typ::Type::Symbol(t) => Type::Symbol(SymbolType::from_base(t)),
            base::typ::Type::Timestamp(t) => Type::Timestamp(TimestampType::from_base(t)),
            base::typ::Type::Date(t) => Type::Date(DateType::from_base(t)),
        }
    }
}

impl base::typ::TypeSum for Type {
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

    fn id(&self) -> &base::typ::TypeId {
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

/// Set Type Object
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct SetType {
    /// The [globally] unique identifer of the set type
    id: base::typ::TypeId,

    /// A formal title for the type
    label: String,

    /// A detailed description of the type
    description: String,

    /// The size of the set
    pub size: base::typ::SetSize,

    /// The types...
    pub type_ids: Vec<base::typ::TypeId>,

    /// Required types
    required: Vec<base::typ::TypeId>,
}

impl SetType {
    fn from_base(typ: base::typ::SetType) -> Self {
        SetType {
            id: typ.id,
            label: typ.label,
            description: typ.description,
            size: typ.size,
            type_ids: typ.type_ids, //.iter().map(|&t| t.clone()).collect(),
            required: typ.required,
        }
    }

    pub fn new(id: TypeId, type_ids: Vec<TypeId>) -> Self {
        SetType {
            id: id,
            label: String::from(""),
            description: String::from(""),
            size: base::typ::SetSize::All,
            type_ids: type_ids,
            required: Vec::new(),
        }
    }
}

impl base::typ::TypeSum for SetType {
    fn kind(&self) -> String {
        return String::from("set");
    }

    fn id(&self) -> &base::typ::TypeId {
        return &self.id;
    }
}

/// SetSize
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum SetSize {
    All,
    N(i64),
}

impl Default for SetSize {
    fn default() -> Self {
        SetSize::All
    }
}

/// List Type
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct ListType {
    id: base::typ::TypeId,
    label: String,
    description: String,
    pub type_ids: Vec<base::typ::TypeId>,
}

impl ListType {
    fn from_base(typ: base::typ::ListType) -> Self {
        return ListType {
            id: typ.id,
            label: typ.label,
            description: typ.description,
            type_ids: typ.type_ids, //.iter().map(|&t| t.clone()).collect(),
        };
    }
}

impl base::typ::TypeSum for ListType {
    fn kind(&self) -> String {
        return String::from("list");
    }

    fn id(&self) -> &base::typ::TypeId {
        return &self.id;
    }
}

/// Text Type
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct TextType {
    id: base::typ::TypeId,
    label: String,
    description: String,
}

impl TextType {
    fn from_base(typ: base::typ::TextType) -> Self {
        return TextType {
            id: typ.id,
            label: typ.label,
            description: typ.description,
        };
    }

    pub fn new(id: base::typ::TypeId) -> Self {
        TextType {
            id: id,
            label: String::from(""),
            description: String::from(""),
        }
    }
}

impl base::typ::TypeSum for TextType {
    fn kind(&self) -> String {
        return String::from("text");
    }

    fn id(&self) -> &base::typ::TypeId {
        return &self.id;
    }
}

/// Integer Type
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct IntegerType {
    id: base::typ::TypeId,
    label: String,
    description: String,
}

impl IntegerType {
    fn from_base(typ: base::typ::IntegerType) -> Self {
        return IntegerType {
            id: typ.id,
            label: typ.label,
            description: typ.description,
        };
    }

    pub fn new(id: base::typ::TypeId) -> Self {
        IntegerType {
            id: id,
            label: String::from(""),
            description: String::from(""),
        }
    }
}

impl base::typ::TypeSum for IntegerType {
    fn kind(&self) -> String {
        return String::from("integer");
    }

    fn id(&self) -> &base::typ::TypeId {
        return &self.id;
    }
}

/// Float Type
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct FloatType {
    id: base::typ::TypeId,
    label: String,
    description: String,
}

impl FloatType {
    fn from_base(typ: base::typ::FloatType) -> Self {
        return FloatType {
            id: typ.id,
            label: typ.label,
            description: typ.description,
        };
    }
}

impl base::typ::TypeSum for FloatType {
    fn kind(&self) -> String {
        return String::from("float");
    }

    fn id(&self) -> &base::typ::TypeId {
        return &self.id;
    }
}

/// Symbol Type
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct SymbolType {
    id: base::typ::TypeId,
    label: String,
    description: String,
}

impl SymbolType {
    fn from_base(typ: base::typ::SymbolType) -> Self {
        return SymbolType {
            id: typ.id,
            label: typ.label,
            description: typ.description,
        };
    }

    pub fn new(id: base::typ::TypeId) -> Self {
        SymbolType {
            id: id,
            label: String::from(""),
            description: String::from(""),
        }
    }
}

impl base::typ::TypeSum for SymbolType {
    fn kind(&self) -> String {
        return String::from("symbol");
    }

    fn id(&self) -> &base::typ::TypeId {
        return &self.id;
    }
}

/// Timestamp Type
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct TimestampType {
    id: base::typ::TypeId,
    label: String,
    description: String,
}

impl TimestampType {
    fn from_base(typ: base::typ::TimestampType) -> Self {
        return TimestampType {
            id: typ.id,
            label: typ.label,
            description: typ.description,
        };
    }
}

impl base::typ::TypeSum for TimestampType {
    fn kind(&self) -> String {
        return String::from("timestamp");
    }

    fn id(&self) -> &base::typ::TypeId {
        return &self.id;
    }
}

/// Date Type
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct DateType {
    id: base::typ::TypeId,
    label: String,
    description: String,
}

impl DateType {
    fn from_base(typ: base::typ::DateType) -> Self {
        return DateType {
            id: typ.id,
            label: typ.label,
            description: typ.description,
        };
    }
}

impl base::typ::TypeSum for DateType {
    fn kind(&self) -> String {
        return String::from("date");
    }

    fn id(&self) -> &base::typ::TypeId {
        return &self.id;
    }
}

//types.iter().map(|&typ| (typ.id().to_string(), typ.clone())).collect(),
