//! Schema tests
//! Tests for public schema functions


mod common;

use lulo::atom::uri;


#[test]
fn object_from_uri_use_case_task_app() {

    let uri = common::data_uri("/use_cases/task_app/schema.yaml");
    //assertq!(true, false);

    let schema_or_err = uri::fetch::schema(&uri);
    match schema_or_err {
        Err(err) => {
            println!("{}", err.to_string());
            assert_eq!(true, false);
        },
        Ok(schema) => {
                let s = serde_yaml::to_string(&schema.to_atom()).unwrap();
                println!("schema:\n{}", s);
        },
    }
}
