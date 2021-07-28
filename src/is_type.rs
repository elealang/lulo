//! Module Coding Style: Code here should be written carefully for optimized 
//! performance this it represents the most critical code path in most use cases.

use crate::typ;
use crate::typ::{Type, TypeSum};
use crate::value;
use crate::value::Value;

/// Parse YAML
pub fn is_type(test_value: &value::Value, expected_type: &typ::Type) -> Option<ErrIsType> {
    match expected_type {
        Type::Set(t) => is_set_type(test_value, t),
        Type::List(t) => is_list_type(test_value, t),
        Type::Text(t) => is_text_type(test_value, t),
        Type::Integer(t) => is_integer_type(test_value, t),
        Type::Float(t) => is_float_type(test_value, t),
        Type::Symbol(t) => is_symbol_type(test_value, t),
    }
}

/// Is Set Type?
fn is_set_type(test_value: &value::Value, set_type: &typ::Set) -> Option<ErrIsType> {
    match test_value {
        // Only a set value can be a set type. 
        Value::Set(set_value) => {
            // The default "size" of a set type is ALL which is the same
            // as a typical product type.
            
            // MATCH ALL
            if set_type.size == typ::SetSize::All {

                if set_type.types.len() != set_value.map.len() {
                    return Some(ErrIsType::SetHasWrongSize(ErrIsTypeSetHasWrongSize{
                        set_value: (*set_value).clone(),
                        set_type: set_type.clone(),
                    }))
                }

                for typ in set_type.types.iter() {
                    // check if type in value map and then type check
                    if !set_value.map.contains_key(typ.id()) {
                        return Some(ErrIsType::SetMissingValue(ErrIsTypeSetMissingValue{
                            set_type: set_type.clone(),
                            missing_type_id: typ.id().to_string(),
                        }))
                    }
                
                    let opt_error = is_type(set_value.map.get(typ.id()).unwrap(), &typ);
                    match opt_error {
                        Some(ref _err) => {
                            return opt_error
                        },
                        _ => {},
                    }
                }

                return None
            } 
            // MATCH N
            else {
                return Some(ErrIsType::NotSupported(ErrIsTypeNotSupported{}))
            }
        }
        _ => {
            return Some(ErrIsType::UnexpectedValue(ErrIsTypeUnexpectedValue{
                value: (*test_value).clone(),
                target_type: Type::Set(set_type.clone()),
            }));
        }
    }
}

/// Is List Type?
fn is_list_type(test_value: &value::Value, list_type: &typ::List) -> Option<ErrIsType> {
    match test_value {
        // Only a set value can be a set type. 
        Value::List(list_value) => {
            // Each value should be one of the types.
            for val in list_value.values.iter() {
                let mut matches_type = false;
                for typ in list_type.types.iter() {
                    let is_type_opt = is_type(&val, &typ);
                    match is_type_opt {
                        None => {
                            matches_type = true;
                        },
                        _ => {},
                    }
                }

                if !matches_type {
                    return Some(ErrIsType::ListValueOfUnexpectedType(
                        ErrIsTypeListValueOfUnexpectedType {
                            value: val.clone(),
                            list_type: list_type.clone(),
                        }
                    ));
                }

            }
            return None
        }
        _ => {
            Some(ErrIsType::UnexpectedValue(ErrIsTypeUnexpectedValue{
                value: (*test_value).clone(),
                target_type: Type::List(list_type.clone()),
            }))
        }
    }
}

/// Is Text Type
fn is_text_type(test_value: &value::Value, text_type: &typ::Text) -> Option<ErrIsType> {
    match test_value {
        Value::Text(_) => {
            return None
        }
        _  => {
            return Some(ErrIsType::UnexpectedValue(ErrIsTypeUnexpectedValue{
                value: (*test_value).clone(),
                target_type: Type::Text(text_type.clone()),
            }));
        }
    }
}

