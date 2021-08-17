use dotenv;
use envy;
use serde::Deserialize;
use std::path::Path;

use lulo::types::base::typ::TypeId;
use lulo::types::obj::value::{IntegerValue, SetValue, SymbolValue, TextValue, Value};

// SETUP -----------------------------------------------------------------------

pub fn setup() -> Config {
    // Load testing env vars
    dotenv::from_filename(".env.test").ok();

    match envy::from_env::<ConfigEnv>() {
        Ok(config_env) => {
            return Config::new(&config_env);
        }
        Err(e) => panic!("Couldn't read test configuration: ({})", e),
    };
}

// CONFIG ----------------------------------------------------------------------

#[derive(Deserialize, Debug)]
struct ConfigEnv {
    lulo_test_values_dir: String,
}

pub struct Config {
    lulo_test_values_dir: String,
}

impl Config {
    fn new(config_env: &ConfigEnv) -> Self {
        Config {
            lulo_test_values_dir: String::from(&config_env.lulo_test_values_dir),
        }
    }

    // add test function to get a test value
    pub fn test_value_file_path(&self, value_name: &str) -> String {
        let test_value_dir = Path::new(&self.lulo_test_values_dir);
        let value_name_path = Path::new(value_name);
        let path = test_value_dir.join(value_name_path);
        match path.to_str() {
            Some(path_str) => return String::from(path_str),
            None => panic!("Could not get value file path"),
        }
    }
    
    pub fn test_value_file_uri(&self, value_name: &str) -> String {
        let filepath = self.test_value_file_path(value_name);
        return format!("file:/{}", filepath);
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
        Some(config.test_value_file_uri("rpg/character_simple.yaml")),
    ));
}
