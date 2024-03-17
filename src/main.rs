use std::{env, process};

use config::Config;

mod config;
mod fuzzy;
mod grep;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::from(&args).unwrap_or_else(|err| {
        eprintln!("Could not parse args due to: {err}");
        process::exit(1)
    });

    grep::run(&config)
}
