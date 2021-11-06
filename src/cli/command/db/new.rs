//! Subcommand: db new 
//! Create a new database from the CLI


use clap::{Arg, App};


pub fn command<'a>() -> App<'a> { 
    return App::new("new")
        .about("Create a new database")
        .arg(Arg::new("DB_NAME")
            .about("The database name")
            .required(true)
            .index(1))
        .arg(Arg::new("DB_PATH")
            .about("Local path to store the database")
            .required(true)
            .index(2));
}
