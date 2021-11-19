//! Subcommand: add schema
//! Add a schema to an existing database

use clap::{App, Arg, ArgMatches};

pub fn command<'a>() -> App<'a> {
    return App::new("add-schema")
        .about("Add a schema to an existing database")
        .arg(
            Arg::new("URI")
                .about("URI of schema")
                .required(true)
                .index(2),
        );
}

/// Evaluate the new command
pub fn eval(matches: &ArgMatches) {
    let _schema_uri = matches.value_of("URI").unwrap();
}
