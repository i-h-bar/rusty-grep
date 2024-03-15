use std::fs;

use crate::fuzzy::lev;

pub fn find_file_match(file: &str) -> Option<String> {
    for potential in fs::read_dir(".").ok()? {
        let pot = potential.ok()?.path();
        let pot_str = match pot.as_path().to_str() {
            Some(n) => n,
            None => continue
        };
        let name = match pot_str.split_once("\\") {
            Some((_, name)) => name,
            None => continue
        };
        if lev::distance(name, file) < 3 {
            return Some(name.to_string());
        }
    }

    None
}