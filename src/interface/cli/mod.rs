//! CLI

pub mod command;
mod util;

use clap::{App, AppSettings, ArgMatches};
use serde::Deserialize;

use crate::interface::cli::command::db;

/// Run the Command Line Interface
pub fn run() {

    let env = envy::from_env::<Env>().unwrap();

    let matches = matches();

    match matches.subcommand() {
        Some(("db", db_matches)) => db::eval(db_matches, env),
        Some((cmd, _)) => {
            println!("Unknown command: {}", cmd)
        }
        _ => {
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

/// Env
#[derive(Deserialize, Debug)]
pub struct Env {
    
  #[serde(alias = "LULO_DATABASE")]
  db_uri: Option<String>,
}