/// Is Integer Type?
fn is_integer_type(test_value: &value::Value, integer_type: &typ::Integer) -> Option<ErrIsType> {
    match test_value {
        Value::Integer(_) => {
            return None
        }
        _ => {
            return Some(ErrIsType::UnexpectedValue(ErrIsTypeUnexpectedValue{
                value: (*test_value).clone(),
                target_type: Type::Integer(integer_type.clone()),
            }));
        }
    }
}

/// Is Float Type?
fn is_float_type(test_value: &value::Value, float_type: &typ::Float) -> Option<ErrIsType> {
    match test_value {
        Value::Float(_) => {
            return None
        }
        _ => {
            return Some(ErrIsType::UnexpectedValue(ErrIsTypeUnexpectedValue{
                value: (*test_value).clone(),
                target_type: Type::Float(float_type.clone()),
            }));
        }
    }
}

/// Is Symbol Type?
fn is_symbol_type(test_value: &value::Value, symbol_type: &typ::Symbol) -> Option<ErrIsType> {
    match test_value {
        Value::Symbol(_) => {
            return None
        }
        _ => {
            return Some(ErrIsType::UnexpectedValue(ErrIsTypeUnexpectedValue{
                value: (*test_value).clone(),
                target_type: Type::Symbol(symbol_type.clone()),
            }));
        }
    }
}

// ERRORS ----------------------------------------------------------------------

#[derive(Debug, Eq, PartialEq)]
pub enum ErrIsType {
    NotSupported(ErrIsTypeNotSupported),
    UnexpectedValue(ErrIsTypeUnexpectedValue),
    SetHasWrongSize(ErrIsTypeSetHasWrongSize),
    SetMissingValue(ErrIsTypeSetMissingValue),
    ListValueOfUnexpectedType(ErrIsTypeListValueOfUnexpectedType),
}

#[derive(Debug, Eq, PartialEq)]
pub struct ErrIsTypeNotSupported {
}

#[derive(Debug, Eq, PartialEq)]
struct ErrIsTypeUnexpectedValue {
    value: value::Value,
    target_type: typ::Type,
}

#[derive(Debug, Eq, PartialEq)]
struct ErrIsTypeSetHasWrongSize {
    set_type: typ::Set,
    set_value: value::Set,
}

#[derive(Debug, Eq, PartialEq)]
struct ErrIsTypeSetMissingValue {
    set_type: typ::Set,
    missing_type_id: String,
}

#[derive(Debug, Eq, PartialEq)]
struct ErrIsTypeListValueOfUnexpectedType {
    value: value::Value,
    list_type: typ::List,
}

// TESTS -----------------------------------------------------------------------

#[cfg(test)]
mod tests {

    use crate::is_type::{is_type};
    use crate::value;
    use crate::value::{Value};
    use crate::typ;
    use crate::typ::{Type};

    #[test]
    fn test_is_type_simple_product_type() {

        // Character Name
        let type_name = Type::Text(typ::Text::new("name"));

        // Character Level
        let type_level = Type::Integer(typ::Integer::new("level"));

        // Character Class
        let type_class = Type::Symbol(typ::Symbol::new("class"));

        // Character
        let type_simple_prod_set = Type::Set(typ::Set::new(
            &String::from("simple_prod_set_type"),
            vec![
                &type_name,
                &type_level,
                &type_class,
            ]
        ));

        let value_is_type_1 = Value::Set(value::Set::from_map(
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
    
        //let value_not_type_1 = Value::Set(value::Set::from_map(
            //vec![
                //(String::from("name"), Value::Text(value::Text::from_string("Osmar"))),
                //(String::from("class"), Value::Symbol(value::Symbol::from_string("cleric"))),
            //].into_iter().collect(),
        //));

        assert_eq!(is_type(&value_is_type_1, &type_simple_prod_set), None);
        //assert_eq!(is_type(&value_is_type_1, &SIMPLE_PROD_SET_TYPE), 
                   //Some(ErrIsType::SetMissingValue(ErrIsTypeSetMissingValue{
                        //set_type: SIMPLE_PROD_SET_TYPE,
                        //missing_type_id: "level",
                   //})));
    
    }

}

