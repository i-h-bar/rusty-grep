use io::ErrorKind;
use std::{env, fs, io, process};

use config::Config;
use fuzzy::search;

mod config;
mod fuzzy;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::from(&args).unwrap_or_else(
        |err| {
            println!("Could not parse args due to: {err}");
            process::exit(1)
        }
    );

    run(&config)
}

fn run(config: &Config) {
    let content = match fs::read_to_string(config.file_path) {
        Err(e) => {
            match e.kind() {
                ErrorKind::NotFound => {
                    match search::find_file_match(&config.file_path) {
                        Some(p) => println!("Could not find {} did you mean {}?", config.file_path, p),
                        None => println!("Could not find any file close to matching {}", config.file_path)
                    };
                }
                ErrorKind::PermissionDenied => println!("You don't have permission to open {}", config.file_path),
                _ => println!("Error reading file")
            }
            process::exit(1)
        }
        Ok(content) => content
    };

    println!("{content}");
}