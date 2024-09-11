use minigrep::Config;
use std::{env, process};

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    println!(
        "Searching for '{}', in '{}'",
        config.query, config.file_path
    );
    if let Err(e) = minigrep::run(config) {
        println!("Problem while running program: {e}");
        process::exit(1);
    }
}
