
mod is_type;
mod typ;
mod uri;
mod value;

use std::env;
use clap::{Arg, App, AppSettings};

fn main() {

    let matches = App::new("Lulo")
        .version("1.0")
        .author("Jeff Wise <jeff@elealang.com>")
        .about("Does awesome things")
        .setting(AppSettings::ArgRequiredElseHelp)
        .arg(Arg::new("config")
            .short('c')
            .long("config")
            .value_name("FILE")
            .about("Sets a custom config file")
            .takes_value(true))
        .arg(Arg::new("v")
            .short('v')
            .multiple_occurrences(true)
            .takes_value(true)
            .about("Sets the level of verbosity"))
        .subcommand(App::new("check")
            .about("Check the type of the input (file, string, uri, ...)")
            .arg(Arg::new("value-uri")
                .about("value URI")
                .index(1)
                .required(true)
            )
            .arg(Arg::new("type-id")
                .about("type id")
                .index(2)
                .required(true)
            )
        )
        .get_matches();

    if let Some(c) = matches.value_of("config") {
        println!("Value for config: {}", c);
    }

    // You can see how many times a particular flag or argument occurred
    // Note, only flags can have multiple occurrences
    match matches.occurrences_of("v") {
        0 => println!("Verbose mode is off"),
        1 => println!("Verbose mode is kind of on"),
        2 => println!("Verbose mode is on"),
        _ => println!("Don't be crazy"),
    }

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level app
    if let Some(ref matches) = matches.subcommand_matches("check") {

        // Load schema
        let schema_uri;
        match env::var("LULO_SCHEMA") {
            Ok(val) => schema_uri = val,
            Err(_e) => panic!("Must define LULO_SCHEMA env var (points to a valid Lulo schema file)")
        }

        println!("schema uri: {:?}", schema_uri);

        if matches.is_present("value-uri") && matches.is_present("type-id"){
            println!("value-uri: {:?}", matches.value_of("value-uri"));
            println!("type-id: {:?}", matches.value_of("type-id"));

            let value = uri::value(matches.value_of("uri").unwrap());

            //is_type(value, typ);
        } else {
            println!("Printing normally...");
        }
    }

    // Continued program logic goes here...
}

