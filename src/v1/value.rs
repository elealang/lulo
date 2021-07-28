
use std::collections::HashMap;

#[allow(dead_code)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Value {
    Set(Set),
    List(List),
    Text(Text),
    Integer(Integer),
    Float(Float),
    Symbol(Symbol),
} 


/// Set values
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Set {
    pub map: HashMap<String, Value>,
} 

impl Set {

    #[allow(dead_code)]
    pub fn from_map(v: HashMap<String, Value>) -> Set {
        Set {
            map: v,
        }
    }
}


/// List value
#[allow(dead_code)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct List {
    pub values: Vec<Value>,
} 


/// Text value
#[allow(dead_code)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Text {
    value: String,
} 

impl Text {

    #[allow(dead_code)]
    pub fn from_string(s: &str) -> Text {
        Text {
            value: s.to_string(),
        }
    }
}


/// Integer value
#[allow(dead_code)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Integer {
    value: i64,
} 

impl Integer {

    #[allow(dead_code)]
    pub fn from_integer(i: i64) -> Integer {
        Integer {
            value: i,
        }
    }
}


/// Float value
#[allow(dead_code)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Float {
    value: i64,
} 


/// Symbol value
#[allow(dead_code)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Symbol {
    value: i64,
} 

impl Symbol {

    #[allow(dead_code)]
    pub fn from_string(_: &str) -> Symbol {
        Symbol {
            value: 1000,
        }
    }
}


// SERIALIZATION FUNCTIONS
// =================================================================================================

//fn from_yaml_file(filepath: &str) -> Result<Value, serde_yaml::Error> {

    //let f = std::fs::File::open(filepath)?;
    //let yaml_string: String = serde_yaml::from_reader(f)?;

    //let de = serde_yaml::Deserializer::from_str(yaml_string);
    //let value = Value::deserialize(de)?;

    //is_type(value, )

//}
