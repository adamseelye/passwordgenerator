use clap::{App, Arg, SubCommand};

mod generator;

use crate::generator::{gen_password, gen_line};


fn main() {
    let matches = App::new("Password Generator")
        .version("1.0.1")
        .author("Adam Seelye")
        .about("Password Generator")
        .subcommand(
            SubCommand::with_name("gen_password")
                .long_flag("cli")
                .short_flag('c')
                .about("Start CLI application")
        )
        .subcommand(
            SubCommand::with_name("generator")
                .long_flag("generate")
                .short_flag('g')
                .about("Generate Password with arguments")
                .arg(
                    Arg::with_name("pass_length")
                    .short('l')
                    .long("length")
                    .help("Number of password characters")
                    .takes_value(true)
                    .required(true),
                )
                .arg(
                    Arg::with_name("excluded")
                    .short('e')
                    .long("excluded")
                    .help("Excluded characters")
                    .takes_value(true)
                    .required(true)
                ),
        )

    .get_matches();

    if let Some(_matches) = matches.subcommand_matches("gen_password") {
        if let Err(e) = gen_password() {
            eprintln!("Error: {}", e);
        }
    } else if let Some(matches) = matches.subcommand_matches("generator") {
        if let Some(pass_length) = matches.value_of("pass_length") {
            if let Some(excluded) = matches.value_of("excluded") {
                if let Err(e) = gen_line(pass_length.to_string(), excluded.to_string()) {
                    eprintln!("Error: {}", e);
                }
            }
        }    
    }
}


