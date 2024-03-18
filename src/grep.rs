use regex::Regex;
use std::io::ErrorKind;
use std::{fs, process};

use crate::config::Config;
use crate::fuzzy::search;

pub fn run(config: &Config) {
    let content = match fs::read_to_string(config.file_path) {
        Err(e) => {
            match e.kind() {
                ErrorKind::NotFound => {
                    match search::find_file_match(&config.file_path) {
                        Some(p) => {
                            eprintln!("Could not find {} did you mean {}?", config.file_path, p)
                        }
                        None => eprintln!(
                            "Could not find any file close to matching {}",
                            config.file_path
                        ),
                    };
                }
                ErrorKind::PermissionDenied => {
                    eprintln!("You don't have permission to open {}", config.file_path)
                }
                _ => eprintln!("Error reading file"),
            }
            process::exit(1)
        }
        Ok(content) => content,
    };

    let hits = search(config.query, &content);

    if hits.len() == 0 {
        println!(
            "No items found matching '{}' in '{}'",
            config.query, config.file_path
        )
    } else {
        for (i, hit) in hits.iter().enumerate() {
            println!("{}.~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~", i + 1);
            println!("{}\n", hit);
        }
    }
}

fn search(query: &str, content: &str) -> Vec<String> {
    let mut lines: Vec<String> = Vec::new();
    let reg = Regex::new(&format!(r#".*?({}).*?(:?\.\.\.|\.|\?|!)(:?['"])??"#, query))
        .expect("Search regex should be valid");

    for captures in reg.captures_iter(content) {
        let line = match captures.get(0) {
            Some(cap) => String::from(cap.as_str()),
            None => continue,
        };

        lines.push(line.replace(query, &format!("\x1b[95m{}\x1b[0m", query)));
    }

    lines
}


#[cfg(test)]
mod tests {
    use crate::grep::search;

    #[test]
    fn test_search() {
        let vec = search("foo", "foo bar foo bar. foo bar foo.");
        assert_eq!(vec.len(), 2);
        assert_eq!(vec, vec!["\x1b[95mfoo\x1b[0m bar \x1b[95mfoo\x1b[0m bar.".to_string(), " \x1b[95mfoo\x1b[0m bar \x1b[95mfoo\x1b[0m.".to_string()])
    }
}
