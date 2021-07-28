
use std::collections::HashMap;

use crate::v1::typ;
use crate::v1::typ::TypeSum;


#[derive(Serialize)]
#[allow(dead_code)]
/// The Schema Schema
pub struct Schema {

    types: Vec<typ::Type>,

    #[serde(skip_serializing)]
    type_catalog: HashMap<String,typ::Type>

}

impl Schema {

    #[allow(dead_code)]
    fn with_types(types: &Vec<typ::Type>) -> Schema {
        let mut catalog_hm = HashMap::new();
        for _type in types.iter() {
            catalog_hm.insert(_type.id().to_string(), _type.clone());
        }
        Schema{
            type_catalog: catalog_hm,
        }
    }
}
