use std::io::Result;
use clap::{Arg, App, SubCommand};

mod egn;
mod emails;

fn main() -> Result<()> {
    let commands = App::new("napgate")
        .version("0.1.0")
        .author("Martin Sotirov <contact@martinsotirov.com>")
        .about("Extracts data from CSV files")
        .subcommand(SubCommand::with_name("extract")
                    .about("Specifies what type of information to extract.")
                    .arg(Arg::with_name("information")
                         .required(true)
                         .help("Type of information to extract"))
                    .arg(Arg::with_name("files")
                         .required(true)
                         .help("Path to files for extraction"))
                    .arg(Arg::with_name("output")
                         .short("o")
                         .takes_value(true)
                         .value_name("OUTPUT")
                         .long("output")
                         .help("The file path to save the extraction to."))
                    )
        .get_matches();

    if let Some(command) = commands.subcommand_matches("extract") {
        let output = command.value_of("output");

        if let Some(path) = command.value_of("files") {
            match command.value_of("information") {
                Some("emails") => emails::run(path, output),
                Some("egn") => egn::run(path, output),
                Some(_) => {
                    println!("Type of information to extract needed.");
                    std::process::exit(1);
                },
                None => {
                    println!("Type of information to extract needed.");
                    std::process::exit(1);
                }
            }
        }
    }

    Ok(())
}
