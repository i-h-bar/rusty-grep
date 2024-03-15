
pub struct Config<'a> {
    pub query: &'a str,
    pub file_path: &'a str
}


impl<'a> Config<'a> {
    pub fn from(args: &'a [String]) -> Result<Self, &'static str> {
        if args.len() < 3 {
            return Err("Not enough args provided.")
        }

        Ok(Self {query: &args[1], file_path: &args[2]})
    }
}
