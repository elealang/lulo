//! CLI: Database commands
//! All commands for creating and managing Lulo databases


pub mod new;


use clap::{Arg, ArgMatches, App};

use crate::types::database::Database;


pub fn command<'a>() -> App<'a> { 
    return App::new("db")
        .about("Create and manage databases")
        .arg(Arg::new("database")
            .short('d')
            .long("database")
            .value_name("DATABASE")
            .about("URI of database to use"))
        .subcommand(new::command());
}


/// Evaluate database commands
pub fn eval(matches: &ArgMatches) {
    match matches.subcommand() {
        Some(("new", matches)) => new::eval(matches),
        _                      => {},
    }
}

