//! Types

use serde::Serialize;
use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::string::String;

/// All of the possible Lulo types.
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
#[allow(dead_code)]
#[derive(Serialize)]
#[serde(tag = "kind", content = "type")]
pub enum Type {
    Set(Set),
    List(List),
    Text(Text),
    Integer(Integer),
    Float(Float),
    Symbol(Symbol),
    Timestamp(Timestamp),
    Date(Date),
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
        }
    }

    fn id(&self) -> &str {
        match &*self {
            Type::Set(t) => t.id(),
            Type::List(t) => t.id(),
            Type::Text(t) => t.id(),
            Type::Integer(t) => t.id(),
            Type::Float(t) => t.id(),
            Type::Symbol(t) => t.id(),
        }
    }
}

pub trait TypeSum {
    fn kind(&self) -> String;
    fn id(&self) -> &str;
}

struct TypeId(&str);

/// A group of types.
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct TypeGroup {
    id    : String,
    label : String,
}

struct TypeGroupId(&str);

/// possible values are powerset 
/// sum type is size one
/// record size is max size
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Set {

    /// The [globally] unique identifer of the set type
    id           : String,

    /// A formal title for the type
    label        : String,

    /// A detailed description of the type
    description  : String,

    /// The type's parent group. Don't remember what groups are for...lol \-.o/
    #[serde(default)]
    parent_group : Option<TypeGroupId>,

    /// The group the type belong's to. Explain groups...
    #[serde(default)]
    group        : Option<TypeGroupId>,

    /// The group the type belong's to. Explain groups...
    #[serde(default)]
    required     : BTreeSet<TypeId>,

    /// The size of the set
    #[serde(default)]
    pub size     : SetSize,

    /// The types...
    pub types    : BTreeSet<TypeId>,

    /// A index of the set's types by 'id'
    #[serde(skip)]
    type_by_id   : BTreeMap<String,Type>,
}

impl Set {
    
    #[allow(dead_code)]
    pub fn new(id: &str, types: Vec<&Type>) -> Set {
        return Set {
            id: id.to_string(),
            label: String::from(""),
            description: String::from(""),
            parent_group: None,
            group: None,
            size: SetSize::All,
            types: types.iter().map(|&t| t.clone()).collect(),
            type_by_id: types.iter().map(|&typ| (typ.id().to_string(), typ.clone())).collect(),
        }
    }
}

impl TypeSum for Set {

    fn kind(&self) -> String {
        return String::from("set")
    }

    fn id(&self) -> &str {
        return &self.id
    }
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
#[allow(dead_code)]
pub enum SetSize {
    All,
    N(i64)
}

impl Default for SetSize {
    fn default() -> Self { SetSize::All }
}


/// List type, any number of values of the types
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct List {
  id           : String,
  label        : String,
  description  : String,
  parent_group : Option<TypeGroup>,
  group        : Option<TypeGroup>,
  pub types    : BTreeSet<Type>,
}

impl TypeSum for List {

    fn kind(&self) -> String {
        return String::from("list")
    }

    fn id(&self) -> &str {
        return &self.id
    }
}


/// The string type
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Text {
  id           : String,
  label        : String,
  description  : String,
  parent_group : Option<TypeGroup>,
  group        : Option<TypeGroup>,
}

impl TypeSum for Text {

    fn kind(&self) -> String {
        return String::from("text")
    }

    fn id(&self) -> &str {
        return &self.id
    }
}

impl Text {
    #[allow(dead_code)]
    pub fn new(id: &str) -> Text {
        return Text {
            id: id.to_string(),
            label: String::from(""),
            description: String::from(""),
            parent_group: None,
            group: None,
        }
    }
}

/// Integer
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Integer {
  id           : String,
  label        : String,
  description  : String,
  parent_group : Option<TypeGroup>,
  group        : Option<TypeGroup>,
}

impl TypeSum for Integer {

    fn kind(&self) -> String {
        return String::from("integer")
    }

    fn id(&self) -> &str {
        return &self.id
    }
}

impl Integer {
    #[allow(dead_code)]
    pub fn new(id: &str) -> Integer {
        return Integer {
            id: id.to_string(),
            label: String::from(""),
            description: String::from(""),
            parent_group: None,
            group: None,
        }
    }
}

/// Float
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Float {
  id           : String,
  label        : String,
  description  : String,
  parent_group : Option<TypeGroup>,
  group        : Option<TypeGroup>,
}

impl TypeSum for Float {

    fn kind(&self) -> String {
        return String::from("float")
    }

    fn id(&self) -> &str {
        return &self.id
    }
}


/// only id matters
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Symbol {
  id           : String,
  label        : String,
  description  : String,
  parent_group : Option<TypeGroup>,
  group        : Option<TypeGroup>,
}

impl TypeSum for Symbol {

    fn kind(&self) -> String {
        return String::from("symbol")
    }

    fn id(&self) -> &str {
        return &self.id
    }
}

impl Symbol {
    #[allow(dead_code)]
    pub fn new(id: &str) -> Symbol {
        return Symbol {
            id: id.to_string(),
            label: String::from(""),
            description: String::from(""),
            parent_group: None,
            group: None,
        }
    }
}


/// only id matters
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Timestamp {
  id           : String,
  label        : String,
  description  : String,
  parent_group : Option<TypeGroup>,
  group        : Option<TypeGroup>,
}

impl TypeSum for Timestamp {

    fn kind(&self) -> String {
        return String::from("timestamp")
    }

    fn id(&self) -> &str {
        return &self.id
    }
}

impl Timestamp {
    #[allow(dead_code)]
    pub fn new(id: &str) -> Timestamp {
        return Timestamp {
            id: id.to_string(),
            label: String::from(""),
            description: String::from(""),
            parent_group: None,
            group: None,
        }
    }
}

/// only id matters
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Date {
  id           : String,
  label        : String,
  description  : String,
  parent_group : Option<TypeGroup>,
  group        : Option<TypeGroup>,
}

impl TypeSum for Date {

    fn kind(&self) -> String {
        return String::from("date")
    }

    fn id(&self) -> &str {
        return &self.id
    }
}

impl Date {
    #[allow(dead_code)]
    pub fn new(id: &str) -> Date {
        return Date {
            id: id.to_string(),
            label: String::from(""),
            description: String::from(""),
            parent_group: None,
            group: None,
        }
    }
}

