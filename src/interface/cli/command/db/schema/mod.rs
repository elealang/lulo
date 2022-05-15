//! CLI: Database commands
//! All commands for creating and managing Lulo databases

pub mod list;

use clap::{App, Arg, ArgMatches};

use crate::interface::cli;

pub fn command<'a>() -> App<'a> {
    return App::new("schema")
        .about("Create and manage schemas for a database")
        .subcommand(list::command());
}

/// Evaluate database commands
pub fn eval(matches: &ArgMatches, env: cli::Env) {
    match matches.subcommand() {
        Some(("list", matches))   => list::eval(matches, env),
        _ => {}
    }
}


fn list(matches: &ArgMatches) {}
