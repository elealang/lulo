
mod common;
use lulo::uri;

#[test]
fn test_value_local_file_system() {

    let config = common::setup();

        
    let uri_value = uri::value(
                        format!("file:/{}", 
                        config.test_value_file_path("character_basic"),
                    )

    assert_eq!(uri_value, common::value_character_simple);
}
