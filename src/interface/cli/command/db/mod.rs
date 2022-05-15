//! CLI: Database commands
//! All commands for creating and managing Lulo databases

pub mod info;
pub mod new;
pub mod schema;

use clap::{App, Arg, ArgMatches};

use crate::interface::cli;

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
        .subcommand(schema::command());
}

/// Evaluate database commands
pub fn eval(matches: &ArgMatches, env: cli::Env) {
    match matches.subcommand() {
        Some(("new", matches)) => new::eval(matches),
        Some(("info", matches)) => info::eval(matches),
        Some(("schema", matches)) => schema::eval(matches, env),
        _ => {}
    }
}
