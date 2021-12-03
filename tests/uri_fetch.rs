//! Schema tests
//! Tests for public schema functions


mod common;

use lulo::atom::change::{Change, Operation, AddOperation};
use lulo::atom::schema::Schema;
use lulo::atom::typ::{Type, TypeId, Kind};
use lulo::atom::value::{Value, SetValue, SetValueMember, RegisterValue};
use lulo::atom::uri;


#[test]
fn object_from_uri_use_case_task_app() {

    let uri = common::data_uri("/use_cases/task_app/schema.yaml");

    // experiment deserializae schema
    //let change = Change {
        //required: true,
        //operation: Operation::Add(AddOperation{
            //value_of_type: TypeId::from_string("type"),
        //}),
    //};
    //let typ = Type {
        //id: TypeId::from_string("my_type"),
        //value: Value::Set(SetValue{
            //values: vec![SetValueMember{
                //type_id: TypeId::from_string("dog"),
                //value: Value::Register(RegisterValue{
                    //uri: uri.clone(),
                //}),
            //}]
        //}),
        //changes: vec![change],
    //};
    //let schema = Schema {
        //id: String::from("id"),
        //namespace: String::from("namespace"),
        //types: vec![typ],
    //};

    //let str_or_err = serde_yaml::to_string(&schema);
    //match str_or_err {
        //Ok(s)    => println!("{}", s),
        //Err(err) => println!("{}", err),
    //};

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
