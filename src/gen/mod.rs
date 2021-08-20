pub mod code;
pub mod error;

use crate::types::obj::schema::Schema;
use crate::gen::code::rust;
use crate::gen::error::{
    Error,
    ErrorCouldNotGenerateCode,
};


/// Generate the artifact as a string
pub fn string(schema: &Schema, artifact: &Artifact) -> Result<String, Error> {
    match artifact {
        Artifact::ProgLangTypes(art) => string_prog_lang_types(schema, art),
        Artifact::HTML(art)          => string_html(schema, art),
    }
}

/// Generate programming language data types from a Schema as a string
fn string_prog_lang_types(schema: &Schema, artifact: &ArtifactProgLangTypes) -> Result<String, Error> {
    match artifact {
        ArtifactProgLangTypes::Rust => {
            match rust::schema_string(schema) {
                Ok(s)  => Ok(s),
                Err(err) => {
                    Err(Error::CouldNotGenerateCode(ErrorCouldNotGenerateCode{
                        err: err,
                    }))
                },
            }
        }
    }
}

/// Generate an HTML view of a Schema as a string
fn string_html(schema: &Schema, artifact: &ArtifactHTML) -> Result<String, Error> {
    match artifact {
        ArtifactHTML => Ok(String::from("")),
    }
}



pub enum Artifact {
    ProgLangTypes(ArtifactProgLangTypes),
    HTML(ArtifactHTML),
}

pub enum ArtifactProgLangTypes {
    Rust,
}

pub struct ArtifactHTML {}
