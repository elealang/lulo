//! CLI

pub mod command;


use clap::{App, AppSettings, ArgMatches};

use crate::interface::cli::command::db;


/// Run the Command Line Interface
pub fn run() {
    
    let matches = matches();

    match matches.subcommand() {
        Some(("db", db_matches)) => db::eval(db_matches),
        Some((cmd    , _    ))   => {
            println!("Unknown command: {}", cmd)
        },
        _                        => {
            println!("{}", "Unknown error")
        }
    }
}


/// Run the CLI and get the user input
fn matches() -> ArgMatches {
    return App::new("Lulo")
        .version("0.1.0")
        .about("A Data Language with an advanced type system, code/documentation generation, and much more!")
        .setting(AppSettings::ArgRequiredElseHelp)
        .subcommand(db::command())
        .get_matches();
}
