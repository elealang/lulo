//! A Type is Rule of Inference that describes a 
//! change in values.

use crate::types::base::change::{
    Change,
    Add,
};
use crate::types::base::value::{
    Value,
    Text, TextAll, TextEmpty, TextLiteral,
};


/// Rule
struct Rule {
    premise: Change,
    conclusion: Value,
}



fn test() {
    let is_text = Rule {
        premise: Change::Add(Add {
            value: Value::Text(Text::Empty(TextEmpty{})),
        }),
        conclusion: Value::Text(Text::All(TextAll{})),
    };

    let is_dog = Rule {
        premise: Change::Add(Add {
            value: Value::Text(Text::Empty(TextEmpty{})),
        }),
        conclusion: Value::Text(Text::Literal(TextLiteral{
            value: String::from("dog"),
        })),
    };


    // define different set of goals
    // before proceeding to using lulo just for elea data types
    // 
    // make lulo great standalone and emphazie that it handles data
    // while elea handles control flow
    //
    // read dependent type book and practical foundations
    //
    // Goals:
    // * 
}
