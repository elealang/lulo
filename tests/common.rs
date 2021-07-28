
use lulo::value;
use lulo::value::Value;

use dotenv;
use envy;
use serde::{Deserialize};
use std::path::Path;

// SETUP -----------------------------------------------------------------------

pub fn setup() -> Config {

    // Load testing env vars
    dotenv::from_filename(".env.test").ok();

    match envy::from_env::<ConfigEnv>() {
        Ok(config_env) => {
            return Config::new(&config_env);
        },
        Err(e) => panic!("Couldn't read test configuration: ({})", e),
    };
}


// CONFIG ----------------------------------------------------------------------

#[derive(Deserialize, Debug)]
struct ConfigEnv {
    lulo_test_values_dir: String,
}

struct Config {
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
}

// DATASETS --------------------------------------------------------------------

pub static value_character_simple: value::Value = 
    Value::Set(value::Set::from_map(
        vec![
            (
                String::from("name"), 
                Value::Text(value::Text::from_string("Osmar")),
            ),
            (
                String::from("level"), 
                Value::Integer(value::Integer::from_integer(2)),
            ),
            (
                String::from("class"), 
                Value::Symbol(value::Symbol::from_string("cleric")),
            ),
        ].into_iter().collect(),
    ));

