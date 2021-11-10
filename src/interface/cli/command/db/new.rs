//! Subcommand: db new 
//! Create a new database from the CLI


use clap::{Arg, ArgMatches, App};
use console::style;

use crate::types::uri;
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

    let uri = val_or_cli_error(
        URI::from_string(db_uri),
        &|e: uri::error::Error| e.to_string(),
    );
    let mut db = lib::db::new(uri);
    db.set_id(db_name);
    println!(
        "🚀 {}", 
        style("database created").cyan(),
    );
    lib::db::sync(&db);
    db.print_cli();
}


pub fn val_or_cli_error<T,E>(res: Result<T,E>, msg: &dyn Fn(E) -> String) -> T 
{
    match res {
        Ok(val) => return val,
        Err(e)  => panic!(msg(e)),
    }
}
