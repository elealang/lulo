
use lulo::validate;
use lulo::types::base::typ::TypeId;
use lulo::types::obj::schema::Schema;
use lulo::types::obj::typ::{IntegerType, SetType, SymbolType, TextType, Type};
use lulo::types::obj::value::{IntegerValue, SetValue, SymbolValue, TextValue, Value};


#[test]
fn test_is_type_simple_product_type() {

    // Type: Character Name
    let type_name_id = TypeId::from_string("name");
    let type_name = Type::Text(TextType::new(type_name_id.clone()));

    // Type: Character Level
    let type_level_id = TypeId::from_string("level");
    let type_level = Type::Integer(IntegerType::new(type_level_id.clone()));

    // Type: Character Class
    let type_class_id = TypeId::from_string("class");
    let type_class = Type::Symbol(SymbolType::new(type_class_id.clone()));

    // Type: Character
    let type_simple_prod_set_id = TypeId::from_string("simple_prod_set_type");
    let type_simple_prod_set = Type::Set(SetType::new(
        type_simple_prod_set_id,
        vec![
            type_name_id.clone(),
            type_level_id.clone(),
            type_class_id.clone(),
        ],
    ));

    // Schema
    let schema = Schema::with_types(&vec![type_name, type_level, type_class]);

    let value_is_type_1 = Value::Set(SetValue::from_vec(
        vec![
            (type_name_id, Value::Text(TextValue::from_string("Osmar"))),
            (type_level_id, Value::Integer(IntegerValue::from_integer(2))),
            (
                type_class_id,
                Value::Symbol(SymbolValue::from_string("cleric")),
            ),
        ],
        None,
    ));

    let validation_res = validate::is_type(&value_is_type_1, &type_simple_prod_set, &schema);
    assert!(validation_res.is_ok());
}
