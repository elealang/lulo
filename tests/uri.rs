mod config;

use lulo::uri;
use lulo::types::base::typ::TypeId;
use lulo::types::obj::value::{
    Value,
    SetValue,
    TextValue, IntegerValue, SymbolValue
};
use config::Config;

#[test]
fn test_uri_rpg_character_simple() {

    let config = Config::from_env();

    let value_uri = config.test_value_file_uri("rpg/character_simple");
    let uri_value = uri::value(&value_uri);

    assert_eq!(uri_value, Ok(value_character_simple(&config)));
}

// TEST VALUES -----------------------------------------------------------------
// Hardcoded values for testing uri functions (otherwise test values are loaded
// using these functions)

pub fn value_character_simple(config: &Config) -> Value {
    return Value::Set(SetValue::from_vec(
        vec![
            (
                TypeId::from_string("character_simple_name"),
                Value::Text(TextValue::from_string("Osmar")),
            ),
            (
                TypeId::from_string("character_simple_level"),
                Value::Integer(IntegerValue::from_integer(2)),
            ),
            (
                TypeId::from_string("character_simple_class"),
                Value::Symbol(SymbolValue::from_string("cleric")),
            ),
        ]
        .into_iter()
        .collect(),
        Some(config.test_value_file_uri("rpg/character_simple")),
    ));
}
