//! CLI: Database commands
//! All commands for creating and managing Lulo databases

pub mod add_schema;
pub mod info;
pub mod new;

use clap::{App, Arg, ArgMatches};

pub fn command<'a>() -> App<'a> {
    return App::new("db")
        .about("Create and manage databases")
        .arg(
            Arg::new("database")
                .short('d')
                .long("database")
                .value_name("DATABASE")
                .about("URI of database to use"),
        )
        .subcommand(new::command())
        .subcommand(info::command())
        .subcommand(add_schema::command());
}

/// Evaluate database commands
pub fn eval(matches: &ArgMatches) {
    match matches.subcommand() {
        Some(("add-schema", matches)) => new::eval(matches),
        Some(("info", matches)) => info::eval(matches),
        Some(("new", matches)) => new::eval(matches),
        _ => {}
    }
}
