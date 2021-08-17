//! Module Coding Style: Code here should be written carefully for optimized
//! performance this it represents the most critical code path in most use cases.

use crate::types::base::typ::{SetSize, TypeId, TypeSum};
use crate::types::obj::schema::Schema;
use crate::types::obj::typ::{
    DateType, FloatType, IntegerType, ListType, SetType, SymbolType, TextType, 
    TimestampType, Type
};
use crate::types::obj::value::{SetValue, Value, ValueObject};


/// Parse YAML
pub fn is_type(test_value: &Value, expected_type: &Type, schema: &Schema) -> Result<IsTypeValidation,Error> {

    let is_type_validation = IsTypeValidation {
        input: IsTypeValidationInput {
            value_uri: (*test_value).origin(),
            type_id: (*expected_type).id().clone(),
        }
    };

    match expected_type {
        Type::Set(t) => is_set_type(test_value, t, schema).map(|_| is_type_validation),
        Type::List(t) => is_list_type(test_value, t, schema).map(|_| is_type_validation),
        Type::Text(t) => is_text_type(test_value, t).map(|_| is_type_validation),
        Type::Integer(t) => is_integer_type(test_value, t).map(|_| is_type_validation),
        Type::Float(t) => is_float_type(test_value, t).map(|_| is_type_validation),
        Type::Symbol(t) => is_symbol_type(test_value, t).map(|_| is_type_validation),
        Type::Timestamp(t) => is_timestamp_type(test_value, t).map(|_| is_type_validation),
        Type::Date(t) => is_date_type(test_value, t).map(|_| is_type_validation),
    }
}

/// Is Set Type?
fn is_set_type(test_value: &Value, set_type: &SetType, schema: &Schema) -> Result<(),Error> {
    match test_value {
        // Only a set value can be a set type.
        Value::Set(set_value) => {
            // The default "size" of a set type is ALL which is the same
            // as a typical product type.
            //
            // MATCH ALL
            if set_type.size == SetSize::All {
                if set_type.type_ids.len() != set_value.values.len() {
                    return Err(Error::SetHasWrongSize(ErrorSetHasWrongSize {
                        set_value: (*set_value).clone(),
                        set_type: set_type.clone(),
                    }));
                }

                for type_id in set_type.type_ids.iter() {
                    // check if type in value map and then type check
                    if !set_value.values.contains_key(type_id) {
                        return Err(Error::SetMissingValue(ErrorSetMissingValue {
                            set_type: set_type.clone(),
                            missing_type_id: type_id.clone(),
                        }));
                    }

                    type_from_schema(type_id, schema).and_then(|typ| {
                        is_type(set_value.values.get(type_id).unwrap(), &typ, schema)
                    })?;
                }

                return Ok(());
            }
            // MATCH N
            else {
                return Err(Error::NotSupported(ErrorNotSupported {}));
            }
        }
        _ => {
            return Err(Error::UnexpectedValue(ErrorUnexpectedValue {
                value: (*test_value).clone(),
                target_type: Type::Set(set_type.clone()),
            }));
        }
    }
}

/// Is List Type?
fn is_list_type(test_value: &Value, list_type: &ListType, schema: &Schema) -> Result<(),Error> {
    match test_value {
        // Only a set value can be a set type.
        Value::List(list_value) => {
            // Get list types by id
            let list_types_result: Result<Vec<_>, _> = list_type
                .type_ids
                .iter()
                .map(|type_id| type_from_schema(&type_id, schema))
                .collect();
            let list_types = list_types_result?;

            // Each value in the list...
            for val in list_value.values.iter() {
                let matches_type = false;
                // Should match at least one of these types
                for typ in &list_types {
                    is_type(val, typ, schema)?;
                }

                if !matches_type {
                    return Err(Error::ListValueOfUnexpectedType(
                        ErrorListValueOfUnexpectedType {
                            value: val.clone(),
                            list_type: list_type.clone(),
                        },
                    ));
                }
            }
            return Ok(());
        }
        _ => Err(Error::UnexpectedValue(ErrorUnexpectedValue {
            value: (*test_value).clone(),
            target_type: Type::List(list_type.clone()),
        })),
    }
}

