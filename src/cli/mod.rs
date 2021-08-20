pub mod command;
pub mod error;
pub mod schema;

use clap::{App, AppSettings, Arg};


/// Run the Command Line Interface
pub fn run() -> clap::ArgMatches {
    return App::new("Lulo")
        .version("0.1.0")
        .about("A Data Language with an advanced type system, code/documentation generation, and other features.")
        .setting(AppSettings::ArgRequiredElseHelp)
        .arg(
            Arg::new("schema")
                .long("schema")
                .takes_value(true)
                .about("Schema URI"),
        )
        .subcommand(
            App::new("check")
                .about("Typecheck a value")
                .arg(
                    Arg::new("value")
                        .about("Value URI")
                        .index(1)
                        .required(true),
                )
                .arg(Arg::new("type").about("Type Id").index(2).required(true)),
        )
        .subcommand(
            App::new("gen")
                .about("Generate artifacts based on a schema")
                .arg(
                    Arg::new("artifact-type")
                        .about("Artifact Type e.g. rust, html, python, ...")
                        .index(1)
                        .required(true),
                ),
        )
        .get_matches();
}

