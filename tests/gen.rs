mod config;

use config::Config;

use lulo::gen;
use lulo::gen::{
    Artifact,
    ArtifactProgLangTypes,
};


#[test]
fn test_rpg_character_simple() {

    // Setup
    let config = Config::from_env();
    let schema = config.test_schema("rpg");

    // Expected Value
    let expected_code_raw = r#"
        struct CharacterSimple {
            name: CharacterSimpleName,
            level: CharacterSimpleLevel,
            class: CharacterSimpleClass,
        };

        struct CharacterSimpleName(String);

        struct CharacterSimpleLevel(i64);

        struct CharacterSimpleClass(String);
    "#;
    let mut expected_code = expected_code_raw.to_string();
    remove_whitespace(&mut expected_code);

    // Calculated Value
    let artifact = Artifact::ProgLangTypes(ArtifactProgLangTypes::Rust);
    let code_result = gen::string(&schema, &artifact);
    match code_result {
        Ok(mut code) => {
            remove_whitespace(&mut code);
            assert_eq!(code, expected_code);
        },
        Err(err) => {
            panic!("{}", err.to_string());
        },
    }
}

fn remove_whitespace(s: &mut String) {
    s.retain(|c| !c.is_whitespace());
}
