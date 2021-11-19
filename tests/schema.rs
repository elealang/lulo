//! Schema tests
//! Tests for public schema functions


mod common;


use lulo::uri::URI;


#[test]
fn object_from_uri_use_case_task_app() {

    let uri = common::data_uri("/use_cases/task_app/schema.yaml");

    //assert!(validation_res.is_ok());
}
