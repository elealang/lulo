//! Types Serialization Structures


/// All of the possible Lulo types.
#[derive(Debug)]
#[derive(Serialize)]
#[serde(tag = "kind", content = "type")]
pub enum Type {
    Set {
        id           : String,
        label        : String,
        description  : String,
        parent_group : Option<&str>,
        group        : Option<&str>,
        pub size     : SetSize,
        pub types    : Vec<&str>,
    },
    List(List),
    Text(Text),
    Integer(Integer),
    Float(Float),
    Symbol(Symbol),
}


