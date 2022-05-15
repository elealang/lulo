//! db-schema-list 
//! List database schemas

use clap::{App, Arg, ArgMatches};
use console::style;

use crate::interface::cli;
use crate::interface::cli::util;
use crate::api;
use crate::atom::uri;
use crate::atom::uri::URI;

pub fn command<'a>() -> App<'a> {
    return App::new("list")
        .about("List schemas");
}

/// Evaluate the new command
pub fn eval(matches: &ArgMatches, env: cli::Env) {

    let uri_str = env.db_uri.expect("Database URI not specified");

    let uri = util::val_or_cli_error(URI::from_string(&uri_str), &|e: uri::error::Error| {
        e.to_string()
    });

    let db = util::val_or_cli_error(api::db::database(&uri), &|e: api::db::error::Error| {
        e.to_string()
    });


    println!("🚀 {} {} {}", "using", style(db.id).cyan(), "database");
    println!("{}", "-------------------");

    for schema in db.schemas.iter() {
        println!(" - {}", schema.id);
    }

}
