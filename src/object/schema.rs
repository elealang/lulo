//! Schema
//! A collection of types


use std::collections::HashMap;

use crate::atom;
use crate::atom::typ::{Type, TypeId};


/// Schema objects
/// TODO use refs in hashmap?
pub struct Schema {
    pub id: String,
    pub namespace: String,
    pub types: Vec<Type>,
    type_by_id: HashMap<TypeId, Type>,
}

impl Schema {

    /// Create a schema object from a schema atom.
    pub fn from_atom(schema: &atom::schema::Schema) -> Schema {
        let mut type_by_id = HashMap::new();
        for typ in schema.types.iter() {
            type_by_id.insert(typ.id.clone(), typ.clone());
        }
        Schema {
            id: schema.id.clone(),
            namespace: schema.namespace.clone(),
            types: schema.types.iter().map(|x| x.clone()).collect(),
            type_by_id: type_by_id,
        }
    }

    /// To atom type
    pub fn to_atom(&self) -> atom::schema::Schema {
        return atom::schema::Schema {
            id: self.id.clone(),
            namespace: self.namespace.clone(),
            types: self.types.clone(),
        }
    }

    pub fn type_with_id(&self, type_id: &TypeId) -> Option<&Type> {
        return self.type_by_id.get(type_id);
    }

}