/// Is Text Type
fn is_text_type(test_value: &Value, text_type: &TextType) -> Result<(),Error> {
    match test_value {
        Value::Text(_) => return Ok(()),
        _ => {
            return Err(Error::UnexpectedValue(ErrorUnexpectedValue {
                value: (*test_value).clone(),
                target_type: Type::Text(text_type.clone()),
            }));
        }
    }
}

/// Is Integer Type?
fn is_integer_type(test_value: &Value, integer_type: &IntegerType) -> Result<(),Error> {
    match test_value {
        Value::Integer(_) => return Ok(()),
        _ => {
            return Err(Error::UnexpectedValue(ErrorUnexpectedValue {
                value: (*test_value).clone(),
                target_type: Type::Integer(integer_type.clone()),
            }));
        }
    }
}

/// Is Float Type?
fn is_float_type(test_value: &Value, float_type: &FloatType) -> Result<(),Error> {
    match test_value {
        Value::Float(_) => return Ok(()),
        _ => {
            return Err(Error::UnexpectedValue(ErrorUnexpectedValue {
                value: (*test_value).clone(),
                target_type: Type::Float(float_type.clone()),
            }));
        }
    }
}

/// Is Symbol Type?
fn is_symbol_type(test_value: &Value, symbol_type: &SymbolType) -> Result<(),Error> {
    match test_value {
        Value::Symbol(_) => return Ok(()),
        _ => {
            return Err(Error::UnexpectedValue(ErrorUnexpectedValue {
                value: (*test_value).clone(),
                target_type: Type::Symbol(symbol_type.clone()),
            }));
        }
    }
}

/// Is Timestamp Type?
fn is_timestamp_type(test_value: &Value, timestamp_type: &TimestampType) -> Result<(),Error> {
    match test_value {
        Value::Timestamp(_) => return Ok(()),
        _ => {
            return Err(Error::UnexpectedValue(ErrorUnexpectedValue {
                value: (*test_value).clone(),
                target_type: Type::Timestamp(timestamp_type.clone()),
            }));
        }
    }
}

/// Is Date Type?
fn is_date_type(test_value: &Value, date_type: &DateType) -> Result<(),Error> {
    match test_value {
        Value::Date(_) => return Ok(()),
        _ => {
            return Err(Error::UnexpectedValue(ErrorUnexpectedValue {
                value: (*test_value).clone(),
                target_type: Type::Date(date_type.clone()),
            }));
        }
    }
}

// TYPES -----------------------------------------------------------------------

pub struct IsTypeValidation {
    input: IsTypeValidationInput,
}

impl ToString for IsTypeValidation {
    fn to_string(&self) -> String {
        return self.input.to_string();     
    }
}

pub struct IsTypeValidationInput {
    value_uri: Option<String>, 
    type_id: TypeId, 
}

impl ToString for IsTypeValidationInput {
    fn to_string(&self) -> String {
        return format!(
            "Value {} has type {}", 
            self.value_uri.clone().unwrap_or(String::from("unknown_origin")),
            self.type_id.to_string(),
        );
    }
}

// UTILS -----------------------------------------------------------------------

fn type_from_schema<'a>(type_id: &TypeId, schema: &'a Schema) -> Result<&'a Type,Error> {
    let opt_type = schema.type_with_id(type_id);
    match opt_type {
        Some(typ) => Ok(typ),
        _ => Err(Error::TypeNotInSchema(ErrorTypeNotInSchema {
            missing_type_id: type_id.clone(),
        })),
    }
}

// ERRORS ----------------------------------------------------------------------

