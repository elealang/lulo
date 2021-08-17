mod common;
use lulo::uri;

#[test]
fn test_value_local_file_system() {
    let config = common::setup();

    let filepath_char_simple = config.test_value_file_path("rpg/character_simple.yaml");
    let uri_value = uri::value(&format!("file:/{}", &filepath_char_simple));

    assert_eq!(uri_value, Ok(common::value_character_simple(&config)));
}
