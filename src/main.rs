mod cli;
mod types;
mod uri;
mod validate;


fn main() {

    // Run CLI
    let matches_main = cli::run(); 

    // Command: Check
    if let Some(ref matches_check) = matches_main.subcommand_matches("check") {

        let schema = cli::schema::schema(&matches_main);
        match schema {
            Ok(schema) => cli::command::check(schema, matches_check),
            Err(err)   => print!("{}", err.to_string()),
        }
    }
}



