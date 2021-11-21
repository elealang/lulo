//! Subcommand: db new
//! Create a new database from the CLI

use clap::{App, Arg, ArgMatches};
use console::style;

use crate::interface::cli::util;
use crate::api;
use crate::atom::uri;
use crate::atom::uri::URI;

pub fn command<'a>() -> App<'a> {
    return App::new("new")
        .about("Create a new database")
        .arg(
            Arg::new("NAME")
                .about("The database name")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::new("URI")
                .about("URI of database storage resource")
                .required(true)
                .index(2),
        );
}

/// Evaluate the new command
pub fn eval(matches: &ArgMatches) {
    let db_name = matches.value_of("NAME").unwrap();
    let db_uri = matches.value_of("URI").unwrap();

    let uri = util::val_or_cli_error(URI::from_string(db_uri), &|e: uri::error::Error| {
        e.to_string()
    });
    let mut db = api::db::new(uri);
    db.set_id(db_name);
    println!("🚀 {}", style("database created").cyan(),);
    println!("{}", "-------------------");

    util::val_or_cli_error(api::db::sync(&db), &|e: api::db::error::Error| {
        e.to_string()
    });
    db.print_cli();
}
