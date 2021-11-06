//! CLI: Database commands
//! All commands for creating and managing Lulo databases


pub mod new;


use clap::{App, ArgMatches};


pub fn command<'a>() -> App<'a> { 
    return App::new("db")
        .about("Create and manage databases")
        .subcommand(new::command());
}


pub fn eval(_matches: &ArgMatches) {
    println!("{}", "database stuff");
}
