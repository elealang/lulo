//! Subcommand: db info
//! Print info on the database

use clap::{App, Arg, ArgMatches};
use console::style;

use crate::interface::cli::util;
use crate::api;
use crate::atom::uri;
use crate::atom::uri::URI;

pub fn command<'a>() -> App<'a> {
    return App::new("info").about("Information on the database").arg(
        Arg::new("URI")
            .about("URI of database")
            .required(true)
            .index(1),
    );
}

/// Evaluate the new command
pub fn eval(matches: &ArgMatches) {
    let db_uri = matches.value_of("URI").unwrap();

    let uri = util::val_or_cli_error(URI::from_string(db_uri), &|e: uri::error::Error| {
        e.to_string()
    });

    let db = util::val_or_cli_error(api::db::database(&uri), &|e: api::db::error::Error| {
        e.to_string()
    });

    println!("🚀 {}", style("database info").cyan(),);
    println!("{}", "----------------");

    db.print_cli();
}
