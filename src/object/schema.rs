//! Schema
//! A collection of types


use std::collections::HashMap;

use crate::types::typ::{Type, TypeId};
use crate::types::uri::URI;


/// Schema objects
/// TODO use refs in hashmap?
pub struct Schema {
    pub id: String,
    pub namespace: String,
    pub types: Vec<Type>,
    type_by_id: HashMap<TypeId, Type>,
}

impl Schema {

    pub fn from_uri(uri: &URI) -> Result<Schema, Error> {

    }

    pub fn with_types(types: &Vec<Type>) -> Schema {
        let mut type_by_id = HashMap::new();
        for typ in types.iter() {
            type_by_id.insert(typ.id.clone(), typ.clone());
        }
        Schema {
            types: types.iter().map(|x| x.clone()).collect(),
            type_by_id: type_by_id,
        }
    }

    pub fn type_with_id(&self, type_id: &TypeId) -> Option<&Type> {
        return self.type_by_id.get(type_id);
    }
}