#[derive(Debug, Eq, PartialEq)]
/// Validation Errors
pub enum Error {
    NotSupported(ErrorNotSupported),
    UnexpectedValue(ErrorUnexpectedValue),
    SetHasWrongSize(ErrorSetHasWrongSize),
    SetMissingValue(ErrorSetMissingValue),
    ListValueOfUnexpectedType(ErrorListValueOfUnexpectedType),
    TypeNotInSchema(ErrorTypeNotInSchema),
}

impl ToString for Error {
    fn to_string(&self) -> String {
        match &*self {
            Error::NotSupported(err)              => err.to_string(),
            Error::UnexpectedValue(err)           => err.to_string(),
            Error::SetHasWrongSize(err)           => err.to_string(),
            Error::SetMissingValue(err)           => err.to_string(),
            Error::ListValueOfUnexpectedType(err) => err.to_string(),
            Error::TypeNotInSchema(err)           => err.to_string(),
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct ErrorNotSupported {}

impl ToString for ErrorNotSupported {
    fn to_string(&self) -> String {
        format!("Feature not supported")
    }
}

#[derive(Debug, Eq, PartialEq)]
struct ErrorUnexpectedValue {
    value: Value,
    target_type: Type,
}

impl ToString for ErrorUnexpectedValue {
    fn to_string(&self) -> String {
        return format!(
            "Unexpected value [{}] for target type [{}]", 
            self.value.origin().unwrap_or(String::from("Unknown origin")), 
            self.target_type.id().to_string(),
        );
    }
}

#[derive(Debug, Eq, PartialEq)]
struct ErrorSetHasWrongSize {
    set_type: SetType,
    set_value: SetValue,
}

impl ToString for ErrorSetHasWrongSize {
    fn to_string(&self) -> String {
        return format!(
            "Set {} should have size: {}", 
            self.set_value.origin().unwrap_or(String::from("unknown_origin")),
            self.set_type.size.to_string(),
        );
    }
}

#[derive(Debug, Eq, PartialEq)]
struct ErrorSetMissingValue {
    set_type: SetType,
    missing_type_id: TypeId,
}

impl ToString for ErrorSetMissingValue {
    fn to_string(&self) -> String {
        return format!("Set missing value of type {}", self.missing_type_id.to_string());
    }
}

#[derive(Debug, Eq, PartialEq)]
struct ErrorListValueOfUnexpectedType {
    value: Value,
    list_type: ListType,
}

impl ToString for ErrorListValueOfUnexpectedType {
    fn to_string(&self) -> String {
        return format!(
            "Found list value {} of unexpected type: {}", 
            self.value.origin().unwrap_or(String::from("unknown_origin")),
            self.list_type.id().to_string(),
        );
    }
}

#[derive(Debug, Eq, PartialEq)]
struct ErrorTypeNotInSchema {
    missing_type_id: TypeId,
}

impl ToString for ErrorTypeNotInSchema {
    fn to_string(&self) -> String {
        return format!("Type {} not in schema", self.missing_type_id.to_string());
    }
}

// TESTS -----------------------------------------------------------------------

#[cfg(test)]
mod tests {

    use crate::validate::is_type;
    use crate::types::base::typ::TypeId;
    use crate::types::obj::schema::Schema;
    use crate::types::obj::typ::{IntegerType, SetType, SymbolType, TextType, Type};
    use crate::types::obj::value::{IntegerValue, SetValue, SymbolValue, TextValue, Value};

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

        //let value_not_type_1 = Value::Set(value::Set::from_map(
        //vec![
        //(String::from("name"), Value::Text(value::Text::from_string("Osmar"))),
        //(String::from("class"), Value::Symbol(value::Symbol::from_string("cleric"))),
        //].into_iter().collect(),
        //));

        let validation_res = is_type(&value_is_type_1, &type_simple_prod_set, &schema);
        assert!(validation_res.is_ok());
        //assert_eq!(is_type(&value_is_type_1, &SIMPLE_PROD_SET_TYPE),
        //Some(ErrIsType::SetMissingValue(ErrIsTypeSetMissingValue{
        //set_type: SIMPLE_PROD_SET_TYPE,
        //missing_type_id: "level",
        //})));
    }
}
