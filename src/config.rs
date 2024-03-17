pub struct Config<'a> {
    pub query: &'a str,
    pub file_path: &'a str,
}

impl<'a> Config<'a> {
    pub fn from(args: &'a [String]) -> Result<Self, &'static str> {
        if args.len() < 3 {
            return Err("Not enough args provided.");
        }

        Ok(Self {
            query: &args[1],
            file_path: &args[2],
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_ok() {
        let args: Vec<String> = vec!["rubish".to_string(), "foo".to_string(), "bar".to_string()];
        let config = Config::from(&args);
        assert!(config.is_ok());
        let config = config.unwrap();
        assert_eq!(config.query, "foo");
        assert_eq!(config.file_path, "bar")
    }

    #[test]
    fn test_from_err() {
        let args: Vec<String> = vec!["foo".to_string(), "bar".to_string()];
        let config = Config::from(&args);
        assert!(config.is_err());
        match config {
            Ok(_) => {
                panic!("Is ok when err is expected")
            }
            Err(e) => {
                assert_eq!(e, "Not enough args provided.")
            }
        }
    }
}
