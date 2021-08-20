//! Generate Lulo Types as Rust Data Structures

use askama::Template; // bring trait in scope
use crate::types::base::typ::TypeId;
use crate::types::obj::schema::Schema;
use crate::types::obj::typ::{Type, TypeObject};
use super::error::{
    Error, 
    ErrorCouldNotRenderTemplate,
    ErrorTypeNotInSchema,
    ErrorNotSupported,
};


#[derive(Template)]
#[template(path = "code/rust/type_set.tmpl", escape = "none")]
/// Set Type Template
struct SetTypeTemplate {
    type_name: String,
    fields: Vec<SetTypeFieldTemplate>,
}

struct SetTypeFieldTemplate {
    name: String,
    type_name: String,
}

#[derive(Template)]
#[template(path = "code/rust/type_primitive.tmpl", escape = "none")]
/// Text Type Template
struct PrimitiveTypeTemplate {
    type_id_title: String,
    rust_type: String,
}

/// A schema as Rust data types
pub fn schema_string(schema: &Schema) -> Result<String, Error> {

    let mut code = String::new();

    for typ in &schema.types {
        match typ {
            Type::Set(set_type) => {
                let member_types_res : Result<Vec<_>,_> = 
                    set_type.type_ids.iter().map(|type_id| {
                        set_type_field_template(type_id, schema)
                    }).collect();
                let member_types = member_types_res?;
                let template = SetTypeTemplate { 
                    type_name: typ.title(),
                    fields: member_types,
                };
                let template_str = template_string(&template)?; // then render it.
                code.push_str(&template_str);
            },
            Type::Text(_) => {
                let template = PrimitiveTypeTemplate {
                    type_id_title: typ.title(),
                    rust_type: String::from("String"),
                };
                let template_str = template_string(&template)?; // then render it.
                code.push_str(&template_str);
            },
            Type::Integer(_) => {
                let template = PrimitiveTypeTemplate {
                    type_id_title: typ.title(),
                    rust_type: String::from("i64"),
                };
                let template_str = template_string(&template)?; // then render it.
                code.push_str(&template_str);
            },
            Type::Symbol(_) => {
                let template = PrimitiveTypeTemplate {
                    type_id_title: typ.title(),
                    rust_type: String::from("String"),
                };
                let template_str = template_string(&template)?; // then render it.
                code.push_str(&template_str);
            },
            _ => {
                return Err(Error::NotSupported(ErrorNotSupported{
                    msg: typ.kind(),
                }))
            },
        }

        code.push_str("\n\n");
    }

    return Ok(code);
}

fn set_type_field_template(type_id: &TypeId, schema: &Schema) -> Result<SetTypeFieldTemplate, Error> {
    let typ = type_from_schema(type_id, schema)?;
    let type_id_str = type_id.to_string();
    let mut field_name = type_id_str.clone();
    let type_id_parts : Vec<&str> = type_id_str.split(".").collect(); 
    if type_id_parts.len() > 1 {
        field_name = type_id_parts.last().unwrap().to_string();
    }
    let template = SetTypeFieldTemplate {
        name: field_name,
        type_name: typ.title(),
    };
    return Ok(template);
}

fn template_string(template: &Template) -> Result<String, Error> {
    return template.render().or(
        Err(Error::CouldNotRenderTemplate(ErrorCouldNotRenderTemplate{}))
    );
}

// UTIL ------------------------------------------------------------------------

/// Get a Type from a Schema
fn type_from_schema<'a>(type_id: &TypeId, schema: &'a Schema) -> Result<&'a Type, Error> {
    let opt_type = schema.type_with_id(type_id);
    match opt_type {
        Some(typ) => Ok(typ),
        _ => Err(Error::TypeNotInSchema(ErrorTypeNotInSchema {
            missing_type_id: type_id.clone(),
        })),
    }
}

