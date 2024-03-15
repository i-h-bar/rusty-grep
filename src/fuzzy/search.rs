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


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_file() {
        let file_match = find_file_match("text.txt");
        assert!(file_match.is_some());
        assert_eq!(find_file_match("text.txt").unwrap(), "test.txt")
    }

    #[test]
    fn no_file() {
        assert!(find_file_match("sadasdad").is_none())
    }
}