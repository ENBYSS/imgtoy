use std::path::PathBuf;

use clap::{arg, command, value_parser, ArgAction, Command};
use owo_colors::OwoColorize;

pub fn clap_demo() {
    clap();
    panic!("CLAP DEMO END");
}

pub fn clap() {
    let matches = command!()
        .arg(arg!([name] "Optional name to operate on."))
        .arg(
            arg!(-c --config <FILE> "Sets a custom config file.")
                .required(false)
                .value_parser(value_parser!(PathBuf)),
        )
        .arg(arg!(-d --debug ... "Turn debugging information on."))
        .subcommand(
            Command::new("test")
                .about("does testing things")
                .arg(arg!(-l --list "lists test values").action(ArgAction::SetTrue)),
        )
        .get_matches();

    println!("\n\n");

    println!(
        "{}",
        "   ~~~   Welcome to the clap test zone.   ~~~   "
            .purple()
            .bold()
            .italic()
    );

    println!("\n");

    // You can check the value provided by positional arguments, or option arguments
    if let Some(name) = matches.get_one::<String>("name") {
        println!("Value for name: {name}");
    }

    if let Some(config_path) = matches.get_one::<PathBuf>("config") {
        println!("Value for config: {}", config_path.display());
    }

    // You can see how many times a particular flag or argument occurred
    // Note, only flags can have multiple occurrences
    let debug = matches
        .get_one::<u8>("debug")
        .expect("Counts are defaulted");

    match debug {
        0 => println!("Debug mode is off"),
        1 => println!("Debug mode is kind of on"),
        2 => println!("Debug mode is on"),
        _ => println!(
            "{}\n{}\n{}\n",
            format!("?????????? DEBUG LEVEL {debug} ??????????")
                .red()
                .bold(),
            "     Woah there settle down....     "
                .on_bright_red()
                .bold()
                .black(),
            "????????????????????????????????????".red().bold(),
        ),
    }

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    if let Some(matches) = matches.subcommand_matches("test") {
        // "$ myapp test" was run
        if matches.get_flag("list") {
            // "$ myapp test -l" was run
            println!("Printing testing lists...");
        } else {
            println!("Not printing testing lists...");
        }
    }
}
