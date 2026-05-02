use std::env;
use std::process;

use colored::Colorize;
use task_manager::{run, Config};

fn main() {
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("{}", format!("Problem parsing arguments: {err}").red());
        print_usage();
        process::exit(1);
    });

    if let Err(error) = run(config) {
        eprintln!("{}", format!("Application error: {error}").red());
        process::exit(1);
    }
}

fn print_usage() {
    eprintln!(
        "\
Usage:
  task_manager add <text...>
  task_manager list
  task_manager done <id>

  cargo run -- add my task
  cargo run -- list
  cargo run -- done 1"
    );
}
