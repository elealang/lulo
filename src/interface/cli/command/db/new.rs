//! Subcommand: db new 
//! Create a new database from the CLI


use clap::{Arg, ArgMatches, App};
use console::style;

use crate::types::uri::URI;
use crate::lib;


pub fn command<'a>() -> App<'a> { 
    return App::new("new")
        .about("Create a new database")
        .arg(Arg::new("NAME")
            .about("The database name")
            .required(true)
            .index(1))
        .arg(Arg::new("URI")
            .about("URI of database storage resource")
            .required(true)
            .index(2));
}

/// Evaluate the new command
pub fn eval(matches: &ArgMatches) {
    let db_name = matches.value_of("NAME").unwrap();
    let db_uri = matches.value_of("URI").unwrap();

    let uri = URI::from_string(db_uri);
    match uri {
        Ok(uri)  => {
            let mut db = lib::db::new(uri);
            db.set_name(db_name);
            println!(
                "🚀 {}", 
                style("database created").cyan(),
            );
            db.print_cli();
        }
        Err(err) => panic!("{}", err.to_string()),
    }
}
