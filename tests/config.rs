use dotenv;
use envy;
use serde::Deserialize;
use std::path::Path;

use lulo::types::base::typ::TypeId;
use lulo::types::obj::value::{IntegerValue, SetValue, SymbolValue, TextValue, Value};
use lulo::types::obj::schema::Schema;
use lulo::uri;


#[derive(Deserialize, Debug)]
pub struct Config {
    lulo_test_values_dir: String,
    lulo_test_schemas_dir: String,
}

impl Config {

    pub fn from_env() -> Config {
        // Load testing env vars
        dotenv::from_filename(".env.test").ok();

        match envy::from_env::<Config>() {
            Ok(conf) => return conf,
            Err(e) => panic!("Couldn't read test configuration: ({})", e),
        };
    }

    pub fn test_schema_file_path(&self, schema_name: &str) -> String {
        let test_schemas_dir = Path::new(&self.lulo_test_schemas_dir);
        let schema_name_path = Path::new(schema_name);
        let path = test_schemas_dir.join(schema_name_path);
        match path.to_str() {
            Some(path_str) => return format!("{}.yaml", String::from(path_str)),
            None => panic!("Could not get schema file path"),
        }
    }

    pub fn test_schema_file_uri(&self, schema_name: &str) -> String {
        let filepath = self.test_schema_file_path(schema_name);
        return format!("file:/{}", filepath);
    }

    pub fn test_schema(&self, schema_name: &str) -> Schema {
        let schema_uri = self.test_schema_file_uri(schema_name);
        match uri::schema(&schema_uri) {
            Ok(schema) => return schema,
            Err(err) => panic!("{}", err.to_string()),
        }
    }

    // add test function to get a test value
    pub fn test_value_file_path(&self, value_name: &str) -> String {
        let test_value_dir = Path::new(&self.lulo_test_values_dir);
        let value_name_path = Path::new(value_name);
        let path = test_value_dir.join(value_name_path);
        match path.to_str() {
            Some(path_str) => return format!("{}.yaml", String::from(path_str)),
            None => panic!("Could not get value file path"),
        }
    }
    
    pub fn test_value_file_uri(&self, value_name: &str) -> String {
        let filepath = self.test_value_file_path(value_name);
        return format!("file:/{}", filepath);
    }

    pub fn test_value(&self, value_name: &str) -> Value {
        let value_uri = self.test_value_file_uri(value_name);
        match uri::value(&value_uri) {
            Ok(value) => return value,
            Err(err) => panic!("{}", err.to_string()),
        }
    }
}


// DATASETS --------------------------------------------------------------------

pub fn value_character_simple(config: &Config) -> Value {
    return Value::Set(SetValue::from_vec(
        vec![
            (
                TypeId::from_string("name"),
                Value::Text(TextValue::from_string("Osmar")),
            ),
            (
                TypeId::from_string("level"),
                Value::Integer(IntegerValue::from_integer(2)),
            ),
            (
                TypeId::from_string("class"),
                Value::Symbol(SymbolValue::from_string("cleric")),
            ),
        ]
        .into_iter()
        .collect(),
        Some(config.test_value_file_uri("rpg/character_simple")),
    ));
}
