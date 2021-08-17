use std::collections::HashMap;

use crate::types::base;
use crate::types::base::typ::TypeSum;
use crate::types::obj;


/// The Schema Schema
/// TODO use refs in hashmap
pub struct Schema {
    types: Vec<obj::typ::Type>,
    type_by_id: HashMap<base::typ::TypeId, obj::typ::Type>,
}

impl Schema {

    pub fn from_base(schema: base::schema::Schema) -> Self {
        Schema::with_types(&schema.types.iter().map(|typ| {
            obj::typ::Type::from_base(typ.clone())
        }).collect())
    }

    pub fn with_types(types: &Vec<obj::typ::Type>) -> Schema {
        let mut type_by_id = HashMap::new();
        for typ in types.iter() {
            type_by_id.insert(typ.id().clone(), typ.clone());
        }
        Schema {
            types: types.iter().map(|x| x.clone()).collect(),
            type_by_id: type_by_id,
        }
    }

    pub fn type_with_id(&self, type_id: &base::typ::TypeId) -> Option<&obj::typ::Type> {
        return self.type_by_id.get(type_id);
    }
}
