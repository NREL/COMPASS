use clap::{arg, command, value_parser, Arg, ArgAction, Command};
use std::path::PathBuf;

fn main() {
    let matches = command!() // requires `cargo` feature
        .arg(arg!(--db <DATABASE>).required(true))
        .arg(Arg::new("verbose").short('v').action(ArgAction::Count))
        .subcommand(Command::new("init").about("Initialize a new database"))
        .subcommand(
            Command::new("load")
                .about("Load ordinance raw data")
                .arg(Arg::new("path").value_parser(value_parser!(PathBuf))),
        )
        .subcommand(
            Command::new("export")
                .about("Export the database")
                .arg(
                    arg!(--format <FORMAT>)
                    .default_value("csv")),
        )
        .subcommand(Command::new("log").about("Show the history of the database"))
        .get_matches();

    let verbose = matches.get_count("verbose");
    if verbose > 0 {
        println!("verbose level: {:?}", verbose);
    }

    //       Command::new("log")
    //          .about("Show the history of the database")
    let db = matches.get_one::<String>("db").expect("required");
    println!("two: {:?}", db);

    match matches.subcommand_name() {
        Some("init") => {
            println!("Creating database at {:?}", &db);
            ordinance::init_db(db).unwrap();
        }
        Some("export") => {
            if verbose > 0 {
                println!("Showing export for database at {:?}", &db);
            }

            ordinance::export_db(&db);
        }
        Some("log") => {
            println!("Showing log for database at {:?}", &db);
        }
        _ => {
            println!("No subcommand was used");
        }
    }

}
